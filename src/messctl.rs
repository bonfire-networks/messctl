use std::path::PathBuf;
use structopt::StructOpt;
use crate::*;

#[derive(Debug, StructOpt)]
#[structopt(name="messctl")]
pub enum Opt {
    /// Add a package with the given version to some deps files, or update it.
    #[structopt(aliases=&["ad"])]
    Add {
        /// Name of package
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="DEPSPEC")]
        version: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
        /// If the package exists, suppress updating the version 
        #[structopt(long = "no-update", parse(from_flag = std::ops::Not::not))]
        update: bool,
    },

    /// Update the version of a package in some deps files, or add it.
    #[structopt(alias="up")]
    Update {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="DEPSPEC")]
        version: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
        /// If the package does not exist, suppress adding it
        #[structopt(long = "no-add", parse(from_flag = std::ops::Not::not))]
        add: bool,
    },

    /// Delete a package from some deps files if it is present.
    #[structopt(aliases=&["de", "del", "rm"])]
    Delete {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
    },

    /// Uncomment a package in some deps files
    #[structopt(alias="en")]
    Enable {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
    },

    /// Comment a package in some deps files
    #[structopt(aliases=&["di", "dis"])]
    Disable {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
    },
}

pub fn run() {
    run_opt(Opt::from_args());
}

pub fn run_opt(opt: Opt) {
    use Opt::*;
    match opt {
        Add { package, version, files, update } => {
            for f in files {
                add(&package, &version, update, &f)
            }
        }
        Delete { package, files } => {
            for f in files {
                delete(&package, &f);
            }
        }
        Update { package, version, files, add } => {
            for f in files {
                update(&package, &version, add, &f)
             }
        }
        Enable { package, files } => {
            for f in files {
                enable(&package, &f)
            }
        }
        Disable { package, files } => {
            for f in files {
                disable(&package, &f);
            }
        }
    }
}

fn add(package: &str, version: &str, update: bool, file: &Path) {
    parse_package(package).expect("package name to be valid");
    let mut lines = parse_file(file);
    let ret = if update {
        add_or_update_line(package, version, &mut lines, file)
    } else {
        add_new_line(package, version, &mut lines, file)
    };
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, file);
        exit(1);
    }
}

fn update(package: &str, version: &str, add: bool, file: &Path) {
    parse_package(package).expect("package name to be valid");
    let mut lines = parse_file(file);
    let ret = if add {
        add_or_update_line(package, version, &mut lines, file)
    } else {
        update_existing_line(package, version, &mut lines, file)
    };
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, file);
        exit(1);
    }
}

fn disable(package: &str, file: &Path) {
    parse_package(package).expect("package name to be valid");
    let mut lines = parse_file(file);
    let ret = disable_line_if_present(package, &mut lines, file);
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, file);
        exit(1);
    }
}

fn enable(package: &str, file: &Path) {
    parse_package(package).expect("package name to be valid");
    let mut lines = parse_file(file);
    let ret = enable_existing_line(package, &mut lines, file);
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, file);
        exit(1);
    }
}

fn delete(package: &str, file: &Path) {
    let mut lines = parse_file(file);
    let mut refs = get_refs(&lines, &package);
    refs.reverse();
    for r in refs {
        println!("Deleting package {} from file {:?}", &package, file);
        lines.remove(r);
    }
    write_file(&lines, file);
}

