//! pkg_cmds
//!
use crate::GpiError;
use crate::GPI;
use crate::VcsType;
use crate::VcsTag;
use crate::Record;
use crate::PackageType;
use crate::GitlabServer;
use crate::Source;

use std::env;
use std::str::FromStr;

pub fn new_package(name: &str, link: &str, ptype: &str, vcs: VcsType, verbose: bool, dryrun: bool) -> Result<Option<reqwest::Response>, GpiError> {
    // test to see if package already exists
    let exists = GPI::package_exists(name, verbose)?;
    if exists {
        return Err(GpiError::PackageLookupError(name.to_string(), "Package already exists".to_string()));
    }
    if verbose {
        println!("package does not exist. looking up url");
    }
    // lookup link
    let link = url::Url::from_str(link)?;
    println!("link: {:?}", link);
    // lookup record type
    let rtype = PackageType::from_str(name)?;
    if verbose {
        println!("PackageType {:?}", rtype);
    }
    // Lookup VcsType from scm

    if verbose {
        println!("vcs type {:?}", vcs);
    }
    // get record type
    let ptype = PackageType::from_str(ptype)?;
    if verbose {
        println!("package type: {:?}", ptype);
    }
    // build source
    let tag = match vcs {
        VcsType::Git => VcsTag::TrunkLoc,
        VcsType::Svn => {
            // if we are a port, then we are in 3ps/trunk
            if ptype == PackageType::Port {
                VcsTag::TrunkLoc
            // otherwise, we are in tags/<tag name>
            } else {

                VcsTag::TagsLoc
            }
        },
        _            => VcsTag::TrunkLoc,
    };
    if verbose {
        println!("tag: {:?}", tag);
    }
    let source = Source::new(vcs, link, tag, None, None);
    if verbose {
        println!("source: {:#?}", source);
    }
    // build record
    let record = Record::new(ptype, vec![source]);
    if verbose {
        println!("record: {:#?}", record);
    }
    let server = GitlabServer::new();
    let login = env::var("USER")?;

    if dryrun {
        println!("server.post_package({}, {}, record)?", login.as_str(), name);
        Ok(None)
    } else {
        let response = server.post_package(login.as_str(), name, record)?;
         Ok(Some(response))
    }

}
