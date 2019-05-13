//! record.rs
//!
//! Global Package Index Record

use serde::{Serialize,Deserialize};
use std::path::PathBuf;
use std::io::Read;
use crate::GpiError;
use crate::PackageType;
use crate::VcsTag;
use crate::VcsType;
use url::Url;
use std::str::FromStr;

/*
 {
  "type": "source",
  "sources": [
    {
      "uses": "svn",
      "link": "http://dd-svn.d2.com/svn/software/packages/prez",
      "tags": "tags/%"
    }
  ]
}
*/
#[serde(rename_all = "lowercase")]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "type")]
    pub package_type: PackageType,
    pub sources: Vec<Source>
}

impl Record {
    /// new up a record
    pub fn new(package_type: PackageType, sources: Vec<Source>) -> Record {
        Self {
            package_type: package_type,
            sources
        }
    }

    /// Convert a `Read`er into json.
    ///
    /// # Parameters
    ///
    /// * `reader` - Implementer of the `io::Read` trait. For instance, a File, or BufReader.
    ///
    /// # Returns
    ///
    /// A result wrapping either a Record, or a GpiError
    pub fn from_reader<R: Read>(reader: R) -> Result<Record, GpiError> {
        let r = serde_json::from_reader(reader)?;
        Ok(r)
    }

    /// Add a source, returning success/failure.
    pub fn add_source(&mut self, source: Source) -> bool {
        if !self.has_vcs(&source.uses) {
            self.sources.push(source);
            return true;
        }
        false
    }
    /// Given a tag name, determine whether the Record contains a tag
    pub fn has_vcs<T: AsRef<VcsType>>(&self, vcs: T) -> bool {
        for source in self.sources.iter() {
            if &source.uses == vcs.as_ref() {
                return true;
            }
        }
        false
    }

}

#[serde(rename_all = "lowercase")]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Source {
    pub uses: VcsType,
    #[serde(with = "url_serde")]
    pub link: Url,
    pub tags: VcsTag,
    pub subdirectory: Option<PathBuf>,
    #[serde(rename = "initSubmodules")]
    pub init_submodules: Option<bool>,
}

impl Source {
    pub fn new(
        vcs_type:VcsType,
        link: Url,
        tags:VcsTag,
        subdirectory: Option<PathBuf>,
        init_submodules: Option<bool>
    ) -> Source {
        Source {
            uses: vcs_type,
            link,
            tags,
            subdirectory,
            init_submodules
        }
    }
}


#[cfg(test)]
mod tests {

    const JSON_RECORD: &'static str =
r#"
{
  "type": "source",
  "sources": [
    {
      "uses": "svn",
      "link": "http://dd-svn.d2.com/svn/software/packages/prez",
      "tags": "tags/%"
    }
  ]
}
"#;

    use super::*;

    #[test]
    fn can_deserialize_json() {
        let result:Record = serde_json::from_str(JSON_RECORD).unwrap();
        let expected = Record {
            package_type: PackageType::Source,
            sources: vec![
                Source {
                    uses: VcsType::Svn,
                    link: Url::parse("http://dd-svn.d2.com/svn/software/packages/prez").unwrap(),
                    tags: VcsTag::TagsLoc,
                    subdirectory: None,
                    init_submodules: None,
                }
            ]
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_new_up() {
        let record = Record::new(PackageType::from_str("source").unwrap(), Vec::new());
        let expected = Record {
            package_type: PackageType::Source,
            sources: Vec::new()
        };
        assert_eq!(record, expected);
    }


    #[test]
    fn can_deserialize_from_reader() {
        let result:Record = Record::from_reader(JSON_RECORD.as_bytes()).unwrap();
        let expected = Record {
            package_type: PackageType::Source,
            sources: vec![
                Source {
                    uses: VcsType::Svn,
                    link: Url::parse("http://dd-svn.d2.com/svn/software/packages/prez").unwrap(),
                    tags: VcsTag::TagsLoc,
                    subdirectory: None,
                    init_submodules: None,
                }
            ]
        };
        assert_eq!(result, expected);
    }
}