use std::path::PathBuf;
use std::process::{exit, Command};
use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches, ArgSettings};
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = App::new("checkexec")
        .version(VERSION)
        .about("Conditionally run a command.\
jeffre
        This b
        ")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::new("target")
            .about("The file created by this checkexec execution.")
        )
        .arg(Arg::new("verbose")
            .long("verbose")
            .short('v')
            .takes_value(false)
        )
        .arg(Arg::new("infer")
            .long("infer")
            .takes_value(false)
            .about("Infer the dependency list. The inference takes all arguments to the command, filters it for files, and uses that list.\
             --infer causes checkexec to fail if it creates an empty dependency list.")
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

    let verbose = args.is_present("verbose");

    let target = args.value_of("target").unwrap();
    let mut dependencies = args.values_of("dependencies").unwrap_or_default().map(str::to_string).collect::<Vec<String>>();
    if args.is_present("infer") {
        if !dependencies.is_empty() {
            eprintln!("--infer is mutually exclusive with explicitly listed dependencies.");
            exit(1);
        }
        let command_args = args.values_of("command").unwrap().into_iter().skip(1).collect::<Vec<&str>>();
        dependencies = command_args.iter()
            .filter_map(|s| fs::metadata(s).ok().map(|m| s.to_string()))
            .collect::<Vec<String>>();
        if dependencies.is_empty() {
            eprintln!("checkexec requires --infer to find at least one dependency, but it found no accessible files from the provided command args. Command arguments are: {}",
                command_args.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<String>>().join(" ")
            );
            exit(1);
        }
        if verbose {
            eprintln!("Found {} dependencies for {}: {}", dependencies.len(), target, dependencies.join(" "));
        }
    }

    let mut needs_execute = false;
    let mut has_dependencies = false;

    match fs::metadata(target) {
        Ok(meta) => {
            for dependency in dependencies {
                has_dependencies = true;
                let dep_meta = fs::metadata(&dependency).expect(&format!("Failed to check metadata for dependency {}", dependency));
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
        if verbose {
            eprintln!("Executing command: {} {}", command, command_and_args.map(|s| format!("\"{}\"", s)).collect::<Vec<String>>().join(" "));
        }
        let output = Command::new(command)
            .args(command_and_args)
            .output().expect(&format!("Failed to execute command {}", command))
        exit(output.status.code().unwrap());
    }
}