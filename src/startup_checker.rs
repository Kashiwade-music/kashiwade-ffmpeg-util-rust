use colored::Colorize;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command as ProcessCommand;

pub struct StartupChecker {
    pub args: super::Args,
    pub config: Option<super::Config>,
}

impl StartupChecker {
    pub fn check(&mut self) -> bool {
        let mut result = self.check_config();
        self.config = Some(self.load_config());
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

    fn load_config(&self) -> super::Config {
        let config_path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("kffmpeg")
            .join("config.yaml");
        let config_str = fs::read_to_string(config_path).expect("Unable to read file");
        let config: super::Config = serde_yaml::from_str(&config_str).unwrap();
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
