use colored::Colorize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command as ProcessCommand;

use crate::get_hash;

pub struct StartupChecker {
    pub args: super::Args,
    pub config: Option<super::Config>,
    pub should_use_ffmpeg_path_field: Option<bool>,
}

impl StartupChecker {
    pub fn check(&mut self) -> bool {
        let mut result = self.check_config();
        self.load_config();
        result = self.check_args() && result;
        result = self.check_ffmpeg_executable() && result;
        println!();
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

    fn load_config(&mut self) {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kffmpeg")
            .join("config.yaml");
        let config_str = fs::read_to_string(config_path).expect("Unable to read file");
        let config: super::Config = serde_yaml::from_str(&config_str).unwrap();
        self.config = Some(config);

        match &self.config {
            Some(config) => {
                self.print_message("Config loaded.", true);
                for command in config.commands.iter() {
                    println!(
                        "    {} -> {}",
                        get_hash(command.title.clone()).as_str().bright_cyan(),
                        command.title.clone().as_str(),
                    );
                }
            }
            None => {}
        }
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
            let mut result: bool;
            if self
                .config
                .as_ref()
                .unwrap()
                .commands
                .iter()
                .map(|c| get_hash(c.title.clone()))
                .collect::<Vec<String>>()
                .contains(&self.args.hash.clone().unwrap())
            {
                self.print_message(
                    format!(
                        "Specified hash code is found in config file. title -> {}",
                        self.config.as_ref().unwrap().commands[self
                            .config
                            .as_ref()
                            .unwrap()
                            .commands
                            .iter()
                            .map(|c| get_hash(c.title.clone()))
                            .collect::<Vec<String>>()
                            .iter()
                            .position(|r| r == self.args.hash.as_ref().unwrap().as_str())
                            .unwrap()]
                        .title
                        .clone()
                        .as_str()
                    )
                    .as_str(),
                    true,
                );
                result = true;
            } else {
                self.print_message("Specified hash code is not found in config file", false);
                result = false;
            }
            if Path::is_file(Path::new(self.args.input_path.clone().unwrap().as_str())) {
                self.print_message("Specified input file is found", true);
                result = true && result;
            } else {
                self.print_message("Specified input file is not found", false);
                result = false && result;
            }
            return result;
        } else {
            self.print_message("You did not specify --hash and --input_path. So, kffmpeg will run with user interaction.", true);
            return true;
        }
    }

    fn check_ffmpeg_executable(&mut self) -> bool {
        let result = ProcessCommand::new("ffmpeg")
            .arg("-version")
            .output()
            .expect("failed to execute process");
        if result.status.success() {
            self.print_message("ffmpeg command found", true);
            self.should_use_ffmpeg_path_field = Some(false);
            return true;
        } else {
            self.print_message("ffmpeg command not found", false);
            let result_2 = ProcessCommand::new(self.config.as_ref().unwrap().ffmpeg_path.clone())
                .arg("-version")
                .output()
                .expect("failed to execute process");
            if result_2.status.success() {
                self.print_message(
                    format!(
                        "ffmpeg command found at {}",
                        self.config.as_ref().unwrap().ffmpeg_path.clone()
                    )
                    .as_str(),
                    true,
                );
                self.should_use_ffmpeg_path_field = Some(true);
                return true;
            } else {
                self.print_message(
                    format!(
                        "ffmpeg command not found at {}",
                        self.config.as_ref().unwrap().ffmpeg_path.clone()
                    )
                    .as_str(),
                    false,
                );
                self.should_use_ffmpeg_path_field = Some(true);
                return false;
            }
        }
    }
}
