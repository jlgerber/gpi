use std::{fmt::Display, str::FromStr, fmt};
use crate::GpiError;
use serde::{Serialize, Deserialize};
use std::convert::From;

#[serde(rename_all = "lowercase")]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum VcsType {
    Git,
    Svn,
    Rcs,
    Perforce,
    Mercurial,
}

impl Display for VcsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vcs = match self {
            VcsType::Git => "git",
            VcsType::Svn => "svn",
            VcsType::Rcs => "rcs",
            VcsType::Perforce => "perforce",
            VcsType::Mercurial => "mercurial",
        };

        write!(f, "{}", vcs)
    }
}

impl AsRef<VcsType> for VcsType {
    fn as_ref(&self) -> &VcsType {
        return self
    }
}

impl From<&str> for VcsType {
    fn from(input: &str) -> VcsType {
        match input.to_lowercase().as_ref() {
            "git" => VcsType::Git,
            "svn" => VcsType::Svn,
            "rcs" => VcsType::Rcs,
            "perforce" => VcsType::Perforce,
            "mercurial" =>  VcsType::Mercurial,
            _ => panic!("unknown type: {}", input),
        }
    }
}

impl From<String> for VcsType {
    fn from(input: String) -> VcsType {
        match input.to_lowercase().as_str() {
            "git" => VcsType::Git,
            "svn" => VcsType::Svn,
            "rcs" => VcsType::Rcs,
            "perforce"=> VcsType::Perforce,
            "mercurial" =>  VcsType::Mercurial,
            _ => panic!("unknown type: {}", input),
        }
    }
}

impl FromStr for VcsType {
    type Err = GpiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "git" => Ok(VcsType::Git),
            "svn" => Ok(VcsType::Svn),
            "rcs" => Ok(VcsType::Rcs),
            "perforce"=> Ok(VcsType::Perforce),
            "mercurial" =>  Ok(VcsType::Mercurial),
            _ => Err(GpiError::UnknownVcs(s.to_owned())),
        }
    }
}
