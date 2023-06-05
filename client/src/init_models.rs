use serde::Deserialize;
use std::fs;
use std::collections::HashMap;

use crate::model::Model;

#[derive(Deserialize)]
struct LoadModels {
    modelname: String,
    path: String,
}

pub fn init_models() -> HashMap<String, Model> {
    let mut models: HashMap<String, Model> = HashMap::new();
    let j = fs::read_to_string("resources/models.json").expect("Error reading file resources/models.json");
    let loadmodels: Vec<LoadModels> = serde_json::from_str(&j).expect("Error deserializing resources/models.json");
    for loadmodel in loadmodels {
        models.insert(loadmodel.modelname, Model::new(&loadmodel.path));
    }
    models
}