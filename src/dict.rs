use crate::model::Answer;
use lazy_static::lazy_static;
use std::collections::HashMap;

fn read_all() -> Vec<Answer> {
    let data = include_str!("../data/all.json");
    serde_json::from_str(data).unwrap()
}

fn read_answers() -> Vec<Answer> {
    let data = include_str!("../data/high-frequency.json");
    serde_json::from_str(data).unwrap()
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
