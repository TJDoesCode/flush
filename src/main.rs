#[macro_use]
extern crate lazy_static;

use clap::Clap;
use flush::{commands::*, ProgState};
use std::{
    error::Error,
    io::{self, stdout, BufRead, Write},
    iter::once,
    process,
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
        Mutex,
    },
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

fn run_interactive(state: &Mutex<ProgState>) {
    loop {
        let stdin = io::stdin();
        let mut line = String::new();

        stdin.lock().read_line(&mut line).unwrap();

        let parsed = Opts::try_parse_from(once("flush").chain(line.trim().split_whitespace()));
        println!("{:#?}", parsed);

        if let Err(e) = &parsed {
            eprintln!("Parsing error: {}", e);
        }

        if let Some(subcmd) = parsed.unwrap_or_default().subcmd {
            if let Err(err) = dispatch_cmd(subcmd, &state) {
                eprintln!("Error: {}", err);
            }
        }
    }
}

fn dispatch_cmd(subcmds: SubCommand, state: &Mutex<ProgState>) -> Result<(), Box<dyn Error>> {
    //let mut state = state.lock().unwrap();
    //state.lock().unwrap().set_child_running(true);
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    match subcmds {
        SubCommand::Cat(c) => Cat::run(c.path, &state),
        SubCommand::Ls(c) => Ls::run(c.dir),
        SubCommand::Echo(c) => Echo::run(c.text),
        SubCommand::Cd(c) => Cd::run(c.dir),
        SubCommand::Pwd(_) => Pwd::run(),
    }
}

lazy_static! {
    static ref STATE: Mutex<ProgState> = ProgState::new();
}

fn main() {
    stdout().flush().expect("Failed to flush stdout");

    ctrlc::set_handler(|| {
        let mut state = STATE.lock().unwrap();

        state.kill_count += 1;

        match state.kill_count {
            1 => {
                if state.child_running {
                    state.set_child_running(false);
                    return println!("(Press Ctrl+C again to abort)");
                }
                process::exit(2)
            }
            2 => process::exit(2),
            _ => (),
        }
    })
    .unwrap();

    let opts = Opts::parse();
    println!("{:#?}", opts);

    if let Some(subcmd) = opts.subcmd {
        if let Err(err) = dispatch_cmd(subcmd, &STATE) {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    } else {
        run_interactive(&STATE)
    }
}
