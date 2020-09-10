use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="mess")]
pub enum Opt {
    Add {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="DEPSPEC")]
        version: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
    },
    #[structopt(alias="up")]
    Update {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="DEPSPEC")]
        version: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
    },
    #[structopt(alias="del")]
    Delete {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
    },
    #[structopt(alias="en")]
    Enable {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
    },
    #[structopt(alias="dis")]
    Disable {
        #[structopt(name="PACKAGE")]
        package: String,
        #[structopt(name="FILES", parse(from_os_str), required=true, min_values=1)]
        files: Vec<PathBuf>,
    },
}
