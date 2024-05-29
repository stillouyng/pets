
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn read_a_non_exist_config_name() {
        let config_read = read_config("DEFAULT", "none");
        assert_eq!(config_read, "Non exist config name");
    }

    #[test]
    fn read_an_exist_config_name() {
        let config_read = read_config("SYSTEM", "test_config_name");
        assert_eq!(config_read, "true")
    }

    #[test]
    #[ignore]
    fn check_date_in_get_date_function() {
        let date_info_result = get_date();
        let datetime = Local::now();
        let date = datetime.date_naive().to_string();
        assert_eq!(date_info_result.date, date);
    }

    #[test]
    #[ignore]
    fn create_dir_if_dir_exist() {
        let create_dir_result = create_dir_if_not_exist("config");
        match create_dir_result {
            Ok(bool_value) => {
                assert!(bool_value)
            },
            Err(error) => {
                panic!("Error: {:?}", error)
            }
        }
    }

    #[test]
    #[ignore]
    fn create_and_delete_dir() {
        let dir_result = create_dir_if_not_exist("test_dir");
        match dir_result {
            Ok(bool_value) => {
                let del_res = delete_dir("test_dir");
                assert!(del_res);
            }
            Err(error) => {
                panic!("Error in test: {:?}", error);
            }
        }
    }

    #[test]
    fn count_cpu_test() {
        let cpu_test_res = count_cpu("0.1234567".to_owned(), "0.9345678".to_owned());
        assert_eq!(cpu_test_res, "1.0580245%");
    }

    #[test]
    fn count_memory_test() {
        let memory_test_res = count_memory("1234".to_owned(), "9012".to_owned());
        assert_eq!(memory_test_res, "10246 MB");
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn count_files_in_non_existence_dir() {
        let result = count_files_in_temp();
        "There is no directory config/temp.";
    }
}