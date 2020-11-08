pub mod parser;
pub mod types;
pub mod messctl;
pub mod messflow;

use parser::*;
pub mod lines;
use lines::*;

use std::fs::{create_dir_all, metadata};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use crate::types::*;

fn adding(package: &str, version: &str, file: &Path) {
    println!("Adding package {} at version {} to file {:?}", package, version, file);
}

fn error_occurs_many(count: usize, package: &str, file: &Path) {
    println!("Error: Package {} occurs {} times in file {:?}", package, count, file);
    exit(1);
}

fn get_version_from_file_and_maybe_disable(package: &str, path: &Path) -> String {
    let mut lines = parse_file(path);
    let refs = get_refs(&lines, &package);
    match refs.len() {
        0 => {
            println!("");
            exit(1);
        }
        1 => {
            match &lines[refs[0]] {
                Line::Enabled(e) => {
                    let v = e.package.version.clone();
                    lines[refs[0]] = Line::Disabled(e.clone().disable());
                    write_file(&lines, path);
                    v
                }
                Line::Disabled(d) => {
                    d.package.version.clone()
                }
                _ => unreachable!(),
            }
        }
        count => {
            error_occurs_many(count, package, path);
            unreachable!()
        }
    }
}

fn fork(package: &str, repo: Option<String>, branch: Option<String>, forks_dir: Option<PathBuf>) {
    parse_package(package).expect("package name to be valid");
    // find the path and branch if not provided
    let deps = PathBuf::from("deps.git");
    let version = repo.unwrap_or_else(|| get_version_from_file_and_maybe_disable(package, &deps));
    let pieces: Vec<&str> = version.split("#").collect();
    let repo = pieces[0];
    let branch = branch.or_else(|| pieces.get(1).map(|x| x.to_string()));
    // create the forks directory
    let mut dir = forks_dir.unwrap_or(PathBuf::from("forks"));
    if let Err(error) = create_dir_all(&dir) {
        println!("Error creating forks directory {:?}: {}", &dir, error);
        exit(1);
    }
    
    // run git clone
    let dir2 = dir.to_str().unwrap();
    if let Some(branch) = branch {
        let status =
            Command::new("git")
            .arg("-C")
            .arg(dir2)
            .arg("clone")
            .arg("-b")
            .arg(branch)
            .arg(repo)
            .arg(package)
            .status()
            .expect("git clone to execute successfully");
        if !status.success() {
            println!("git clone failed.");
            exit(status.code().unwrap());
        }
    } else {
        let status =
            Command::new("git")
            .arg("-C")
            .arg(dir2)
            .arg("clone")
            .arg(repo)
            .arg(package)
            .status()
            .expect("git clone to execute successfully");
        if !status.success() {
            println!("git clone failed.");
            exit(status.code().unwrap());
        }
    }
    dir.push(package);
    // add to deps.path
    let dir = dir.to_str().unwrap();
    let deps = PathBuf::from("deps.path");
    let mut lines = parse_file(&deps);
    let ret = add_or_update_line(&package, dir, &mut lines, &deps);
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, &deps);
        exit(1);
    }
    // disable hex deps
    let deps = PathBuf::from("deps.hex");
    let mut lines = parse_file(&deps);
    let ret = disable_line_if_present(&package, &mut lines, &deps);
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, &deps);
        exit(1);
    }
}

fn borrow(package: &str, path: &Path) {
    let meta = metadata(path).expect("package path to exist");
    if !meta.is_dir() {
        println!("path is not a directory!");
        exit(1);
    }
    let deps = PathBuf::from("deps.path");
    let path = path.to_str().unwrap();
    let mut lines = parse_file(&deps);
    let ret = add_or_update_line(&package, path, &mut lines, &deps);
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, &deps);
        exit(1);
    }
    let deps = PathBuf::from("deps.hex");
    let mut lines = parse_file(&deps);
    let ret = disable_line_if_present(&package, &mut lines, &deps);
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, &deps);
        exit(1);
    }
    let deps = PathBuf::from("deps.git");
    let mut lines = parse_file(&deps);
    let ret = disable_line_if_present(&package, &mut lines, &deps);    
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, &deps);
        exit(1);
    }
}
fn return_(package: &str, hex: bool) {
    if hex {
        let deps = PathBuf::from("deps.hex");
        let mut lines = parse_file(&deps);
        let ret = enable_existing_line(package, &mut lines, &deps);
        if let Err(e) = ret {
            println!("Error: {:?} in file {:?}", e, &deps);
            exit(1);
        }
        let deps = PathBuf::from("deps.git");
        let mut lines = parse_file(&deps);
        let ret = disable_line_if_present(package, &mut lines, &deps);
        if let Err(e) = ret {
            println!("Error: {:?} in file {:?}", e, &deps);
            exit(1);
        }
    } else {
        let deps = PathBuf::from("deps.git");
        let mut lines = parse_file(&deps);
        let ret = enable_existing_line(package, &mut lines, &deps);
        if let Err(e) = ret {
            println!("Error: {:?} in file {:?}", e, &deps);
            exit(1);
        }
        let deps = PathBuf::from("deps.hex");
        let mut lines = parse_file(&deps);
        let ret = disable_line_if_present(package, &mut lines, &deps);
        if let Err(e) = ret {
            println!("Error: {:?} in file {:?}", e, &deps);
            exit(1);
        }
    }
    let deps = PathBuf::from("deps.path");
    let mut lines = parse_file(&deps);
    let ret = disable_line_if_present(package, &mut lines, &deps);
    if let Err(e) = ret {
        println!("Error: {:?} in file {:?}", e, &deps);
        exit(1);
    }
}
