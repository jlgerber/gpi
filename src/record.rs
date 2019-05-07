//! record.rs
//!
//! Global Package Index Record

use serde::{Serialize,Deserialize};
use std::path::PathBuf;
use crate::VcsType;
use url::Url;
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
    pub rtype: String,
    pub sources: Vec<Source>
}

impl Record {
    pub fn new<I: Into<String>>(record_type: I, sources: Vec<Source>) -> Record {
        Self {
            rtype: record_type.into(),
            sources
        }
    }


    /// Given a tag name, determine whether the Record contains a tag
    pub fn has_vcs(&self, vcs: VcsType) -> bool {
        for source in self.sources.iter() {
            if source.uses == vcs {
                return true;
            }
        }
        false
    }

    /// Given a tag name, determine whether the Record contains a tag
    pub fn has_tag(&self, tag: &str) -> bool {
        for source in self.sources.iter() {
            let pieces = source.tags.split("/");
            for piece in pieces {
                if tag == piece {
                    return true;
                }
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
    pub tags: String,
    pub subdirectory: Option<PathBuf>,
    #[serde(rename = "initSubmodules")]
    pub init_submodules: Option<bool>,
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
            rtype: "source".to_string(),
            sources: vec![
                Source {
                    uses: VcsType::Svn,
                    link: Url::parse("http://dd-svn.d2.com/svn/software/packages/prez").unwrap(),
                    tags: "tags/%".to_string(),
                    subdirectory: None,
                    init_submodules: None,
                }
            ]
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_new_up() {
        let record = Record::new("source", Vec::new());
        let expected = Record {
            rtype: "source".to_string(),
            sources: Vec::new()
        };
        assert_eq!(record, expected);
    }

    #[test]
    fn has_tag() {
        let record: Record = serde_json::from_str(JSON_RECORD).unwrap();
        assert!(record.has_vcs(VcsType::Svn));
        assert!(!record.has_vcs(VcsType::Git));
    }

}