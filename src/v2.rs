use serde::{Deserialize, Serialize};

/// yaml repesentation of rhone  project version2

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<Contributor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_trigger: Option<BuildTrigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Scripts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<Notify>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_import_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_build: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_build_ansible: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_build_ansible: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_build: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_scm: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {

    use crate::v2::Project;

    #[test]
    fn parse_api_v2() -> Result<(), serde_yaml::Error> {
        let data = r#"
---
apiVersion: build.rhone.io/v2
name: express-train
version: 2.1.3
image: "docker.io/library/python:3"
language: python

"#;

        println!("yaml: \n {}", data);
        let project: Project = serde_yaml::from_str(&data)?;
        println!("project v2 : \n {:?}", project);
        //assert_eq!(project.name, "express-train");
        assert_eq!(project.api_version, "build.rhone.io/v2");

        Ok(())
    }
}
