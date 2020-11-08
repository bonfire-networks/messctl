use std::path::PathBuf;
use structopt::StructOpt;
use crate::*;

#[derive(Debug, StructOpt)]
#[structopt(name="messflow")]
pub enum Opt {
    /// Fork a dependency locally from git.
    ///
    /// Repo and branch are optional. If not provided we will attempt
    /// to source them from `deps.git`. Values provided here override
    /// those. Disables the dep in `deps.hex` and `deps.git` if present.
    #[structopt(alias="fo")]
    Fork {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(long="repo", short="r", name="REPO")]
        repo: Option<String>,
        #[structopt(long="branch", short="b", name="BRANCH")]
        branch: Option<String>,
        /// Directory forks are being kept in, defaults to "forks"
        #[structopt(long="forks", short="f", name="DIR", parse(from_os_str))]
        forks_dir: Option<PathBuf>,
    },

    /// Use an existing fork of a library.
    ///
    /// Disables the dep in `deps.hex` and `deps.git` if present.
    #[structopt(alias="bo")]
    Borrow {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="PATH", parse(from_os_str), required=true)]
        path: PathBuf,
    },

    /// Stop using a fork of a library 
    #[structopt(alias="ret")]
    Return {
        #[structopt(name="PACKAGE")]
        package: String,
        /// If set, updates to use hex, if not, git.
        #[structopt(long, short)]
        hex: bool,
    }

}

pub fn run() {
    run_opt(Opt::from_args());
}

pub fn run_opt(opt: Opt) {
    use Opt::*;
    match opt {
        Fork { package, repo, branch, forks_dir } => {
            fork(&package, repo, branch, forks_dir);
        }
        Borrow { package, path } => {
            borrow(&package, &path);
        }
        Return { package, hex } => {
            return_(&package, hex);
        }
    }
}
