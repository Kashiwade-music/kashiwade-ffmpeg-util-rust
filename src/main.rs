use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
mod runner;
mod startup_checker;
use data_encoding::HEXLOWER;
use ring::digest;

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[arg(short, long, help = "Show config")]
    config: bool,

    #[arg(
        long,
        help = "Hash code to select a command. If this option is specified, the command will be executed without user interaction. You are also need to set --input_path. This option is useful when you want to use kffmpeg in a script. Hash code can be found in the console when you choose a command."
    )]
    hash: Option<String>,

    #[arg(
        long,
        help = "Input path to select a command. If this option is specified, the command will be executed without user interaction. You are also need to set --hash. This option is useful when you want to use kffmpeg in a script."
    )]
    input_path: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct CommandOption {
    flag: String,
    value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Command {
    title: String,
    options: Vec<CommandOption>,
    output_extension: String,
    output_filename_suffix: String,
    command: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    ffmpeg_path: String,
    commands: Vec<Command>,
}

fn get_hash(input: String) -> String {
    let digest = digest::digest(&digest::SHA256, input.as_bytes());
    let hash = HEXLOWER.encode(digest.as_ref());
    return hash[0..8].to_string();
}

fn main() {
    let args = Args::parse();
    let mut checker = startup_checker::StartupChecker {
        args: args,
        config: None,
        should_use_ffmpeg_path_field: None,
    };
    let check_result = checker.check();
    if check_result {
        let runner = runner::Runner {
            args: checker.args,
            config: checker.config.unwrap(),
        };
        runner.run();
    }
}
