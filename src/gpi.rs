use crate::Record;
use crate::GpiError;
use shellfn::shell;
use failure;
use serde_json;
use crate::constants::*;
use crate::GpiFile;

pub struct GPI;

impl GPI {
    /// Retrieve record from Global Package Index
    ///
    /// # Parameters
    ///
    /// * `package` Name of package to retrieve record for
    ///
    /// # Returns
    ///
    /// Result wrapping a Record if successful, and a GpiError if not successful
    pub fn get_record(package: &str, verbose: bool) -> Result<Record, GpiError> {
        let result = GPI::get_json(package, verbose)?;
        if result == "{\"message\":\"404 File Not Found\"}" {
            return Err(GpiError::MissingPackage(package.to_string()))
        }

        let record: Record = serde_json::from_str(result.as_str())?;

        Ok(record)
    }

    /// Retrieve package information from GPI as json
    ///
    /// # Parameters
    ///
    /// * `package` Name of package to look up in GPI
    /// * `verbose` print out details
    ///
    /// # Returns
    ///
    /// Result wrapping a Json String, if successful, or a GpiError if not successful
    pub fn get_json(package: &str, verbose: bool) -> Result<String, GpiError> {
        let request = format!("{}://{}/api/{}/projects/{}/repository/files/packages%2F{}%2Ejson/raw?ref=master",
            PROTOCOL, HOSTNAME, API_VERSION, PROJECT_ID, package ) ;
            if verbose {
                println!("Request: '{}'", request);
            }
        let result = _get_package_json(request.as_str())?;
        Ok(result)
    }


    /// Retrieve package list from GPI as json
    ///
    /// # Parameters
    ///
    /// * `verbose` print out details
    ///
    /// # Returns
    ///
    /// Result wrapping Vec<GpiFile>, if successful, or a GpiError if not successful
    pub fn get_packages(page: usize, verbose: bool) -> Result<Vec<GpiFile>, GpiError> {
        let request = format!("{}://{}/api/{}/projects/{}/repository/tree?ref=master&path=packages&per_page=1000&page={}",
            PROTOCOL, HOSTNAME, API_VERSION, PROJECT_ID, page ) ;
        if verbose {
            println!("get_packages route: {}", request.as_str());
        }
        let mut requests = _get_packages_json(request.as_str())?;

        let  result: Vec<GpiFile> = serde_json::from_str(requests.next().ok_or("no gpi files")?.as_str())?;

        Ok(result)
    }
}

// retrieve the json from
#[shell]
fn _get_package_json(request: &str) -> Result<String, failure::Error> {
    r#"
        /usr/bin/curl -s --request GET "$REQUEST"
    "#
}

#[shell]
fn _get_packages_json(request: &str) -> Result<impl Iterator<Item=String>, failure::Error> {
    r#"
        /usr/bin/curl -s --request GET "$REQUEST"
    "#
}