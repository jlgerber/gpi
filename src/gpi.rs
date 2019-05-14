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
                println!("get_json() Request: '{}'", request);
            }
        let result = _get_package_json(request.as_str())?;
        Ok(result)
    }

    /// Test to see whether a package with the supplied name exists
    ///
    /// # Parameters
    ///
    /// * `package` Name of package to look up in GPI
    /// * `verbose` print out details
    ///
    /// # Returns
    ///
    /// Result wrapping a bool, if successful, or a GpiError if not successful
    pub fn package_exists(package: &str, verbose: bool) -> Result<bool, GpiError> {
        let request = format!("{}://{}/api/{}/projects/{}/repository/files/packages%2F{}%2Ejson?ref=master",
            PROTOCOL, HOSTNAME, API_VERSION, PROJECT_ID, package ) ;
            if verbose {
                println!("package_exists() Request: '{}'", request);
            }
        let result = _package_exists(request.as_str())?;
        if result == "{\"message\":\"404 File Not Found\"}" {
            return Ok(false);
        }
        Ok(true)
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
            println!("get_packages() route: {}", request.as_str());
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

// retrieve the json from
#[shell]
fn _package_exists(request: &str) -> Result<String, failure::Error> {
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

/*
curl --request POST --header 'PRIVATE-TOKEN: <your_access_token>' --header "Content-Type: application/json" \
  --data '{"branch": "master", "author_email": "author@example.com", "author_name": "Firstname Lastname", \
    "content": "some content", "commit_message": "create a new file"}' \
  'https://gitlab.example.com/api/v4/projects/13083/repository/files/app%2Fproject%2Erb'
 */