#[cfg(test)]
mod tests {
    use std::{collections::HashMap, error::Error};

    use appconfig_derive::*;

    fn return_four() -> i32 {
        return 4;
    }

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

    #[derive(AppConfig)]
    pub struct ConfigAttrs {
        #[appconfig(default = "default", name = "attr_field")]
        field: String,
        #[appconfig(default = 4)]
        field2: i32,
    }

    #[derive(AppConfig)]
    pub struct ConfigDefaultFn {
        #[appconfig(default_fn = return_four)]
        field3: i32,
    }

    #[test]
    fn it_reads_from_ds() {
        let mut data_src = MockDataSource::new();
        data_src.set("FIELD", "hello").unwrap();

        let config = ConfigStr::build(&mut data_src).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[test]
    fn it_reads_from_env() {
        let mut data_src = MockDataSource::new();
        std::env::set_var("FIELD", "hello");

        let config = ConfigStr::build(&mut data_src).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[test]
    fn ds_takes_precedence() {
        let mut data_src = MockDataSource::new();
        data_src.set("FIELD", "hello").unwrap();
        std::env::set_var("FIELD", "world");

        let config = ConfigStr::build(&mut data_src).unwrap();
        assert_eq!(config.field, "hello");
        std::env::remove_var("FIELD");
    }

    #[test]
    fn it_reads_name_from_attrs() {
        let mut data_src = MockDataSource::new();
        data_src.set("ATTR_FIELD", "hello").unwrap();

        let config = ConfigAttrs::build(&mut data_src).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[test]
    fn it_reads_default_from_attrs() {
        let mut data_src = MockDataSource::new();

        let config = ConfigAttrs::build(&mut data_src).unwrap();
        assert_eq!(config.field, "default");
        assert_eq!(config.field2, 4);
    }

    #[test]
    fn it_reads_default_from_fn() {
        let mut data_src = MockDataSource::new();

        let config = ConfigDefaultFn::build(&mut data_src).unwrap();
        assert_eq!(config.field3, 4);
    }
}
