use std::io::{self, Write};

use anstyle::{AnsiColor, Style};
use clap::CommandFactory;

use crate::Cli;

pub(crate) fn write() -> io::Result<()> {
    let stream = anstream::stdout();
    let mut output = stream.lock();
    render(&mut output)
}

fn render(output: &mut dyn Write) -> io::Result<()> {
    let cyan = Style::new()
        .fg_color(Some(AnsiColor::BrightCyan.into()))
        .bold();
    let blue = Style::new().fg_color(Some(AnsiColor::BrightBlue.into()));
    let violet = Style::new()
        .fg_color(Some(AnsiColor::BrightMagenta.into()))
        .bold();
    let command_style = Style::new()
        .fg_color(Some(AnsiColor::BrightYellow.into()))
        .bold();
    let heading = Style::new()
        .fg_color(Some(AnsiColor::BrightBlue.into()))
        .bold();
    let muted = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));

    writeln!(
        output,
        "{cyan}       .------------------.{cyan:#}      \
         {violet}mailghost {}{violet:#}",
        env!("CARGO_PKG_VERSION")
    )?;
    writeln!(
        output,
        "{cyan}      / \\                /{cyan:#}       {}",
        env!("CARGO_PKG_DESCRIPTION")
    )?;
    writeln!(output, "{blue}     |   \\     >_       /{blue:#}")?;
    writeln!(output, "{blue}     '------------------'{blue:#}")?;

    writeln!(output)?;
    writeln!(output, "{heading}Usage{heading:#}")?;
    writeln!(output, "  mailghost {muted}<COMMAND>{muted:#}")?;

    writeln!(output)?;
    writeln!(output, "{heading}Commands{heading:#}")?;
    let command = Cli::command();
    for subcommand in command
        .get_subcommands()
        .filter(|subcommand| subcommand.get_name() != "help")
    {
        let alias = subcommand.get_all_aliases().next().unwrap_or("");
        let description = subcommand
            .get_about()
            .map(ToString::to_string)
            .unwrap_or_default();
        writeln!(
            output,
            "  {command_style}{:<9}{command_style:#} {cyan}{:<3}{cyan:#}  {}",
            subcommand.get_name(),
            alias,
            description
        )?;
    }

    writeln!(output)?;
    writeln!(output, "{heading}Options{heading:#}")?;
    writeln!(
        output,
        "  {command_style}-h, --help{command_style:#}       Print help"
    )?;
    writeln!(
        output,
        "  {command_style}-V, --version{command_style:#}    Print version"
    )?;

    Ok(())
}
