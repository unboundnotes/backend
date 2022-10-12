#[cfg(test)]
mod tests {
    use std::{collections::HashMap, error::Error};

    use appconfig_derive::*;

    struct MockDataSource {
        data: HashMap<String, String>,
    }

    impl MockDataSource {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }
    }

    impl DataSource for MockDataSource {
        fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
            Ok(self.data.get(key).map(|s| s.to_string()))
        }

        fn set(&mut self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
            self.data.insert(key.to_string(), value.to_string());
            Ok(())
        }
    }

    #[derive(AppConfig)]
    pub struct ConfigStr {
        field: String,
    }

    #[test]
    fn it_reads_from_ds() {
        let mut data_src = MockDataSource::new();
        data_src.set("field", "hello").unwrap();

        let config = ConfigStr::build(&mut data_src).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[test]
    fn it_reads_from_env() {
        let mut data_src = MockDataSource::new();
        std::env::set_var("field", "hello");

        let config = ConfigStr::build(&mut data_src).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[test]
    fn ds_takes_precedence() {
        let mut data_src = MockDataSource::new();
        data_src.set("field", "hello").unwrap();
        std::env::set_var("field", "world");

        let config = ConfigStr::build(&mut data_src).unwrap();
        assert_eq!(config.field, "hello");
    }
}
