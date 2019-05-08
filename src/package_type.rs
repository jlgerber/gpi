use serde::{Serialize,Deserialize};
use std::convert::From;

/// The type as defined in the plobal packages index
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Source,
    Port,
    Unknown,
}

impl From<&str> for PackageType {
    fn from(value: &str) -> PackageType {
        match value {
            "source" | "Source" | "SOURCE" => PackageType::Source,
            "port" | "Port" | "PORT"       => PackageType::Port,
            _                              => PackageType::Unknown,
        }
    }
}

impl From<String> for PackageType {
    fn from(value: String) -> PackageType {
        PackageType::from(value.as_ref())
    }
}

impl std::fmt::Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        match self {
            &PackageType::Source =>  write!(f, "source"),
            &PackageType::Port =>  write!(f, "port"),
            &PackageType::Unknown =>  write!(f, "unknown"),
        }
    }
}

// impl PackageType {
//     pub fn tag_for_vcs(&self, vcs: VcsType) ->
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_from_string() {
        let result = PackageType::from("source".to_string());
        assert_eq!(result, PackageType::Source);
    }

    #[test]
    fn can_convert_from_str_ref() {
        let tst = "source";
        let result = PackageType::from(tst);
        assert_eq!(result, PackageType::Source);
    }
}