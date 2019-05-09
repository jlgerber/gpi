use serde::{Serialize,Deserialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Serialize, Deserialize)]
pub struct GpiFile {
    pub name: String
}

impl Display for GpiFile {

    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name.split(".").next().unwrap())
    }
}

impl GpiFile {
    /// Test to see if a GpiFile represents a particular package
    pub fn is(&self, package: &str) -> bool {
        let tst = self.name.split(".").next().unwrap() ;
       tst == package
    }
}