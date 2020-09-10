use std::fmt;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
}

impl Package {
    pub fn new(name: &str, version: &str) -> Package {
        Package { name: name.to_owned(), version: version.to_owned() }
    }

    pub fn update(&mut self, version: &str, file: &Path) {
        if self.version != version {
            updating(&self.name, version, file);
            self.version = version.to_string();
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = \"{}\"", &self.name, &self.version)
    }
}

#[derive(Clone, Debug)]
pub struct Enabled {
    pub pre: String,
    pub package: Package,
    pub post: String,
}

impl Enabled {
    pub fn new(pre: &str, post: &str, package: Package) -> Enabled {
        Enabled { pre: pre.to_owned(), post: post.to_owned(), package }
    }

    pub fn disable(self) -> Disabled {
        Disabled { pre: "# ".to_string(), package: self.package, post: self.post }
    }

    pub fn update(&mut self, version: &str, file: &Path) {
        self.package.update(version, file)
    }
}

impl fmt::Display for Enabled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", &self.pre, &self.package, &self.post)
    }
}

#[derive(Clone, Debug)]
pub struct Disabled {
    pub pre: String,
    pub package: Package,
    pub post: String,
}

impl Disabled {
    pub fn new(pre: String, post: &str, package: Package) -> Disabled {
        Disabled { pre, post: post.to_owned(), package }
    }

    pub fn enable(self) -> Enabled {
        Enabled { pre: "".to_string(), package: self.package, post: self.post }
    }

    pub fn update(&mut self, version: &str, file: &Path) {
        self.package.update(version, file)
    }
}

impl fmt::Display for Disabled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", &self.pre, &self.package, &self.post)
    }
}

#[derive(Clone, Debug)]
pub enum Line {
    Enabled(Enabled),
    Disabled(Disabled),
    Ignored(String),
}

impl Line {
    pub fn enable(self, package: &str, file: &Path) -> Line {
        match self {
            Line::Disabled(d) => Line::Enabled(d.enable()),
            Line::Enabled(_) => {
                println!("Warning: Package {} already enabled in file {:?}", package, file);
                self
            }
            _ => unreachable!()
        }
    }
    pub fn disable(self, package: &str, file: &Path) -> Line {
        match self {
            Line::Enabled(e) => Line::Disabled(e.disable()),
            Line::Disabled(_) => {
                println!("Warning: Package {} already disabled in file {:?}", package, file);
                self
            }
            _ => unreachable!()
        }
    }
    pub fn update(&mut self, version: &str, file: &Path) {
        match self {
            Line::Enabled(ref mut e) => e.update(version, file),
            Line::Disabled(ref mut d) => d.update(version, file),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Line::Enabled(a) => write!(f, "{}", a),
            Line::Disabled(i) => write!(f, "{}", i),
            Line::Ignored(i) => write!(f, "{}", i),
        }
    }
}

fn updating(package: &str, version: &str, file: &Path) {
    println!("Updating package {} to version {} in file {:?}", package, version, file);
}
