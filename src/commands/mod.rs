use clap::{Clap, ValueHint};
use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead},
    path::PathBuf,
    sync::Mutex,
};

use super::ProgState;

#[derive(Clap, Debug)]
pub struct Cat {
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub path: Option<PathBuf>,
}

impl Cat {
    pub fn run(path: Option<PathBuf>, state: &Mutex<ProgState>) -> Result<(), Box<dyn Error>> {
        match path {
            Some(path) => {
                let contents = fs::read_to_string(path)?;

                println!("{}", contents);

                Ok(())
            }
            None => {
                while state.lock().unwrap().child_running {
                    let stdin = io::stdin();
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line).unwrap();
                    println!("{}", line);
                };
                Ok(())
            }
        }
    }
}

#[derive(Clap, Debug)]
pub struct Ls {
    #[clap(name = "directory", parse(from_os_str), default_value = ".")]
    pub dir: PathBuf,
}

impl Ls {
    pub fn run(dir: PathBuf) -> Result<(), Box<dyn Error>> {
        if dir.is_dir() {
            for e in fs::read_dir(dir)? {
                let path = e?.path();
                // if path.is_dir() {
                // 	Ls::run(path)?;
                // }
                let str_path = &path.to_str().unwrap()[2..];
                print!("{} ", str_path);
            }
        }
        Ok(())
    }
}

#[derive(Clap, Debug)]
pub struct Echo {
    #[clap(multiple = true, default_value = "")]
    pub text: Vec<String>,
}

impl Echo {
    pub fn run(text: Vec<String>) -> Result<(), Box<dyn Error>> {
        println!("{}", text.join(" "));
        Ok(())
    }
}

#[derive(Clap, Debug)]
pub struct Pwd {}

impl Pwd {
    pub fn run() -> Result<(), Box<dyn Error>> {
        let pwd = env::var("PWD").unwrap_or_default();
        println!("{}", pwd);

        Ok(())
    }
}

#[derive(Clap, Debug)]
pub struct Cd {
    #[clap(name = "directory", parse(from_os_str), default_value = "~")]
    pub dir: PathBuf,
}

impl Cd {
    pub fn run(dir: PathBuf) -> Result<(), Box<dyn Error>> {
        env::set_var("PWD", dir.clone());
        assert_eq!(
            env::var("PWD"),
            Ok(String::from(dir.to_str().unwrap_or_default()))
        );

        Ok(())
    }
}
