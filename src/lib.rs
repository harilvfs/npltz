mod app;
mod calendar;
mod cli;
mod commands;
mod config;
mod error;
mod log;
mod self_manage;
mod setup;
mod theme;
mod tui;
mod ui;

pub use cli::{Cli, Commands};

use crate::commands::{
    convert_ad_to_bs, convert_bs_to_ad, export_ics, show_ad_date, show_bs_date, show_today,
    show_upcoming, show_week,
};
use crate::tui::run_tui;
use clap::{CommandFactory, Parser};
use config::Config;
use error::{NpltzError, Result};
use std::io::{self, Write};

pub fn run() -> Result<()> {
    log::Log::init();
    let cli = Cli::parse();
    let mut out = io::stdout();

    if cli.version {
        let _ = writeln!(out, "npltz v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if let Some(theme) = &cli.set_theme {
        let mut config = Config::load();
        if theme == "default" {
            config.theme = None;
            config.save()?;
            let _ = writeln!(out, "Default theme restored.");
        } else if theme::is_valid_theme(theme) {
            config.theme = Some(theme.to_string());
            config.save()?;
            let _ =
                writeln!(out, "{} has been set as your default theme.", theme::display_name(theme));
        } else {
            let msg = format!(
                "Unknown theme '{}'. Valid themes: {}",
                theme,
                theme::THEME_NAMES.join(", ")
            );
            log::Log::error(&msg);
            return Err(NpltzError::InvalidTheme(msg));
        }
        return Ok(());
    }

    match cli.command {
        Some(Commands::Show { date, bs, json, upcoming }) => {
            if let Some(n) = upcoming {
                show_upcoming(n)?;
            } else if let Some(bs_date) = bs {
                show_bs_date(&bs_date, json)?;
            } else if let Some(ad_date) = date {
                show_ad_date(&ad_date, json)?;
            } else {
                show_today(json)?;
            }
        }
        Some(Commands::Convert { date }) => {
            convert_ad_to_bs(&date)?;
        }
        Some(Commands::ConvertBs { date }) => {
            convert_bs_to_ad(&date)?;
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "npltz", &mut out);
        }
        Some(Commands::Setup { dry_run }) => {
            setup::run_setup(dry_run)?;
        }
        Some(Commands::CheckUpdate) => {
            self_manage::check_update()?;
        }
        Some(Commands::Update) => {
            self_manage::update()?;
        }
        Some(Commands::Week) => {
            show_week()?;
        }
        Some(Commands::Export { month, count, output }) => {
            export_ics(month.as_deref(), count, output.as_deref())?;
        }
        Some(Commands::Uninstall) => {
            self_manage::uninstall()?;
        }
        None => {
            run_tui()?;
        }
    }
    Ok(())
}
