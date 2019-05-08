use crate::Record;
use crate::GpiError;
use shellfn::shell;
use failure;
use serde_json;

pub struct GPI;

impl GPI {
    /// Retrieve record from Global Package Index
    pub fn get_record(package: &str) -> Result<Record, GpiError> {
        let result = GPI::get_json(package)?;
        let record: Record = serde_json::from_str(result.as_str())?;

        Ok(record)
    }

    pub fn get_json(package: &str) -> Result<String, GpiError> {
        let request = format!("http://dd-git.d2.com:/api/v4/projects/1350/repository/files/packages%2F{}%2Ejson/raw?ref=master", package);
        let result = _get_package_json(request.as_str())?;

        Ok(result)
    }
}

#[shell]
fn _get_package_json(request: &str) -> Result<String, failure::Error> {
    r#"
        /usr/bin/curl -s --request GET "$REQUEST"
    "#
}