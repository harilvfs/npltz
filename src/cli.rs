use clap::builder::styling::{AnsiColor, Style};
use clap::{ArgAction, Parser, Subcommand};

pub fn styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(Style::new().fg_color(Some(AnsiColor::Green.into())).bold())
        .usage(Style::new().fg_color(Some(AnsiColor::Green.into())).bold())
        .literal(Style::new().fg_color(Some(AnsiColor::Cyan.into())))
        .placeholder(Style::new().fg_color(Some(AnsiColor::Cyan.into())))
}

#[derive(Parser)]
#[command(name = "npltz", about = "Nepali Patro in your terminal", styles = styles())]
pub struct Cli {
    #[arg(
        short = 'v',
        visible_short_alias = 'V',
        long = "version",
        action = ArgAction::SetTrue,
        help = "Print version information"
    )]
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(
        long,
        global = true,
        help = "Set default theme (catppuccin-mocha, dracula, gruvbox, nord, rose-pine, or default to reset)"
    )]
    pub set_theme: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Show today's Nepali and English date")]
    Show {
        #[arg(long, help = "Convert an AD date to BS and display both (YYYY-MM-DD)")]
        date: Option<String>,

        #[arg(long, help = "Convert a BS date to AD and display both (YYYY-MM-DD)")]
        bs: Option<String>,

        #[arg(long, help = "Output as JSON")]
        json: bool,

        #[arg(
            long,
            value_name = "N",
            help = "Print the next N BS dates with their AD equivalents"
        )]
        upcoming: Option<u32>,
    },
    #[command(about = "Convert an AD date to Bikram Sambat (YYYY-MM-DD)")]
    Convert { date: String },
    #[command(about = "Convert a Bikram Sambat date to AD (YYYY-MM-DD)")]
    ConvertBs { date: String },
    #[command(about = "Generate shell completions (bash, zsh, fish)")]
    Completions { shell: clap_complete::Shell },
    #[command(about = "Install completions and man page to system paths")]
    Setup {
        #[arg(long, help = "Show what would be installed without making changes")]
        dry_run: bool,
    },
    #[command(about = "Check for updates")]
    CheckUpdate,
    #[command(about = "Update npltz to the latest version")]
    Update,
    #[command(about = "Show the current BS week")]
    Week,
    #[command(about = "Export BS dates to an iCalendar (.ics) file")]
    Export {
        #[arg(long, value_name = "YYYY-MM", help = "Start month (default: current BS month)")]
        month: Option<String>,

        #[arg(long, value_name = "N", help = "Number of months to export (default: 1)")]
        count: Option<u32>,

        #[arg(short, long, value_name = "FILE", help = "Output file path")]
        output: Option<String>,
    },
    #[command(about = "Uninstall npltz and its files")]
    Uninstall,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_no_args() {
        let cli = Cli::try_parse_from(["npltz"]).unwrap();
        assert!(cli.command.is_none());
        assert_eq!(cli.set_theme, None);
    }

    #[test]
    fn test_cli_show() {
        let cli = Cli::try_parse_from(["npltz", "show"]).unwrap();
        match cli.command.unwrap() {
            Commands::Show { date, bs, json, upcoming } => {
                assert!(date.is_none());
                assert!(bs.is_none());
                assert!(!json);
                assert!(upcoming.is_none());
            }
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn test_cli_show_json() {
        let cli = Cli::try_parse_from(["npltz", "show", "--json"]).unwrap();
        match cli.command.unwrap() {
            Commands::Show { json, .. } => assert!(json),
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn test_cli_show_date() {
        let cli = Cli::try_parse_from(["npltz", "show", "--date", "2024-04-13"]).unwrap();
        match cli.command.unwrap() {
            Commands::Show { date, bs, .. } => {
                assert_eq!(date.unwrap(), "2024-04-13");
                assert!(bs.is_none());
            }
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn test_cli_show_bs() {
        let cli = Cli::try_parse_from(["npltz", "show", "--bs", "2081-01-01"]).unwrap();
        match cli.command.unwrap() {
            Commands::Show { bs, .. } => assert_eq!(bs.unwrap(), "2081-01-01"),
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn test_cli_convert() {
        let cli = Cli::try_parse_from(["npltz", "convert", "2024-04-13"]).unwrap();
        match cli.command.unwrap() {
            Commands::Convert { date } => assert_eq!(date, "2024-04-13"),
            _ => panic!("Expected Convert command"),
        }
    }

    #[test]
    fn test_cli_convert_bs() {
        let cli = Cli::try_parse_from(["npltz", "convert-bs", "2081-01-01"]).unwrap();
        match cli.command.unwrap() {
            Commands::ConvertBs { date } => assert_eq!(date, "2081-01-01"),
            _ => panic!("Expected ConvertBs command"),
        }
    }

    #[test]
    fn test_cli_set_theme() {
        let cli = Cli::try_parse_from(["npltz", "--set-theme", "dracula"]).unwrap();
        assert_eq!(cli.set_theme.unwrap(), "dracula");
    }

    #[test]
    fn test_cli_invalid_command_fails() {
        let result = Cli::try_parse_from(["npltz", "unknown"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_set_theme_invalid_still_parses() {
        let cli = Cli::try_parse_from(["npltz", "--set-theme", "invalid"]).unwrap();
        assert_eq!(cli.set_theme.unwrap(), "invalid");
    }
}
