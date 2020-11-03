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

    #[serde(
        rename = "interpreter-version",
        skip_serializing_if = "Option::is_none"
    )]
    pub interpreter_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<Contributor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_trigger: Option<BuildTrigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<Scripts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<Notify>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_import_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notify {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "failure")]
    pub failure: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BuildTrigger {
    #[serde(rename = "every")]
    Every(String),
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contributor {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Scripts {
    #[serde(rename = "preBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_build: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "postBuild")]
    pub post_build: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preSCM")]
    pub pre_scm: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::v1::Project;

    #[test]
    fn parse_api_version_v1() -> Result<(), serde_yaml::Error> {
        let data = r#"
---
apiVersion: build.rhone.io/v1
name: simple-go
version: v1.dev
language: golang
interpreter-version: 1.13.6
build_trigger: none
"#;

        println!("yaml with version 1: \n {}", data);
        let project: Project = serde_yaml::from_str(&data)?;
        println!("project: \n {:?}", project);
        //assert_eq!(project.name, "express-train");
        assert_eq!(project.api_version, "build.rhone.io/v1");

        Ok(())
    }
}
