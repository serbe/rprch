use std::{
    collections::{hash_map, HashMap},
    convert::TryFrom,
    str::FromStr,
};

use crate::error::ChError;

#[derive(Debug, PartialEq, Clone)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new() -> Headers {
        Headers(HashMap::new())
    }

    pub fn iter(&self) -> hash_map::Iter<String, String> {
        self.0.iter()
    }

    pub fn get<T: ToString + ?Sized>(&self, k: &T) -> Option<String> {
        match self.0.get(&k.to_string().to_lowercase()) {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    pub fn insert<T: ToString + ?Sized, U: ToString + ?Sized>(
        &mut self,
        key: &T,
        val: &U,
    ) -> Option<String> {
        self.0
            .insert(key.to_string().to_lowercase(), val.to_string())
    }

    pub fn remove<T: ToString + ?Sized>(&mut self, key: &T) -> Option<String> {
        self.0.remove(&key.to_string().to_lowercase())
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Headers {
    type Err = ChError;

    fn from_str(s: &str) -> Result<Headers, ChError> {
        let headers = s.trim();

        if headers.is_empty() {
            Err(ChError::EmptyHeader)
        } else if headers.lines().all(|e| e.contains(':')) {
            let headers = headers
                .lines()
                .map(|elem| {
                    let idx = elem.find(':').unwrap();
                    let (key, value) = elem.split_at(idx);
                    (
                        key.to_string().to_lowercase(),
                        value[1..].trim().to_string(),
                    )
                })
                .collect();

            Ok(Headers(headers))
        } else {
            Err(ChError::HeaderParse(headers.to_owned()))
        }
    }
}

impl TryFrom<Option<&str>> for Headers {
    type Error = ChError;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        Headers::from_str(value.unwrap_or(""))
    }
}

impl From<HashMap<String, String>> for Headers {
    fn from(map: HashMap<String, String>) -> Headers {
        let headers = map
            .iter()
            .map(|(key, value)| (key.to_string().to_lowercase(), value.to_string()))
            .collect();
        Headers(headers)
    }
}

impl TryFrom<std::str::Lines<'_>> for Headers {
    type Error = ChError;

    fn try_from(lines: std::str::Lines<'_>) -> Result<Self, Self::Error> {
        let headers: String = lines.into_iter().collect();
        Headers::from_str(&headers)
    }
}
