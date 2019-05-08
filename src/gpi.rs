use crate::Record;
use crate::GpiError;
use shellfn::shell;
use failure;
use serde_json;
use crate::constants::*;

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
    pub fn get_record(package: &str) -> Result<Record, GpiError> {
        let result = GPI::get_json(package)?;
        let record: Record = serde_json::from_str(result.as_str())?;

        Ok(record)
    }

    /// Retrieve package information from GPI as json
    ///
    /// # Parameters
    ///
    /// * `package` Name of package to look up in GPI
    ///
    /// # Returns
    ///
    /// Result wrapping a Json String, if successful, or a GpiError if not successful
    pub fn get_json(package: &str) -> Result<String, GpiError> {
        let request = format!("{}::/{}/api/{}/projects/{}/repository/files/packages%2F{}%2Ejson/raw?ref=master",
            PROTOCOL, HOSTNAME, API_VERSION, PROJECT_ID, package ) ;
        let result = _get_package_json(request.as_str())?;

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