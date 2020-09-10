
mod opts;
use opts::Opt;

mod types;
use types::*;

mod parser;
use parser::*;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use structopt::StructOpt;

fn get_refs(lines: &[Line], package: &str) -> Vec<usize> {
    lines.iter().enumerate().filter(|(_, line)| {
        match line {
            Line::Enabled(a) => a.package.name == package,
            Line::Disabled(i) => i.package.name == package,
            _ => false,
        }
    }).map(|(i,_)| i).collect()
}

fn write_file(lines: &[Line], path: &Path) {
    if let Ok(mut file) = File::create(path) {
        for line in lines {
            write!(file, "{}\n", &line).unwrap();
        }
    }
}

fn adding(package: &str, version: &str, file: &Path) {
    println!("Adding package {} at version {} to file {:?}", package, version, file);
}

fn error_does_not_occur(package: &str, file: &Path) {
    println!("Error: Package {} does not occur in file {:?}", package, file);
    exit(1);
}

fn error_occurs_many(count: usize, package: &str, file: &Path) {
    println!("Error: Package {} occurs {} times in file {:?}", package, count, file);
    exit(1);
}

fn exactly_once(count: usize, package: &str, file: &Path) {
    match count {
        1 => (),
        0 => error_does_not_occur(package, file),
        _ => error_occurs_many(count, package, file),
    }
}

fn main() {
    let opt = Opt::from_args();
    use Opt::*;
    match opt {
        Add { package, version, files } => {
            for f in files {
                let mut lines = parse_file(&f);
                let refs = get_refs(&lines, &package);
                match refs.len() {
                    0 => {
                        adding(&package, &version, &f);
                        let package = Package::new(&package, &version);
                        lines.push(Line::Enabled(Enabled::new("", "", package)));
                        write_file(&lines, &f);
                    }
                    1 => {
                        lines[refs[0]].update(&version, &f);
                        write_file(&lines, &f);
                    }
                    count => error_occurs_many(count, &package, &f),
                }
            }
        }
        Delete { package, files } => {
            for f in files {
                let mut lines = parse_file(&f);
                let mut refs = get_refs(&lines, &package);
                refs.reverse();
                if !refs.is_empty() {
                    println!("Deleting package {} from file {:?}", &package, &f);
                }
                for r in refs {
                    lines.remove(r);
                }
                write_file(&lines, &f);
            }
        }
        Update { package, version, files } => {
            for f in files {
                let mut lines = parse_file(&f);
                let refs = get_refs(&lines, &package);
                match refs.len() {
                    0 => {
                        adding(&package, &version, &f);
                        let package = Package::new(&package, &version);
                        lines.push(Line::Enabled(Enabled::new("", "", package)));
                        write_file(&lines, &f);
                    }
                    1 => {
                        lines[refs[0]].update(&version, &f);
                        write_file(&lines, &f);
                    }
                    count => error_occurs_many(count, &package, &f),
                }
            }
        }
        Enable { package, files } => {
            for f in files {
                let mut lines = parse_file(&f);
                let refs = get_refs(&lines, &package);
                exactly_once(refs.len(), &package, &f);
                lines[refs[0]] = lines[refs[0]].clone().enable(&package, &f);
                write_file(&lines, &f);
            }
        }
        Disable { package, files } => {
            for f in files {
                let mut lines = parse_file(&f);
                let refs = get_refs(&lines, &package);
                exactly_once(refs.len(), &package, &f);
                lines[refs[0]] = lines[refs[0]].clone().disable(&package, &f);
                write_file(&lines, &f);
            }
        }
    }
}
