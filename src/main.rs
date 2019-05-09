use gpi::{ GPI, table_record, VcsType };
use structopt::StructOpt;

/// Commands to interact with the Global Package Index, which
/// provides a means to look up the url of a package within
/// one of our heterogeneous version control systems, reducing
/// friction associated with relocating packages within a vcs or
/// moving pakcages between vcs systems (eg svn to git).
#[derive(StructOpt, Debug)]
#[structopt(name = "gpi", )]
enum GpiCmd {
    /// List the packages in the Global Package Index.
    #[structopt(name = "list")]
    List {
        #[structopt(short = "v", long="verbose")]
        verbose: bool,
        #[structopt(short = "H", long="has")]
        package: Option<String>,
    },
    /// Query for a package in Global Package Index
    #[structopt(name = "query")]
    Query {
        /// The name of the package to search for
        #[structopt(name = "PACKAGE")]
        package: String,
        /// Print results as a table
        #[structopt(short = "t", long="table")]
        table: bool,
        /// Print results as json
        #[structopt(short = "j", long="json")]
        json: bool,
        /// Print results as pretty json
        #[structopt(short = "p", long="pretty-json")]
        pjson: bool,
        #[structopt(short = "v", long="verbose")]
        verbose: bool,
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
        GpiCmd::List{verbose, package} => {
            if let Some(p) = package {
                let mut found = false;
                for page in 0..11 {
                    let packages = GPI::get_packages(page, verbose)?;
                    let has = packages.iter().any(|x| x.is(p.as_str()));
                    if has {
                        println!("true");
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("false");
                }
            } else {

                for page in 0..11 {
                    let packages = GPI::get_packages(page, verbose)?;
                    for package in packages {
                        println!("{}", package);
                    }
                }
            }
        },
        GpiCmd::Query {package, table, json, pjson, verbose } => {
            let pkg = GPI::get_record(&package, verbose)?;
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
