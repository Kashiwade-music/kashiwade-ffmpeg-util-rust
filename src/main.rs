use clap::Parser;
use colored::Colorize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,)
    ]
struct Args {
    #[arg(short, long, help = "Show version")]
    version: bool,

    #[arg(short, long, help = "Show config")]
    config: bool,

    #[arg(
        short,
        long,
        help = "Hash code to select a command. If this option is specified, the command will be executed without user interaction. You are also need to set --input_path. This option is useful when you want to use kffmpeg in a script. Hash code can be found in the console when you choose a command."
    )]
    hash: Option<String>,

    #[arg(
        short,
        long,
        help = "Input path to select a command. If this option is specified, the command will be executed without user interaction. You are also need to set --hash. This option is useful when you want to use kffmpeg in a script."
    )]
    input_path: Option<String>,
}
struct StartupChecker {
    args: Args,
}

impl StartupChecker {
    pub fn check(&self) -> bool {
        let mut result = false;
        result = self.check_config();

        return result;
    }

    fn print_message(&self, message: &str, is_ok: bool) {
        if is_ok {
            println!("[  {}  ] {}", "OK".green(), message);
        } else {
            println!("[  {}  ] {}", "NG".red(), message);
        }
    }

    fn check_config(&self) -> bool {
        if Path::is_file(Path::new("~/.config/kffmpeg/config.yaml")) {
            self.print_message("Config file found", true);
            return true;
        } else {
            self.print_message(
                format!(
                    "Config file was not found. -> make at {}",
                    fs::canonicalize(Path::new("~/.config/kffmpeg/config.yaml"))
                        .unwrap()
                        .display()
                )
                .as_str(),
                false,
            );
            return false;
        }
    }

    fn create_config(&self) {
        let config_path = Path::new("~/.config/kffmpeg/config.yaml");
    }
}

fn main() {
    let args = Args::parse();
    let checker = StartupChecker { args: args };
}
