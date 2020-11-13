use pyo3::{prelude::*, wrap_pyfunction};

use std::fs;
mod v1;
mod v2;
use v1::Project as ProjectV1;
use v2::Project as ProjectV2;

/// Read yaml file and return json as string
#[pyfunction]
fn get_json_from_yaml_file(path: String) -> PyResult<String> {
    let contents = fs::read_to_string(&path)?;
    get_json_from_yaml_str(contents)
}

/// Read yaml as string  and return json as string
#[pyfunction]
fn get_json_from_yaml_str(contents: String) -> PyResult<String> {
    let json;

    //if contents.starts_with("build.rhone.io/v2") {
    //        let project: ProjectV2 = serde_yaml::from_str(&contents).unwrap();
    //}
    if contents.contains("build.rhone.io/v2") {
        //println!("parsing project v2");
        let project: ProjectV2 = serde_yaml::from_str(&contents).unwrap();
        json=serde_json::to_string(&project).unwrap();

    }else{
        //println!("parsing project v1");
        let project: ProjectV1 = serde_yaml::from_str(&contents).unwrap();

        json = serde_json::to_string(&project).unwrap();
    }
    
    
    Ok(json)
}

#[pymodule]
fn rhone_rusty_yaml(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_json_from_yaml_file))?;
    m.add_wrapped(wrap_pyfunction!(get_json_from_yaml_str))?;
    Ok(())
}
