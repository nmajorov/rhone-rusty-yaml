use serde::{Deserialize, Serialize};

/// yaml repesentation of rhone  project version2

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(
        rename = "interpreter-version",
        skip_serializing_if = "Option::is_none"
    )]
    pub interpreter_version: Option<String>,

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
    pub pre_install: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_install: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_build: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scm: Option<Vec<String>>,
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
language: python
interpreter-version: 3
"#;

        println!("yaml: \n {}", data);
        let project: Project = serde_yaml::from_str(&data)?;
        println!("project v2 : \n {:?}", project);
        //assert_eq!(project.name, "express-train");
        assert_eq!(project.api_version, "build.rhone.io/v2");

        Ok(())
    }

    #[test]
    fn parse_image_scripts() -> Result<(), serde_yaml::Error> {
        let data = r#"
---
apiVersion: build.rhone.io/v2
name: express-train
version: 2.1.3
image: "registry.redhat.io/rhel8/python-38"
env:
  - FOO=foo
  - BAR=bar
  - FOO=bar
  - BAR=foo
scripts:
  cms:
    - git clone -l -s -n . ../copy
    - cd ../copy
    - git show-branch
  pre_install:
    - env
    - podman version
  pre_build:
    - pip install ansible
  build:
    - python setup.py install
    - pytest -v tests
  post_build:
    - ls
    - ./scripts/call.py
  post_install:
     - ls
     - ./script/delete_pod.sh

"#;

        println!("yaml: \n {}", data);
        let project: Project = serde_yaml::from_str(&data)?;
        println!("project v2 : \n {:?}", project);
        //assert_eq!(project.name, "express-train");
        assert_eq!(project.api_version, "build.rhone.io/v2");

        Ok(())
    }

    // parse golang project with scripts
    #[test]
    fn parse_golang_project() -> Result<(), serde_yaml::Error>{
        let yaml=r#"
apiVersion: build.rhone.io/v2
name: kivotion-server-backend
version: 1.0.0.dev0
language: golang
interpreter-version: 1.13.6
go_import_path: bitbucket.org/kivotion/server
scripts:
    pre_build:
      - make dep
    build:
      - make test
"#;


        println!("yaml: \n {}", yaml);
        let project: Project = serde_yaml::from_str(&yaml)?;
        println!("project v2 golang with gopath : \n {:?}", project);
        //assert_eq!(project.name, "express-train");
        assert_eq!(project.api_version, "build.rhone.io/v2");

        Ok(())
    }
}
