use clap::{App, AppSettings, Clap, IntoApp};
use flush::{Cat, Cd, Echo, Ls, Pwd};
use std::{
    io::{self, BufRead},
    path::PathBuf,
    process,
    str::FromStr,
};

#[derive(Clap, Debug)]
#[clap(
    version = "0.0.1",
    author = "Trevor R. <tjdoescode@gmail.com>",
    about = "A $%@!tty shell"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

impl Default for Opts {
    fn default() -> Self {
        Opts { subcmd: None }
    }
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(
        version = "0.0.1",
        author = "Trevor R. <tjdoescode@gmail.com",
        about = "Concatenate the contents of a file to standard output"
    )]
    Cat(Cat),
    #[clap(
        version = "0.0.1",
        author = "Trevor R. <tjdoescode@gmail.com",
        about = "List contents of a directory"
    )]
    Ls(Ls),
    #[clap(
        version = "0.0.1",
        author = "Trevor R. <tjdoescode@gmail.com",
        about = "Display a line of text to standard output"
    )]
    Echo(Echo),
    #[clap(
        version = "0.0.1",
        author = "Trevor R. <tjdoescode@gmail.com",
        about = "Print the path of the current woking directory"
    )]
    Pwd(Pwd),
    #[clap(
        version = "0.0.1",
        author = "Trevor R. <tjdoescode@gmail.com",
        about = "Change the current woking directory"
    )]
    Cd(Cd),
}

impl FromStr for SubCommand {
    type Err = ();
    fn from_str(input: &str) -> Result<SubCommand, Self::Err> {
        let str_ver = input.to_uppercase();
        let mut iter = str_ver.split_whitespace();
        match iter.next() {
            Some("CAT") => {
                let maybe_path = iter.next();
                match maybe_path {
                    Some(path) => Ok(SubCommand::Cat(Cat {
                        path: Some(PathBuf::from(path)),
                    })),
                    None => Ok(SubCommand::Cat(Cat { path: None })),
                }
            }
            Some("LS") => Ok(SubCommand::Ls(Ls {
                dir: PathBuf::from(iter.next().unwrap_or("")),
            })),
            Some("ECHO") => {
                let text: Vec<String> = iter.map(|t| t.to_owned()).collect();
                Ok(SubCommand::Echo(Echo { text }))
            }
            _ => Err(()),
        }
    }
}

fn main() {
    let opts = Opts::parse();
    println!("{:#?}", opts);

    let result = match opts.subcmd {
        Some(SubCommand::Cat(c)) => Cat::run(c.path),
        Some(SubCommand::Ls(c)) => Ls::run(c.dir),
        Some(SubCommand::Echo(c)) => Echo::run(c.text),
        Some(SubCommand::Cd(c)) => Cd::run(c.dir),
        Some(SubCommand::Pwd(_)) => Pwd::run(),
        None => loop {
            println!("inner");
            let mut app = Opts::into_app();
            let stdin = io::stdin();
            let mut line = String::new();

            app = app.setting(AppSettings::NoBinaryName);
            stdin.lock().read_line(&mut line).unwrap();

            let parsed = app.try_get_matches_from(line.trim().split_whitespace());
            println!("{:#?}", parsed);

            if let Err(e) = &parsed {
                eprintln!("Parsing error: {}", e);
            }
            let result;
            match parsed.unwrap_or_default().subcommand() {
                Some(("cat", sub_m)) => {
                    let path: PathBuf = sub_m.value_of("path").unwrap().into();
                    result = Cat::run(Some(path));
                }
                // Some(SubCommand::Ls(c)) => Ls::run(c.dir),
                // Some(SubCommand::Echo(c)) => Echo::run(c.text),
                // Some(SubCommand::Cd(c)) => Cd::run(c.dir),
                // Some(SubCommand::Pwd(_)) => Pwd::run(),
                _ => println!("Command not found"),
            };
            if let Err(err) = result {
                eprintln!("Error: {}", err);
            }
        },
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
