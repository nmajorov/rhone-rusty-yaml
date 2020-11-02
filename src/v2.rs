use serde::{Deserialize, Serialize};

/// yaml repesentation of rhone  project version1

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub version: String,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
