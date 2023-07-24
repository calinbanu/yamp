use parser::{entry::Entry, xmlwriter::ToXmlWriter, xmlwriter::XmlWriter};
use xml::ParserConfig;

mod uthelper;
use uthelper::*;

const RAND_NAME_STRING_LEN: usize = 20;
const RAND_DATA_STRING_LEN: usize = 100;

#[test]
fn new() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);

    let entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);

    assert_eq!(entry.get_name(), entry_name);
    assert_eq!(entry.get_address(), entry_address);
    assert_eq!(entry.get_size(), entry_size);
    assert_eq!(entry.get_data(), entry_data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);
}

#[test]
fn new_size_non_overlaping() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);
    let fill_address = entry_address + 1 + get_random_number(100);
    let fill_size = get_random_number(RAND_SIZE_MAX);

    assert_ne!(entry_address, fill_address);

    let mut entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);
    entry.set_fill(fill_address, fill_size);

    assert_eq!(entry.get_name(), entry_name);
    assert_eq!(entry.get_address(), entry_address);
    assert_eq!(entry.get_size(), entry_size + fill_size);
    assert_eq!(entry.get_data(), entry_data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);
}

#[test]
fn new_size_overlaping() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);
    let fill_address = entry_address;
    let fill_size = get_random_number(RAND_SIZE_MAX);

    assert_eq!(entry_address, fill_address);

    let mut entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);
    entry.set_fill(fill_address, fill_size);

    assert_eq!(entry.get_name(), entry_name);
    assert_eq!(entry.get_address(), entry_address);
    assert_eq!(entry.get_size(), fill_size);
    assert_eq!(entry.get_data(), entry_data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);
}

#[test]
fn new_object_name() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let object_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);

    let mut entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);

    assert_eq!(entry.get_name(), entry_name);
    assert_eq!(entry.get_address(), entry_address);
    assert_eq!(entry.get_size(), entry_size);
    assert_eq!(entry.get_data(), entry_data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);

    entry.set_object_name(&object_name);

    assert_eq!(entry.get_object_name(), Some(object_name.as_str()));
    assert_eq!(entry.get_library_name(), None);
}

#[test]
fn new_library_name() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let library_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);

    let mut entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);

    assert_eq!(entry.get_name(), entry_name);
    assert_eq!(entry.get_address(), entry_address);
    assert_eq!(entry.get_size(), entry_size);
    assert_eq!(entry.get_data(), entry_data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);

    entry.set_library_name(&library_name);

    assert_eq!(entry.get_library_name(), Some(library_name.as_str()));
    assert_eq!(entry.get_object_name(), None);
}

fn test_xml_output(entry: &Entry, skip_data: bool) {
    let sink = UTSinkSource::new();
    let mut writer = XmlWriter::new_empty(sink.clone());
    if skip_data {
        writer.set_skip_data(true);
    }
    entry.to_xml_writer(&mut writer);

    drop(writer);

    let mut parser = ParserConfig::default()
        .ignore_root_level_whitespace(true)
        .trim_whitespace(true)
        .create_reader(sink);

    check_start_document_event(parser.next().unwrap());

    check_entry_start_element_event(parser.next().unwrap(), entry);

    if !skip_data {
        check_start_element_event(parser.next().unwrap(), "data");

        check_characters_event(parser.next().unwrap(), entry.get_data());

        check_end_element_event(parser.next().unwrap(), "data");
    }

    check_end_element_event(parser.next().unwrap(), "entry");

    check_end_document_event(parser.next().unwrap());
}

#[test]
fn xml_writer() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);

    let entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);

    test_xml_output(&entry, false);
}

#[test]
fn xml_writer_size_non_overlaping() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);
    let fill_address = entry_address + 1 + get_random_number(100);
    let fill_size = get_random_number(RAND_SIZE_MAX);

    assert_ne!(entry_address, fill_address);

    let mut entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);
    entry.set_fill(fill_address, fill_size);

    test_xml_output(&entry, false);
}

#[test]
fn xml_writer_size_overlaping() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);
    let fill_address = entry_address;
    let fill_size = get_random_number(RAND_SIZE_MAX);

    assert_eq!(entry_address, fill_address);

    let mut entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);
    entry.set_fill(fill_address, fill_size);

    test_xml_output(&entry, false);
}

#[test]
fn xml_writer_skip_data() {
    let entry_name = get_random_string(RAND_NAME_STRING_LEN);
    let entry_data = get_random_string(RAND_DATA_STRING_LEN);
    let entry_address = get_random_number(RAND_ADDRESS_MAX);
    let entry_size = get_random_number(RAND_SIZE_MAX);

    let entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);

    test_xml_output(&entry, true);
}
