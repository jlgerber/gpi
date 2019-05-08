use serde::{Serialize,Deserialize};
use std::convert::From;

/// The type as defined in the global packages index
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Serialize, Deserialize)]
pub enum VcsTag {
    #[serde(rename = "tags/%")]
    TagsLoc,
    #[serde(rename = "%")]
    TrunkLoc,
    Unknown(String),
}

impl From<&str> for VcsTag {
    fn from(value: &str) -> Self {
        match value {
            "tags/%" => VcsTag::TagsLoc,
            "%"      => VcsTag::TrunkLoc,
            _        => VcsTag::Unknown(value.to_string()),
        }
    }
}


impl From<String> for VcsTag {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "tags/%" => VcsTag::TagsLoc,
            "%"      => VcsTag::TrunkLoc,
            _        => VcsTag::Unknown(value),
        }
    }
}

impl std::string::ToString for VcsTag {
    fn to_string(&self) -> String {
        match self {
            VcsTag::TagsLoc => "tags/%".to_string(),
            VcsTag::TrunkLoc => "%".to_string(),
            VcsTag::Unknown(value) => format!("unknown({})", value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Serialize, Deserialize)]
    struct TagTest {
        tag: VcsTag
    }

    #[test]
    fn can_convert_from_str_ref() {
        let result = VcsTag::from("tags/%");
        assert_eq!(result, VcsTag::TagsLoc);
        let result = VcsTag::from("%");
        assert_eq!(result, VcsTag::TrunkLoc);
    }

    #[test]
    fn can_convert_from_string() {
        let result = VcsTag::from("tags/%".to_string());
        assert_eq!(result, VcsTag::TagsLoc);
        let result = VcsTag::from("%".to_string());
        assert_eq!(result, VcsTag::TrunkLoc);
    }

    #[test]
    fn can_deserialize_tagsloc_from_json() {
        let json = r#"{ "tag": "tags/%" }"#;
        let result: TagTest = serde_json::from_str(json).unwrap();

        assert_eq!(result, TagTest{tag: VcsTag::TagsLoc});
    }

    #[test]
    fn can_deserialize_trunkloc_from_json() {
        let json = r#"{ "tag": "%" }"#;
        let result: TagTest = serde_json::from_str(json).unwrap();

        assert_eq!(result, TagTest{tag: VcsTag::TrunkLoc});
    }
}