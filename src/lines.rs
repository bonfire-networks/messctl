use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;
use std::process::exit;
use crate::*;

pub fn get_refs(lines: &[Line], package: &str) -> Vec<usize> {
    lines.iter().enumerate().filter(|(_, line)| {
        match line {
            Line::Enabled(a) => a.package.name == package,
            Line::Disabled(i) => i.package.name == package,
            _ => false,
        }
    }).map(|(i,_)| i).collect()
}

pub fn find_line<'a>(lines: &'a [Line], package: &str) -> Result<(&'a Line, usize), FindError> {
    let refs = get_refs(lines, package);
    match refs.len() {
        0 => Err(FindError::Missing),
        1 => Ok((&lines[refs[0]], refs[0])),
        _ => Err(FindError::OccursMany),
    }
}

pub fn write_file(lines: &[Line], path: &Path) {
    match File::create(path) {
        Ok(mut file) => {
            for line in lines {
                write!(file, "{}\n", &line).unwrap();
            }
        }
        Err(error) => {
            println!("Error writing {:?}: {}", path, error);
            exit(1);
        }
    }
}

pub fn write_lines(lines: &[Line], path: &Path) -> Result<(), Error> {
    let mut file = File::create(path)?;
    for line in lines {
        write!(file, "{}\n", &line)?;
    }
    Ok(())
}

pub fn add_new_line(package: &str, version: &str, lines: &mut Vec<Line>, path: &Path) -> Result<(), ChangeError> {
    match find_line(&*lines, package) {
        Ok((_, _)) => Err(ChangeError::AlreadyExists),
        Err(FindError::OccursMany) => Err(ChangeError::OccursMany),
        Err(FindError::Missing) => {
            adding(package, version, path);
            let package = Package::new(package, version);
            lines.push(Line::Enabled(Enabled::new("", "", package)));
            write_lines(&*lines, path).map_err(ChangeError::IO)?;
            Ok(())
        }
    }
}

pub fn add_or_update_line(package: &str, version: &str, lines: &mut Vec<Line>, path: &Path) -> Result<(), ChangeError> {
    match find_line(&*lines, package) {
        Ok((_, index)) => {
            lines[index].update(version, path);
            write_lines(&*lines, path).map_err(ChangeError::IO)?;
            Ok(())
        }
        Err(FindError::Missing) => {
            adding(package, version, path);
            let package = Package::new(package, version);
            lines.push(Line::Enabled(Enabled::new("", "", package)));
            write_lines(&*lines, path).map_err(ChangeError::IO)?;
            Ok(())
        }
        Err(FindError::OccursMany) => Err(ChangeError::OccursMany)
    }
}

pub fn update_existing_line(package: &str, version: &str, lines: &mut Vec<Line>, path: &Path) -> Result<(), ChangeError> {
    match find_line(&*lines, package) {
        Err(FindError::Missing) => Err(ChangeError::Missing),
        Err(FindError::OccursMany) => Err(ChangeError::OccursMany),
        Ok((_, index)) => {
            lines[index].update(version, path);
            write_lines(&*lines, path).map_err(ChangeError::IO)?;
            Ok(())
        }
    }
}

pub fn disable_existing_line(package: &str, lines: &mut Vec<Line>, path: &Path) -> Result<(), ChangeError> {
    match find_line(&*lines, package) {
        Ok((Line::Disabled(_), _)) => {
            println!("{:?}: already disabled", path);
            Ok(())
        }
        Ok((Line::Enabled(e), index)) => {
            println!("{:?}: disabling", path);
            lines[index] = Line::Disabled(e.clone().disable());
            write_lines(&*lines, path).map_err(ChangeError::IO)?;
            Ok(())
        }
        Err(FindError::Missing) => Err(ChangeError::Missing),
        Err(FindError::OccursMany) => Err(ChangeError::OccursMany),
        _ => unreachable!(),
    }
}

pub fn disable_line_if_present(package: &str, lines: &mut Vec<Line>, path: &Path) -> Result<(), ChangeError> {
    match find_line(&*lines, package) {
        Ok((Line::Disabled(_), _)) => {
            println!("{:?}: already disabled", path);
            Ok(())
        }
        Ok((Line::Enabled(e), index)) => {
            println!("{:?}: disabling", path);
            lines[index] = Line::Disabled(e.clone().disable());
            write_lines(&*lines, path).map_err(ChangeError::IO)?;
            Ok(())
        }
        Err(FindError::Missing) => {
            println!("{:?}: not present", path);
            Ok(())
        }
        Err(FindError::OccursMany) => Err(ChangeError::OccursMany),
        _ => unreachable!(),
    }
}

pub fn enable_existing_line(package: &str, lines: &mut Vec<Line>, path: &Path) -> Result<(), ChangeError> {
    match find_line(&*lines, package) {
        Ok((Line::Disabled(d), index)) => {
            println!("{:?}: enabling", path);
            lines[index] = Line::Enabled(d.clone().enable());
            write_lines(&*lines, path).map_err(ChangeError::IO)?;
            Ok(())
        }
        Ok((Line::Enabled(_), _)) => {
            println!("{:?}: already enabled", path);
            Ok(())
        }
        Err(FindError::Missing) => Err(ChangeError::Missing),
        Err(FindError::OccursMany) => Err(ChangeError::OccursMany),
        _ => unreachable!(),
    }
}

pub fn enable_line_if_present(package: &str, lines: &mut Vec<Line>, path: &Path) -> Result<(), ChangeError> {
    match find_line(&*lines, package) {
        Ok((Line::Disabled(d), index)) => {
            println!("{:?}: enabling", path);
            lines[index] = Line::Enabled(d.clone().enable());
            write_lines(&*lines, path).map_err(ChangeError::IO)?;
            Ok(())
        }
        Ok((Line::Enabled(_), _)) => {
            println!("{:?}: already enabled", path);
            Ok(())
        }
        Err(FindError::Missing) => {
            println!("{:?}: not present", path);
            Ok(())
        }
        Err(FindError::OccursMany) => Err(ChangeError::OccursMany),
        _ => unreachable!(),
    }
}

