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
    #[serde(skip_deserializing)]
    pub description: String,
    pub version: String,
    pub language: String,
    #[serde(skip_deserializing)]
    pub interpreter_version: String,
    #[serde(skip_deserializing)]
    pub contributors: Vec<Contributor>,
    pub build_trigger: String,
    pub scripts: Scripts,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contributor {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
fn get_json_from_yaml(path:String) -> PyResult <String> {
    




    let contents = fs::read_to_string(&path)?;
    
    let project: Project = serde_yaml::from_str(&contents).unwrap();
   
    let j = serde_json::to_string(&project).unwrap();
    Ok(j)

}


#[pymodule]
fn rhone_rusty_yaml(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_json_from_yaml))?;
    Ok(())
}

#[cfg(test)]
mod tests {
 

use super::{Contributor, Project, Scripts};

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
        let project = Project {
            api_version: String::from("build.rhone.io/v1"),
            name: String::from("rust-yaml"),
            description: String::from("simple rest project"),
            language: String::from("rust"),
            version: String::from("1.0.1"),
            interpreter_version: String::from("1.39.0"),
            contributors: vec![contr1],
            scripts: script,
            build_trigger: String::from("every 3 hours"),
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

    //fn parse_file() ->
}
