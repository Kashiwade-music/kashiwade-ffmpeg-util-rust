use clap::Parser;
use colored::Colorize;
use dirs;
use serde::{Deserialize, Serialize};
use std::{env, fs, io, io::Write, path::Path, path::PathBuf, process::Command as ProcessCommand};

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
struct Config {
    ffmpeg_path: String,
    commands: Vec<Command>,
}

struct StartupChecker {
    args: Args,
    config: Option<Config>,
}

impl StartupChecker {
    pub fn check(&mut self) -> bool {
        let mut result = self.check_config();
        self.config = Some(self.load_config());
        result = self.check_args() && result;
        result = self.check_ffmpeg_executable() && result;
        return result;
    }

    fn print_message(&self, message: &str, is_ok: bool) {
        if is_ok {
            println!("[  {}  ] {}", "OK".green(), message);
        } else {
            println!("[  {}  ] {}", "NG".red(), message);
        }
    }

    fn create_config(&self) {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kffmpeg")
            .join("config.yaml");
        let yaml_str = r#"ffmpeg_path: /usr/bin/ffmpeg
commands:
  - title: Make video lighter by using h264_nvenc CQ 32
    options:
      - flag: -cq
        value: 32
      - flag: -c:v
        value: h264_nvenc
    output_extension: .mp4
    output_filename_suffix: _light
    command:
      - "{{ffmpeg_path}}"
      - -i
      - "{{input_path}}"
      - "{{options}}"
      - "{{output_path}}"
  - title: Concat videos by getting txt file
    options:
      - flag: -safe
        value: 0
      - flag: -c
        value: copy
    output_extension: .mp4
    output_filename_suffix: _concat
    command:
      - "{{ffmpeg_path}}"
      - -f
      - concat
      - -i
      - "{{input_path}}"
      - "{{options}}"
      - "{{output_path}}"
"#;
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        let mut file = fs::File::create(config_path.clone()).unwrap();
        file.write_all(yaml_str.as_bytes()).unwrap();
    }

    fn check_config(&self) -> bool {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kffmpeg")
            .join("config.yaml");
        if Path::is_file(&config_path) {
            self.print_message("Config file found", true);
            return true;
        } else {
            self.create_config();
            self.print_message(
                format!(
                    "Config file was not found. -> make at {}",
                    config_path.display()
                )
                .as_str(),
                false,
            );
            return false;
        }
    }

    fn load_config(&self) -> Config {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kffmpeg")
            .join("config.yaml");
        let config_str = fs::read_to_string(config_path).expect("Unable to read file");
        let config: Config = serde_yaml::from_str(&config_str).unwrap();
        return config;
    }

    fn check_args(&self) -> bool {
        if self.args.hash != None && self.args.input_path == None {
            self.print_message(
                "You need to specify --input_path when you specify --hash.",
                false,
            );
            return false;
        } else if self.args.hash == None && self.args.input_path != None {
            self.print_message(
                "You need to specify --hash when you specify --input_path.",
                false,
            );
            return false;
        } else if self.args.hash != None && self.args.input_path != None {
            self.print_message("You specified --hash and --input_path. So, kffmpeg will run without user interaction.", true);
            return true;
        } else {
            self.print_message("You did not specify --hash and --input_path. So, kffmpeg will run with user interaction.", true);
            return true;
        }
    }

    fn check_ffmpeg_executable(&self) -> bool {
        // check by run command
        let result = ProcessCommand::new("ffmpeg")
            .arg("-version")
            .output()
            .expect("failed to execute process");
        if result.status.success() {
            self.print_message("ffmpeg command found", true);
            return true;
        } else {
            self.print_message("ffmpeg command not found", false);
            return false;
        }
    }
}

struct Runner {
    args: Args,
    config: Config,
}

impl Runner {
    pub fn run(&self) {
        let command: &Command = self.get_command();
        let input_path = self.get_input_path();
        let options = self.get_options(command.options.clone());
        let output_path = self.get_output_path(input_path.clone(), command);
    }

    fn print_message(&self, message: &str, is_from_system: bool) {
        if is_from_system {
            println!("[{}] {}", "SYSTEM".yellow(), message);
        } else {
            println!("[ {} ] {}", "USER".blue(), message);
        }
    }

    fn get_command(&self) -> &Command {
        self.print_message("Choose a command", true);
        for (idx, command) in self.config.commands.iter().enumerate() {
            println!("{}: {}", idx, command.title);
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: usize = input.trim().parse().expect("Please type a number!");
        let command = &self.config.commands[input];
        self.print_message(format!("You chose {}", command.title).as_str(), false);
        return command;
    }

    fn get_input_path(&self) -> PathBuf {
        self.print_message("Input the path of the video file.", true);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let input_path = Path::new(input);
        if input_path.is_file() {
            self.print_message(
                format!("You chose {}", input_path.display().to_string().bold()).as_str(),
                false,
            );
            return input_path.to_path_buf();
        } else {
            self.print_message(
                format!("{} is not a file.", input_path.display().to_string().bold()).as_str(),
                false,
            );
            return self.get_input_path();
        }
    }

    fn get_options(&self, options: Vec<CommandOption>) -> Vec<String> {
        self.print_message("Current options are as follows.", true);
        for (idx, option) in options.iter().enumerate() {
            println!(
                "{} {}: {}",
                idx.to_string().green(),
                option.flag,
                option.value
            );
        }
        self.print_message("Is it OK? type 'y' or index you want to change", true);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "y" {
            self.print_message("You chose to use current options.", false);
            return options.iter().map(|option| option.value.clone()).collect();
        } else {
            let input_idx: usize = input.parse().expect("Please type a number!");
            let option = &options[input_idx];
            self.print_message(
                format!(
                    "You chose to change {} {}",
                    option.flag.bold(),
                    option.value.bold()
                )
                .as_str(),
                false,
            );
            let mut new_options = options.clone();
            self.print_message("Input new value", true);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            new_options[input_idx] = CommandOption {
                flag: option.flag.clone(),
                value: input.to_string(),
            };
            return self.get_options(new_options);
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut checker = StartupChecker {
        args: args,
        config: None,
    };
    let check_result = checker.check();
    if check_result {
        println!("OK");
    } else {
        println!("NG");
    }
}
