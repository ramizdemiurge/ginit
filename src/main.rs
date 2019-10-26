use std::env;
use std::process::exit;
use std::process::Command;
use ansi_term::Colour::Red;
use ansi_term::Colour::Green;

struct Git {
    error: bool,
    red_arrow: String,
    green_arrow: String,
}

impl Git {
    fn new() -> Git {
        Git {
            error: false,
            red_arrow: Red.paint("➜").to_string(),
            green_arrow: Green.paint("➜").to_string(),
        }
    }

    fn check(&mut self) -> &mut Git {
        use std::path::Path;

        print!("Checking");
        self.error = Path::new("./.git").exists();

        if self.error {
            print!(" {} Error.", self.red_arrow);
            print!("\n✖ Git already exist in this directory");
        } else {
            print!(" {} Ok", self.green_arrow);
        }

        self
    }

    fn init(&mut self) -> &mut Git {
        if !self.error {
            print!(", Init");
            let mut command = Command::new("git");

            command.arg("init");

            if command.output().is_ok() {
                print!(" {} Ok", self.green_arrow);
            } else {
                print!(" {} Error.", self.red_arrow);
                print!("\n✖️ Failed to execute process: git init");

                self.error = true;
            }
        }
        self
    }

    fn config_name(&mut self, name: &String) -> &mut Git {
        if !self.error {
            print!(", Config:Name");
            let mut name_config = Command::new("git");

            name_config
                .arg("config")
                .arg("user.name")
                .arg(name);

            if name_config.output().is_ok() {
                print!(" {} Ok", self.green_arrow);
            } else {
                print!(" {} Error.", self.red_arrow);
                print!("\n✖️ Failed to execute process: git config user.name {}", name);

                self.error = true;
            }
        }
        self
    }

    fn config_email(&mut self, email: &String) -> &mut Git {
        if !self.error {
            print!(", Config:Email");

            let mut email_config = Command::new("git");
            email_config
                .arg("config")
                .arg("user.email")
                .arg(email);

            if email_config.output().is_ok() {
                print!(" {} Ok", self.green_arrow);
            } else {
                print!(" {} Error.", self.red_arrow);
                print!("\n✖️ Failed to execute process: git config user.email {}", email);
                self.error = true;
            }
        }
        self
    }

    fn print_help(&mut self) -> &Git {
        println!("USAGE: ginit");
        println!("USAGE: ginit [NAME] [EMAIL]");
        println!();
        println!("      --help     display this help and exit");
        println!("      --version  output version information and exit");
        println!();
        println!("Examples:");

        println!("  ginit Linus torwalds@email");
        self
    }

    fn print_version(&mut self) -> &Git {
        println!("version x.xx (YYYY-MM-DD) © 2019 Ramiz Abdullayev");
        self
    }

    fn finish(&mut self) {
        println!()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_size = env::args().count();


    let mut git = Git::new();

    if args_size < 2 {
        git
            .check()
            .init();
    } else if args_size == 2 {
        let command = &args[1];

        match command.as_ref() {
            "--version" | "-v" => { git.print_version(); }
            "--help" | "-h" => { git.print_help(); }

            _ => {
                println!("ginit: invalid command\n");
                git.print_help();
            }
        }
    } else if args_size == 3 {
        let name = &args[1];
        let email = &args[2];

        git
            .check()
            .init()
            .config_name(name)
            .config_email(email);
    } else {
        println!("ginit: invalid command");
        git.print_help();
    }


    git.finish();
}
