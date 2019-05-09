use gpi::GPI;
use gpi::VcsType;
use std::env;
use gpi::table_record;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "gpi", about = "global package index")]
enum GpiCmd {
    #[structopt(name = "query")]
    Query {
        #[structopt(name = "PACKAGE")]
        package: String,
    },
    #[structopt(name = "new")]
    New {
        #[structopt(name = "PACKAGE")]
        package: String,
    },
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "s", long="scm")]
        scm: VcsType,

        #[structopt(name = "PACKAGE")]
        package: String,
    }
}

fn process_cmds() -> Result<(), gpi::GpiError> {
    match GpiCmd::from_args() {
        GpiCmd::Query {package } => {
            let pkg = GPI::get_record(&package)?;
            table_record(&package, &pkg);
        },
        GpiCmd::New { package,  } => {
            println!("create {} Not Supported Yet", package);
        },
        GpiCmd::Add { scm, package  } => {
            println!("create {} {} Not Supported Yet", scm, package);
        }
    }
    Ok(())
}

fn main() -> Result<(), gpi::GpiError> {
    process_cmds()
}
