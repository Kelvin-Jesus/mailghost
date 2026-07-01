use std::io::{self, Write};

use anyhow::{bail, Context, Result};
use chrono::Utc;
use ephemeral_email::{ProviderType, TempMail};

#[cfg(not(target_os = "android"))]
use std::fs;

use crate::account::{Account, AccountStore};
use crate::cli::{Cli, Command};

pub async fn run(cli: Cli) -> Result<()> {
    let store = AccountStore::from_config_dir()?;

    match cli.command {
        Command::Generate => generate(&store).await,
        Command::Messages => messages(&store).await,
        Command::Delete => delete(&store),
        Command::Account => show_account(&store),
    }
}

async fn generate(store: &AccountStore) -> Result<()> {
    if store.load()?.is_some() {
        bail!("an inbox already exists; delete it first with `mailghost delete`");
    }

    let inbox = TempMail::new()
        .provider_type(ProviderType::MailTm)
        .create_inbox()
        .await
        .context("could not create a Mail.tm inbox")?;

    let account = Account {
        email: inbox.get_email_address().to_string(),
        created_at: Utc::now(),
    };
    store.save(&account)?;

    println!("{}", account.email);
    Ok(())
}

async fn messages(store: &AccountStore) -> Result<()> {
    let account = require_account(store)?;
    let inbox = TempMail::new()
        .provider_type(ProviderType::MailTm)
        .name(account.local_part()?)
        .create_inbox()
        .await
        .context("could not connect to the Mail.tm inbox")?;

    let messages = inbox
        .get_messages()
        .await
        .context("could not fetch messages")?;

    if messages.is_empty() {
        println!("No messages.");
        return Ok(());
    }

    println!("Messages:");
    for (index, message) in messages.iter().enumerate() {
        println!(
            "  {}. {} — from {}",
            index + 1,
            message.subject,
            message.from
        );
    }

    print!("\nEnter a message number to open, or press Enter to exit: ");
    io::stdout().flush().context("could not flush stdout")?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("could not read the selection")?;
    let selection = input.trim();
    if selection.is_empty() {
        return Ok(());
    }

    let number = selection
        .parse::<usize>()
        .context("message selection must be a number")?;
    let message = messages
        .get(number.saturating_sub(1))
        .filter(|_| number > 0)
        .with_context(|| format!("message {number} does not exist"))?;

    if message.body.trim().is_empty() {
        println!("This message has no text content.");
        return Ok(());
    }

    open_message(&message.body)
}

fn delete(store: &AccountStore) -> Result<()> {
    if store.delete()? {
        println!("Local inbox data deleted.");
    } else {
        println!("No local inbox data found.");
    }
    Ok(())
}

fn show_account(store: &AccountStore) -> Result<()> {
    let account = require_account(store)?;
    println!("Email: {}", account.email);
    println!(
        "Created: {}",
        account.created_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    Ok(())
}

fn require_account(store: &AccountStore) -> Result<Account> {
    store
        .load()?
        .context("no inbox exists; create one with `mailghost generate`")
}

#[cfg(not(target_os = "android"))]
fn open_message(body: &str) -> Result<()> {
    let path = std::env::temp_dir().join(format!("mailghost-message-{}.txt", std::process::id()));
    fs::write(&path, body).with_context(|| format!("could not write {}", path.display()))?;
    open::that(&path).with_context(|| format!("could not open {}", path.display()))
}

#[cfg(target_os = "android")]
fn open_message(body: &str) -> Result<()> {
    println!("\n{body}");
    Ok(())
}
