use crate::{ constants::{ API_VERSION, PROJECT_ID , PROTOCOL, HOSTNAME, DOMAIN}, GpiError, Record };
use reqwest::Response;
use serde::{Serialize, Deserialize};
use url::Url;
use std::str::FromStr;

/// The server in which we store the gpi
pub struct GitlabServer {}

impl GitlabServer {
    pub fn new() -> Self {
        Self {}
    }
    fn get_package_route(&self, package:&str) -> String {
        // dont think i have to do this --------------->
        //"api/{}/projects/{}/repository/files/packages%2F{}%2Ejson"
        format!(
            "api/{}/projects/{}/repository/files/packages/{}.json",
            API_VERSION,
            PROJECT_ID,
            package
        )
    }

    fn get_new_package_route(&self, package: &str) -> Result<Url, url::ParseError> {
        Url::from_str(format!(
                "{}://{}/{}",
                PROTOCOL,
                HOSTNAME,
                self.get_package_route(package).as_str()
            ).as_str()
        )
    }

    /// Given a login, package name, and record, post a new package
    pub fn post_package(&self, login: &str, package: &str, record: Record) -> Result<reqwest::Response, GpiError> {
        let route = self.get_new_package_route(package)?;
        let request_struct = PackagePostRequest::new(DOMAIN, login, record);
        let client = reqwest::Client::new();
        let results =
        client
        .post(route.as_str())
        .json(&request_struct)
        .send()?;

        Ok(results)
    }
}

// {"branch": "master", "author_email": "author@example.com", "author_name": "Firstname Lastname", \
///    "content": "some content", "commit_message": "create a new file"}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PackagePostRequest {
    branch: String,
    author_email: String,
    author_name: String,
    content: Record,
    commit_message: String,
}

impl PackagePostRequest {
    pub fn new<I: Into<String>>(domain: &str, login: I, content: Record) -> Self {
        let login = login.into();
        Self {
            branch: "master".to_string(),
            author_email: format!("{}.{}", login.as_str(), domain),
            author_name: login,
            content,
            commit_message: "package generated via gpi".to_string()
        }
    }
}