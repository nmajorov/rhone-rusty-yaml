use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs;

use serde::{Deserialize, Serialize};

/// yaml repesentation of rhone  project
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
    pub interpreter_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<Contributor>>,
    pub build_trigger: BuildTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Scripts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BuildTrigger {
     #[serde(rename = "every")]
     Every(String),
     #[serde(rename = "none")]
     None
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contributor {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub struct Scripts {
    #[serde(rename = "preBuild")]
    pub pre_build: String,
  
    pub build: String,
  
    #[serde(rename = "postBuild")]
    pub post_build: String,
    
    #[serde(rename = "preSCM")]
    pub pre_scm: String,
}



/// Read yaml file and return json as string
#[pyfunction]
fn get_json_from_yaml_file(path:String) -> PyResult <String> {
    


    println!("try to parse file: {}", &path);   

    let contents = fs::read_to_string(&path)?;
    
    let project: Project = serde_yaml::from_str(&contents).unwrap();
   
    let j = serde_json::to_string(&project).unwrap();
    Ok(j)

}

/// Read yaml as string  and return json as string
#[pyfunction]
fn get_json_from_yaml_str(contents:String) -> PyResult <String> {
    
   
    let project: Project = serde_yaml::from_str(&contents).unwrap();
   
    let j = serde_json::to_string(&project).unwrap();
    Ok(j)

}


#[pymodule]
fn rhone_rusty_yaml(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_json_from_yaml_file))?;
    m.add_wrapped(wrap_pyfunction!(get_json_from_yaml_str))?;
    Ok(())
}

#[cfg(test)]
mod tests {
 

use super::{Contributor, Project, Scripts,BuildTrigger};

    #[test]
    fn it_works() -> Result<(), serde_json::Error> {
        let contr1 = Contributor {
            name: "Nikolaj Majorov".to_string(),
            email: "nikolaj@majorov.biz".to_string(),
        };

        let script = Scripts {
            pre_build: String::from("run-prebuild.zsh"),
            build: String::from("build"),
            post_build: String::from("run-build.sh"),
            pre_scm: String::from("run-preSCM.zsh"),
        };

       // let trigger= BuildTrigger::Every(String::from("3 hours"));

        let trigger= BuildTrigger::None;


        let project = Project {
            api_version: String::from("build.rhone.io/v1"),
            name: String::from("rust-yaml"),
            description: Some(String::from("simple rest project")),
            language: String::from("rust"),
            version: String::from("1.0.1"),
            interpreter_version: Some(String::from("1.39.0")),
            contributors: Some(vec![contr1]),
            scripts: Some(script),
            build_trigger: trigger,
        };

        let yaml = serde_yaml::to_string(&project);
        match yaml {
            Ok(yaml) => println!("yaml: \n {}", yaml),
            Err(e) => println!("error yaml {}",e) 

        } 
        

        let j = serde_json::to_string(&project)?;
        println!("json: \n {}", j);

        Ok(())

    }

    #[test]
    fn parse_yaml_as_string() ->  Result<(), serde_yaml::Error>{
        let data = r#"
---
name: express-train
apiVersion: build.rhone.io/v1
description: some framework
version: 2.1.3
language: python
interpreter-version: '3.4'
contributors:
- name: Nikolaj Majorov
  email: nikolaj@majorov.biz
- name: Oleg Mayer
  email: oleg@majorov.biz
repository:
    brache: develop
build_trigger:
    every: 5 minutes
notify:
    success: 'false'
    failure: 'true'
"#;

        println!("yaml: \n {}", data);
        let project:Project = serde_yaml::from_str(&data)?;
        assert_eq!(project.name, "express-train" );

        Ok(())
    }

    #[test]
    fn parse_yaml_build_trigger_none() ->  Result<(), serde_yaml::Error>{
        let data = r#"
---
apiVersion: build.rhone.io/v1
name: simple-go
version: v1.dev
language: golang
interpreter-version: 1.13.6
build_trigger: none
"#;

        println!("yaml: \n {}", data);
        let project:Project = serde_yaml::from_str(&data)?;
        assert_eq!(project.name, "simple-go" );
        //assert_eq!(project.build_trigger, BuildTrigger::None);
        Ok(())
    }
}
