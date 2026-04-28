use std::collections::HashMap;

#[derive(Clone)]
pub struct Locale {
    map: HashMap<String, String>,
}

impl Locale {
    fn from_json(json: &str) -> Self {
        let map: HashMap<String, String> = serde_json::from_str(json).unwrap_or_default();
        Self { map }
    }

    pub fn tr<'a>(&'a self, key: &'a str) -> &'a str {
        self.map.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    pub fn load(lang: &str) -> Self {
        match lang {
            "zh_cn" => Self::from_json(include_str!("../assets/lang/zh_cn.json")),
            _ => Self::from_json(include_str!("../assets/lang/en_us.json")),
        }
    }
}