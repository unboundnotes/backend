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

        fn set(&mut self, key: &str, value: String) -> Result<(), Box<dyn Error>> {
            self.data.insert(key.to_string(), value);
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
        data_src.set("FIELD", "hello".to_string()).unwrap();

        let config = ConfigStr::build(&mut data_src, None).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[test]
    fn it_reads_from_env() {
        let mut data_src = MockDataSource::new();
        std::env::set_var("FIELD", "hello");

        let config = ConfigStr::build(&mut data_src, None).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[test]
    fn ds_takes_precedence() {
        let mut data_src = MockDataSource::new();
        data_src.set("FIELD", "world".to_string()).unwrap();
        std::env::set_var("FIELD", "hello");

        let config = ConfigStr::build(&mut data_src, None).unwrap();
        assert_eq!(config.field, "world");
    }

    #[test]
    fn it_uses_prefix() {
        let mut data_src = MockDataSource::new();
        data_src.set("PREFIX_FIELD", "hello".to_string()).unwrap();

        let config = ConfigStr::build(&mut data_src, Some("PREFIX_".to_string())).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[derive(AppConfig)]
    pub struct ConfigAttrs {
        #[appconfig(default = "default", name = "attr_field")]
        field: String,
        #[appconfig(default = 4)]
        field2: i32,
    }

    #[test]
    fn it_reads_name_from_attrs() {
        let mut data_src = MockDataSource::new();
        data_src.set("ATTR_FIELD", "hello".to_string()).unwrap();

        let config = ConfigAttrs::build(&mut data_src, None).unwrap();
        assert_eq!(config.field, "hello");
    }

    #[test]
    fn it_reads_default_from_attrs() {
        let mut data_src = MockDataSource::new();

        let config = ConfigAttrs::build(&mut data_src, None).unwrap();
        assert_eq!(config.field, "default");
        assert_eq!(config.field2, 4);
    }

    fn return_four() -> i32 {
        return 4;
    }

    #[derive(AppConfig)]
    pub struct ConfigDefaultFn {
        #[appconfig(default_fn = return_four)]
        field3: i32,
    }

    #[test]
    fn it_reads_default_from_fn() {
        let mut data_src = MockDataSource::new();

        let config = ConfigDefaultFn::build(&mut data_src, None).unwrap();
        assert_eq!(config.field3, 4);
    }

    #[test]
    fn it_writes_to_data_src() {
        let mut data_src = MockDataSource::new();

        let _config = ConfigDefaultFn::build(&mut data_src, None).unwrap();
        let val = data_src.get("FIELD3").unwrap().unwrap();
        assert_eq!(val, "4")
    }

    #[derive(AppConfig)]
    pub struct ConfigSkip {
        #[appconfig(skip)]
        field: String,
    }

    #[test]
    fn it_skips_fields() {
        let mut data_src = MockDataSource::new();
        data_src.set("FIELD", "hello".to_string()).unwrap();

        let config = ConfigSkip::build(&mut data_src, None, "world".to_string()).unwrap();
        assert_eq!(config.field, "world");
    }

    #[derive(AppConfig)]
    pub struct ConfigNested {
        field4: i64,
    }

    #[test]
    fn it_should_error_if_no_values_are_provided() {
        let mut data_src = MockDataSource::new();

        let config = ConfigNested::build(&mut data_src, None);
        assert!(config.is_err());
    }

    #[derive(AppConfig)]
    pub struct ConfigNested2 {
        field5: i64,
        #[appconfig(nested)]
        field6: ConfigNested,
    }

    #[test]
    fn it_reads_nested() {
        let mut data_src = MockDataSource::new();
        data_src.set("FIELD5", "5".to_string()).unwrap();
        data_src.set("FIELD6_FIELD4", "6".to_string()).unwrap();

        let config = ConfigNested2::build(&mut data_src, None).unwrap();
        assert_eq!(config.field5, 5);
        assert_eq!(config.field6.field4, 6);
    }

    #[derive(AppConfig)]
    pub struct ConfigNested3 {
        #[appconfig(nested, prefix = "nested_")]
        field7: ConfigNested,
    }

    #[test]
    fn it_reads_nested_with_prefix() {
        let mut data_src = MockDataSource::new();
        data_src.set("NESTED_FIELD4", "7".to_string()).unwrap();

        let config = ConfigNested3::build(&mut data_src, None).unwrap();
        assert_eq!(config.field7.field4, 7);
    }

    #[derive(AppConfig)]
    pub struct ConfigNested4 {
        #[appconfig(nested, data_src = nested_data)]
        field8: ConfigNested,
    }

    #[test]
    fn it_reads_nested_with_data_src() {
        let mut data_src = MockDataSource::new();
        let mut nested_data = MockDataSource::new();
        std::env::set_var("FIELD8_FIELD4", "8");

        let config = ConfigNested4::build(&mut data_src, None, &mut nested_data).unwrap();
        assert_eq!(config.field8.field4, 8);
        assert_eq!(nested_data.get("FIELD8_FIELD4").unwrap().unwrap(), "8");
        std::env::remove_var("FIELD8_FIELD4");
    }
}
