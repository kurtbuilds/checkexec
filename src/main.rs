use std::fmt::{Display};
use std::path::{Path};
use std::process::{exit, Command};

use clap::{App, AppSettings, Arg};
use std::fs;

const VERSION: &str = env!("CARGO_PKG_VERSION");


struct Error {
    message: String,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

macro_rules! err {
    ($($arg:tt)*) => {
        Error {
            message: format!($($arg)*),
        }
    }
}


fn infer_dependencies<'a>(command: &[&'a str]) -> Result<Vec<&'a str>, Error> {
    let inferred_deps = command.iter()
        .filter_map(|s| fs::metadata(s).ok().map(|_| *s))
        .collect::<Vec<&str>>();
    if inferred_deps.is_empty() {
        Err(err!("--infer must find at least one accessible file in command arguments. Command arguments are: {}",
                  command.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<String>>().join(" ")
        ))
    } else {
        Ok(inferred_deps)
    }
}


fn should_execute<T: AsRef<Path> + Display>(target: &str, dependencies: &[T]) -> Result<bool, Error> {
    match fs::metadata(target) {
        Ok(meta) => {
            let modified = meta.modified().unwrap();
            for dependency in dependencies {
                let dep_meta = fs::metadata(&dependency)
                    .map_err(|e| err!("{}: Could not read file metadata", &dependency))?;
                if dep_meta.modified().unwrap() > modified {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        Err(_) => Ok(true)
    }
}


fn main() -> std::result::Result<(), Error> {
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
            .required(true)
        )
        .arg(Arg::new("verbose")
            .long("verbose")
            .short('v')
            .takes_value(false)
        )
        .arg(Arg::new("infer")
            .long("infer")
            .takes_value(false)
            .conflicts_with("dependencies")
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
    let command_args = args.values_of("command").unwrap().into_iter().skip(1).collect::<Vec<&str>>();
    let dependencies = if args.is_present("infer") {
        infer_dependencies(&command_args)?
    } else {
        args.values_of("dependencies").map(|d| d.collect::<Vec<&str>>()).unwrap_or_default()
    };

    if should_execute(target, &dependencies)? {
        let command = args.values_of("command").unwrap().collect::<Vec<&str>>();
        if verbose {
            eprintln!("{} {}", command[0], command.iter().skip(1).map(|s| format!("\"{}\"", s)).collect::<Vec<String>>().join(" "));
        }
        let output = Command::new(command[0])
            .args(command[1..].iter())
            .status()
            .map_err(|_| err!("{}: command not found", command[0]))?;
        exit(output.code().unwrap());
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use std::io::Write;
    use super::*;
    use tempfile::{TempDir, tempdir};

    struct TempFiles {
        #[allow(dead_code)]
        dir: TempDir,
        pub files: Vec<String>,
    }

    fn touch(path: &str) -> std::io::Result<()> {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(b"")
    }

    fn touch_and_untouch(touched: usize, untouched: usize) -> TempFiles {
        let tempdir = tempdir().unwrap();
        let dir = tempdir.path();
        let mut files: Vec<String> = Vec::new();
        files.extend((0..touched).map(|i| dir.join(i.to_string()).to_str().unwrap().to_string()));
        files.extend((touched..(touched + untouched)).map(|i| dir.join(i.to_string()).to_str().unwrap().to_string()));
        for file in files.iter().take(touched) {
            touch(file).unwrap();
            // tries to eliminate ties between files. 1ms should be more than enough
            // and we dont need a ton of tests for this program where 1ms is noticeable.
            // apparently 1ms isn't enough for github actions because ...... reasons?
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        TempFiles {
            dir: tempdir,
            files,
        }
    }

    #[test]
    fn test_infer_dependencies() {
        let TempFiles { dir: _dir, files } = touch_and_untouch(3, 0);
        let dependencies = infer_dependencies(&["cc",
            &files[0],
            &files[1]]).unwrap();
        assert_eq!(dependencies, vec![&files[0], &files[1]]);
    }

    #[test]
    fn test_no_inferred_dependencies_errors() {
        let TempFiles { dir: _dir, files } = touch_and_untouch(0, 1);
        assert!(infer_dependencies(&["cc",
            &files[0]]).is_err())
    }

    #[test]
    fn test_should_execute_errors_on_failed_dependency_access() {
        let TempFiles { dir: _dir, files } = touch_and_untouch(1, 1);
        assert!(should_execute(&files[0], &files[1..]).is_err(), "Should have failed to access file");
    }

    #[test]
    fn test_should_execute_target_doesnt_exist() {
        let TempFiles { dir: _dir, files } = touch_and_untouch(1, 1);
        assert!(should_execute(&files[1], &files[0..1]).unwrap(), "Should execute because target doesn't exist");
    }

    #[test]
    fn test_should_not_execute_newer_target() {
        let TempFiles { dir: _dir, files } = touch_and_untouch(2, 0);
        assert!(!should_execute(&files[1], &files[0..1]).unwrap(), "Should not execute because target is newer");
    }

    #[test]
    fn test_should_execute_newer_dependencies() {
        let TempFiles { dir: _dir, files } = touch_and_untouch(2, 0);
        assert!(should_execute(&files[0], &files[1..]).unwrap())
    }
}
