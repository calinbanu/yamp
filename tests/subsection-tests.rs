use parser::subsection::SubSection;
use std::{fs, process::exit, str::FromStr};

struct Fixture {}

impl Fixture {
    fn load_test_file(path: &str) -> String {
        match fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => {
                println!("Problem reading test file: \"{}\" ({})", path, err);
                exit(1);
            }
        }
    }
}

#[test]
fn subsection() {
    let file =
        Fixture::load_test_file("/workspaces/yamp/tests/test-valid-single-line-subsection.txt");
    println!("{}", file);
    let test = SubSection::from_str(&file);
    assert!(true);
}

#[test]
fn new() {
    let name = "test".to_string();
    let address = 0;
    let size = 0;

    let section = SubSection::new(name.clone(), address, size);

    assert_eq!(section.get_name(), name);
    assert_eq!(section.get_address(), address);
    assert_eq!(section.get_size(), size);
}

#[test]
fn new_size_non_overlaping() {
    let address = 0;
    let size = 1;
    let fill_address = 1;
    let fill_size = 2;

    let mut section = SubSection::new("test".to_string(), address, size);
    section.set_fill(fill_address, fill_size);

    assert_eq!(section.get_size(), size + fill_size)
}

#[test]
fn new_size_overlaping() {
    let address = 0;
    let size = 1;
    let fill_address = 0;
    let fill_size = 2;

    let mut section = SubSection::new("test".to_string(), address, size);
    section.set_fill(fill_address, fill_size);

    assert_eq!(section.get_size(), fill_size)
}
