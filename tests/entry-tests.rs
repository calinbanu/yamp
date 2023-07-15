use parser::{entry::Entry, xmlwriter::ToXmlWriter, xmlwriter::XmlWriter};
use xml::ParserConfig;

mod uthelper;
use uthelper::*;

#[test]
fn new() {
    let name = "test";
    let data = "data";
    let address = 0;
    let size = 0;

    let entry = Entry::new(name, address, size, data);

    assert_eq!(entry.get_name(), name);
    assert_eq!(entry.get_address(), address);
    assert_eq!(entry.get_size(), size);
    assert_eq!(entry.get_data(), data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);
}

#[test]
fn new_size_non_overlaping() {
    let name = "test";
    let data = "data";
    let address = 0;
    let size = 1;
    let fill_address = 1;
    let fill_size = 2;

    assert_ne!(address, fill_address);

    let mut entry = Entry::new(name, address, size, data);
    entry.set_fill(fill_address, fill_size);

    assert_eq!(entry.get_name(), name);
    assert_eq!(entry.get_address(), address);
    assert_eq!(entry.get_size(), size + fill_size);
    assert_eq!(entry.get_data(), data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);
}

#[test]
fn new_size_overlaping() {
    let name = "test";
    let data = "data";
    let address = 0;
    let size = 1;
    let fill_address = 0;
    let fill_size = 2;

    assert_eq!(address, fill_address);

    let mut entry = Entry::new(name, address, size, data);
    entry.set_fill(fill_address, fill_size);

    assert_eq!(entry.get_name(), name);
    assert_eq!(entry.get_address(), address);
    assert_eq!(entry.get_size(), fill_size);
    assert_eq!(entry.get_data(), data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);
}

#[test]
fn new_object_name() {
    let name = "test";
    let data = "data";
    let object_name = "object";
    let address = 0;
    let size = 1;

    let mut entry = Entry::new(name, address, size, data);

    assert_eq!(entry.get_name(), name);
    assert_eq!(entry.get_address(), address);
    assert_eq!(entry.get_size(), size);
    assert_eq!(entry.get_data(), data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);

    entry.set_object_name(object_name);

    assert_eq!(entry.get_object_name(), Some(object_name));
    assert_eq!(entry.get_library_name(), None);
}

#[test]
fn new_library_name() {
    let name = "test";
    let data = "data";
    let library_name: &str = "object";
    let address = 0;
    let size = 1;

    let mut entry = Entry::new(name, address, size, data);

    assert_eq!(entry.get_name(), name);
    assert_eq!(entry.get_address(), address);
    assert_eq!(entry.get_size(), size);
    assert_eq!(entry.get_data(), data);

    assert_eq!(entry.get_library_name(), None);
    assert_eq!(entry.get_object_name(), None);

    entry.set_library_name(library_name);

    assert_eq!(entry.get_library_name(), Some(library_name));
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
    let entry_name = "test";
    let entry_data = "data";
    let entry_address = 0;
    let entry_size = 0;

    let entry = Entry::new(entry_name, entry_address, entry_size, entry_data);

    test_xml_output(&entry, false);
}

#[test]
fn xml_writer_size_non_overlaping() {
    let name = "test";
    let data = "data";
    let address = 0;
    let size = 1;
    let fill_address = 1;
    let fill_size = 2;

    assert_ne!(address, fill_address);

    let mut entry = Entry::new(name, address, size, data);
    entry.set_fill(fill_address, fill_size);

    test_xml_output(&entry, false);
}

#[test]
fn xml_writer_size_overlaping() {
    let name = "test";
    let data = "data";
    let address = 0;
    let size = 1;
    let fill_address = 0;
    let fill_size = 2;

    assert_eq!(address, fill_address);

    let mut entry = Entry::new(name, address, size, data);
    entry.set_fill(fill_address, fill_size);

    test_xml_output(&entry, false);
}

#[test]
fn xml_writer_skip_data() {
    let entry_name = "test";
    let entry_data = "data";
    let entry_address = 0;
    let entry_size = 0;

    let entry = Entry::new(entry_name, entry_address, entry_size, entry_data);

    test_xml_output(&entry, true);
}
