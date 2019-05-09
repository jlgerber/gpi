use gpi::GPI;
use gpi::VcsType;
use gpi::table_record;

use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "gpi", about = "global package index")]
enum GpiCmd {
    /// List the packages in the Global Package Index.
    #[structopt(name = "list")]
    List,
    /// Query for a package in global package index
    #[structopt(name = "query")]
    Query {
        /// The name of the package to search for
        #[structopt(name = "PACKAGE")]
        package: String,
        #[structopt(short = "t", long="table")]
        table: bool,
         #[structopt(short = "j", long="json")]
        json: bool,
         #[structopt(short = "p", long="pretty-json")]
        pjson: bool,
    },
    /// Create a new package in the global package index
    #[structopt(name = "new")]
    New {
        /// The name of the source code management system used (git | svn)
         #[structopt(short = "s", long="scm")]
        scm: VcsType,
        /// The name of the package to create
        #[structopt(name = "PACKAGE")]
        package: String,
    },
    /// Add a repo for a package in the global package index
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
        GpiCmd::List => {
            // ge
        },
        GpiCmd::Query {package, table, json, pjson } => {
            let pkg = GPI::get_record(&package)?;
            if table || (!table && !json && !pjson) {
                table_record(&package, &pkg);
            } else if json {
                println!("{:?}", pkg);
            } else if pjson {
                println!("{:#?}", pkg);
            }
        },
        GpiCmd::New { scm, package,  } => {
            println!("Creating the package named '{}' in the '{}' source code management system is not supported yet",  package, scm);
        },
        GpiCmd::Add { scm, package  } => {
            println!("Adding '{}' to the '{}' record in the GPI not supported yet", scm, package);
        }
    }
    Ok(())
}

fn main() -> Result<(), gpi::GpiError> {
    process_cmds()
}
