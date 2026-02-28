use crate::model::Answer;
use lazy_static::lazy_static;
use std::{collections::HashMap, fs, path::PathBuf};

fn data_dir() -> PathBuf {
    "/data".into()
}

fn read_answers_from(file_name: &str) -> Vec<Answer> {
    let path = data_dir().join(file_name);
    let data = fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!("failed to read {}: {}", path.display(), err);
    });

    serde_json::from_str(&data).unwrap_or_else(|err| {
        panic!("failed to parse {}: {}", path.display(), err);
    })
}

fn read_all() -> Vec<Answer> {
    read_answers_from("all.json")
}

fn read_answers() -> Vec<Answer> {
    read_answers_from("high-frequency.json")
}

fn reserve_index() -> HashMap<String, usize> {
    let mut res = HashMap::new();

    for (i, v) in DICT.iter().enumerate() {
        res.entry(v.word.clone()).or_insert(i);
    }

    res
}

lazy_static! {
    pub static ref DICT: Vec<Answer> = read_all();
    pub static ref ANSWERS: Vec<Answer> = read_answers();
    pub static ref REVERSE_ANSWERS: HashMap<String, usize> = reserve_index();
}
