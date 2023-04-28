// #[derive(serde::Deserialize, Debug)]
// pub struct ShortUrlConfig {
//     pub reserved_words: String,
// }

// impl ShortUrlConfig {
//     pub fn reserved_words(&self) -> Vec<&str> {
//         self.reserved_words.split(',').collect()
//     }
//     pub fn in_reserved_words(&self, word: &str) -> bool {
//         for w in self.reserved_words() {
//             if w == word {
//                 return true;
//             }
//         }
//         false
//     }
// }

#[derive(serde::Deserialize, Debug)]
pub struct Web {
  pub addr: String
}

/// 应用配置
#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub web: Web,
}


impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
