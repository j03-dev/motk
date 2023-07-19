use std::{collections::HashMap, fs::File, io::Read};

use pyo3::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Dom {
    pub nom: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DEM {
    #[serde(rename = "M")]
    pub m: String,
    #[serde(rename = "DOM")]
    pub dom: Dom,
    #[serde(rename = "LVF")]
    pub lvf: bool,
}

pub fn read_json_file(path: &str) -> Vec<DEM> {
    let data = std::fs::read_to_string(path).expect("Failed to Read JSON file");
    let list_dem: Vec<DEM> = serde_json::from_str(&data).expect("Failed to parse JSON data");
    list_dem
}

#[derive(Clone)]
pub struct LexicalFieldFinder {
    list_dem: Vec<DEM>,
}

impl LexicalFieldFinder {
    pub fn new(path: &str) -> Self {
        let list_dem = read_json_file(path);
        Self { list_dem }
    }

    pub fn find_lexical_fields(&self, word: String) -> Vec<String> {
        let result = self
            .list_dem
            .iter()
            .filter(|item| item.m == word)
            .collect::<Vec<_>>();

        let mut champs: Vec<String> = Vec::new();
        if let Some(r) = result.get(0) {
            for it in &self.list_dem {
                if it.dom.nom == r.dom.nom && it.m.split(' ').collect::<Vec<&str>>().len() == 1 {
                    champs.push(String::from(&it.m))
                }
            }
        }
        champs
    }
}

#[derive(Clone)]
struct TextProcessor {}

impl TextProcessor {
    fn new() -> Self {
        Self {}
    }

    fn load_stopwords(&self, file_path: &str) -> Vec<String> {
        let mut file = File::open(file_path).expect("failed to open this file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("failed to read to string file");
        buf.split('\n')
            .map(|line| line.to_string())
            .filter(|w| !w.is_empty())
            .collect()
    }

    fn clean_text_from_punctuation(&self, text: &str) -> String {
        text.chars()
            .map(|l| {
                if l.is_alphabetic() || l == ' ' {
                    l
                } else {
                    ' '
                }
            })
            .collect::<String>()
            .replace("  ", " ")
    }

    fn filter_text_and_get_wordlist(&self, text: &str, stop_word_file_path: &str) -> Vec<String> {
        let text = self.clean_text_from_punctuation(text);
        let stop_word = self.load_stopwords(stop_word_file_path);
        text.split(' ')
            .filter(|word| !word.is_empty() && !(stop_word.contains(&String::from(*word))))
            .map(|w| w.to_lowercase())
            .collect()
    }
}

fn find_most_occurrences(word_list: Vec<String>, lex: LexicalFieldFinder, n: usize) -> Vec<String> {
    let mut word_occurrences: HashMap<String, i32> = HashMap::new();
    for word in word_list.iter() {
        let champs = lex.find_lexical_fields(word.clone());
        for champ in champs {
            if word_list.contains(&champ) {
                let value = word_occurrences.get(word).unwrap_or(&0) + 1;
                word_occurrences.insert(word.clone(), value);
            }
        }
    }

    let mut sorted_word_occurrences: Vec<(&String, &i32)> = word_occurrences.iter().collect();
    sorted_word_occurrences.sort_by(|a, b| b.1.cmp(a.1));

    sorted_word_occurrences
        .iter()
        .map(|(key, _value)| String::from(*key))
        .take(n)
        .collect()
}

#[pyclass]
#[derive(Clone)]
struct MotKFinder {
    text_processor: TextProcessor,
    lex: LexicalFieldFinder,
    stop_word: String,
}

#[pymethods]
impl MotKFinder {
    #[new]
    fn new(dem: String, stop_word: String) -> PyResult<Self> {
        let text_processor = TextProcessor::new();
        let lex = LexicalFieldFinder::new(&dem);
        Ok(Self {
            text_processor,
            lex,
            stop_word,
        })
    }

    fn find_keywords(&self, text: String, n: usize) -> Vec<String> {
        let word_list = self
            .text_processor
            .filter_text_and_get_wordlist(&text, &self.stop_word);
        find_most_occurrences(word_list, self.lex.clone(), n)
    }
}

#[pymodule]
fn motk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MotKFinder>()?;
    Ok(())
}
