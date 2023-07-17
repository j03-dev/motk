use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

use pyo3::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Dom {
    pub nom: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DEM {
    pub M: String,
    pub DOM: Dom,
    pub LVF: bool,
}

pub fn read_json_file(path: &str) -> Vec<DEM> {
    let data = std::fs::read_to_string(path).unwrap();
    let list_dem: Vec<DEM> = serde_json::from_str(&data).unwrap();
    list_dem
}

#[derive(Clone)]
pub struct FamillyFinder {
    list_dem: Vec<DEM>,
}

impl FamillyFinder {
    pub fn new(path: &str) -> Self {
        let list_dem = read_json_file(path);
        Self { list_dem }
    }

    pub fn find_champs(&self, word: String) -> Vec<String> {
        let result = self
            .list_dem
            .iter()
            .filter(|item| item.M == word)
            .collect::<Vec<_>>();

        let mut champs: Vec<String> = Vec::new();
        if let Some(r) = result.get(0) {
            for it in &self.list_dem {
                if it.DOM.nom == r.DOM.nom && it.M.split(' ').collect::<Vec<&str>>().len() == 1 {
                    champs.push(String::from(&it.M))
                }
            }
        }
        champs
    }
}

fn is_in<T: PartialEq>(elt: T, list: Vec<T>) -> bool {
    !list
        .iter()
        .filter(|item| **item == elt)
        .collect::<Vec<_>>()
        .is_empty()
}

#[derive(Clone)]
struct TextFilter {}

impl TextFilter {
    fn new() -> Self {
        Self {}
    }

    fn get_stopwords(&self, file_path: &str) -> Vec<String> {
        let mut file = File::open(file_path).expect("failed to open this file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("failed to read to string file");
        buf.split('\n')
            .map(|line| line.to_string())
            .filter(|w| !w.is_empty())
            .collect()
    }

    fn remove_punctuation(&self, text: &str) -> String {
        let mut new_text = String::new();
        for l in text.chars() {
            if l.is_alphabetic() || l.to_string() == " " {
                new_text += l.to_string().as_str();
            } else {
                new_text += " ";
            }
        }
        new_text.replace("  ", " ")
    }

    fn get_filter_wordlist(&self, text: &str, stop_word_file_path: &str) -> Vec<String> {
        let text = self.remove_punctuation(text);
        let stop_word = self.get_stopwords(stop_word_file_path);
        text.split(' ')
            .filter(|word| !word.is_empty() && !(is_in(word.to_lowercase(), stop_word.clone())))
            .map(|w| w.to_lowercase())
            .collect()
    }
}

fn most_occurate(
    word_list: Arc<Mutex<Vec<String>>>,
    familly_finder: FamillyFinder,
    n: usize,
) -> Vec<String> {
    let mut poids: HashMap<String, i32> = HashMap::new();
    let word_list = &mut word_list.lock().unwrap();
    for word in word_list.iter() {
        let champs = familly_finder.find_champs(word.clone());
        for champ in champs {
            if is_in(champ.clone(), word_list.to_vec()) {
                let value = poids.get(word).unwrap_or(&0) + 1;
                poids.insert(word.clone(), value);
            }
        }
    }

    let mut sorted_poids: Vec<(&String, &i32)> = poids.iter().collect();
    sorted_poids.sort_by(|a, b| b.1.cmp(a.1));

    println!("{sorted_poids:?}");

    sorted_poids
        .iter()
        .map(|(key, _value)| String::from(*key))
        .take(n)
        .collect()
}

#[pyclass]
#[derive(Clone)]
struct MotK {
    text_filter: TextFilter,
    familly_finder: FamillyFinder,
    stop_word: String,
}

#[pymethods]
impl MotK {
    #[new]
    fn new(dem: String, stop_word: String) -> PyResult<Self> {
        let text_filter = TextFilter::new();
        let familly_finder = FamillyFinder::new(&dem);
        Ok(Self {
            text_filter,
            familly_finder,
            stop_word,
        })
    }

    fn find_key_words(&self, text: String, n: usize) -> Vec<String> {
        let word_list = self.text_filter.get_filter_wordlist(&text, &self.stop_word);
        most_occurate(
            Arc::new(Mutex::new(word_list)),
            self.familly_finder.clone(),
            n,
        )
    }
}

#[pymodule]
fn motk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MotK>()?;
    Ok(())
}
