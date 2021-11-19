use std::path::PathBuf;
use std::process::{exit, Command};
use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches, ArgSettings};
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = App::new("checkexec")
        .version(VERSION)
        .about("Conditionally run a command (Like Makefile functionality).\
        ")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::new("target")
            .about("The file created by this checkexec execution.")
        )
        .arg(Arg::new("dependencies").min_values(0)
            .about("The list of files")
        )
        .arg(Arg::new("command").min_values(1)
            .last(true)
            .required(true)
            .about("The command to execute if the check passes.")
        )
        .get_matches();

    let target = args.value_of("target").unwrap();
    let dependencies = args.values_of("dependencies").unwrap();

    let mut needs_execute = false;
    let mut has_dependencies = false;

    match fs::metadata(target) {
        Ok(meta) => {
            for dependency in dependencies {
                has_dependencies = true;
                let dep_meta = fs::metadata(dependency).expect(&format!("Failed to check metadata for dependency {}", dependency));
                if dep_meta.modified().unwrap() > meta.modified().unwrap() {
                    needs_execute = true;
                    break;
                }
            }
        }
        Err(_) => {
            needs_execute = true;
        }
    }

    if !has_dependencies {
        needs_execute = true;
    }

    if needs_execute {
        let mut command_and_args = args.values_of("command").unwrap().into_iter();
        let command = command_and_args.next().unwrap();
        let mut child = Command::new(command);
        child.args(command_and_args);
        let mut child = child.spawn().expect(&format!("Failed to execute command {}", command));
        let output = child.wait_with_output().unwrap();
        exit(output.status.code().unwrap());
    }
}