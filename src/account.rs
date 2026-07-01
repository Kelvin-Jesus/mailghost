use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Account {
    pub(crate) email: String,
    pub(crate) created_at: DateTime<Utc>,
}

impl Account {
    pub(crate) fn local_part(&self) -> Result<&str> {
        let (local_part, domain) = self
            .email
            .split_once('@')
            .context("stored account contains an invalid email address")?;

        if local_part.is_empty() || domain.is_empty() {
            anyhow::bail!("stored account contains an invalid email address");
        }

        Ok(local_part)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AccountStore {
    path: PathBuf,
}

impl AccountStore {
    pub(crate) fn from_config_dir() -> Result<Self> {
        let config_dir = dirs::config_dir().context("could not determine the config directory")?;
        Ok(Self::new(config_dir.join("mailghost").join("account.json")))
    }

    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub(crate) fn load(&self) -> Result<Option<Account>> {
        let data = match fs::read_to_string(&self.path) {
            Ok(data) => data,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("could not read {}", self.path.display()));
            }
        };

        serde_json::from_str(&data)
            .with_context(|| format!("could not parse {}", self.path.display()))
            .map(Some)
    }

    pub(crate) fn save(&self, account: &Account) -> Result<()> {
        let parent = self
            .path
            .parent()
            .context("account path has no parent directory")?;
        fs::create_dir_all(parent)
            .with_context(|| format!("could not create {}", parent.display()))?;

        let data = serde_json::to_vec_pretty(account).context("could not serialize account")?;
        let temporary_path = temporary_path(&self.path);
        fs::write(&temporary_path, data)
            .with_context(|| format!("could not write {}", temporary_path.display()))?;
        fs::rename(&temporary_path, &self.path)
            .with_context(|| format!("could not replace {}", self.path.display()))
    }

    pub(crate) fn delete(&self) -> Result<bool> {
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => {
                Err(error).with_context(|| format!("could not remove {}", self.path.display()))
            }
        }
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut file_name = path.file_name().unwrap_or_default().to_os_string();
    file_name.push(".tmp");
    path.with_file_name(file_name)
}

#[cfg(test)]
mod tests {
    use super::{Account, AccountStore};
    use chrono::{TimeZone, Utc};
    use tempfile::tempdir;

    fn account() -> Account {
        Account {
            email: "person@example.test".to_owned(),
            created_at: Utc.with_ymd_and_hms(2026, 6, 30, 12, 0, 0).unwrap(),
        }
    }

    #[test]
    fn missing_account_is_not_an_error() {
        let directory = tempdir().unwrap();
        let store = AccountStore::new(directory.path().join("nested/account.json"));

        assert_eq!(store.load().unwrap(), None);
        assert!(!store.delete().unwrap());
    }

    #[test]
    fn account_round_trips_and_can_be_deleted() {
        let directory = tempdir().unwrap();
        let store = AccountStore::new(directory.path().join("nested/account.json"));
        let expected = account();

        store.save(&expected).unwrap();
        assert_eq!(store.load().unwrap(), Some(expected));
        assert!(store.delete().unwrap());
        assert_eq!(store.load().unwrap(), None);
    }

    #[test]
    fn rejects_invalid_stored_address() {
        let mut account = account();
        account.email = "not-an-address".to_owned();

        assert!(account.local_part().is_err());
    }
}
