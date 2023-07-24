use parser::{
    entry::Entry,
    segment::Segment,
    xmlwriter::{ToXmlWriter, XmlWriter},
};

mod uthelper;
use uthelper::*;
use xml::ParserConfig;

const ENTRIES_COUNT: usize = 10;
const RAND_NAME_STRING_LEN: usize = 20;

#[test]
fn new_no_size_and_address() {
    let segment_name = get_random_string(RAND_NAME_STRING_LEN);

    let segment = Segment::new(&segment_name);

    assert_eq!(segment.get_name(), segment_name);
    assert_eq!(segment.get_address(), None);
    assert_eq!(segment.get_size(), None);
    assert!(segment.get_entries().is_empty());
    assert_eq!(segment.get_entries_total_size(), 0);
}

#[test]
fn new_with_size_and_address() {
    let segment_name = get_random_string(RAND_NAME_STRING_LEN);
    let segment_address = get_random_number(RAND_ADDRESS_MAX);
    let segment_size = get_random_number(RAND_SIZE_MAX);

    let mut segment = Segment::new(&segment_name);
    segment.set_size_and_address(segment_size, segment_address);

    assert_eq!(segment.get_name(), segment_name);
    assert_eq!(segment.get_address(), Some(segment_address));
    assert_eq!(segment.get_size(), Some(segment_size));
    assert!(segment.get_entries().is_empty());
    assert_eq!(segment.get_entries_total_size(), 0);
}

#[test]
fn entries_test() {
    let segment_name = get_random_string(RAND_NAME_STRING_LEN);

    let mut segment = Segment::new(&segment_name);

    assert!(segment.get_entries().is_empty());

    let mut test_entries: Vec<Entry> = vec![];
    for _ in 0..ENTRIES_COUNT {
        let entry_name = get_random_string(RAND_NAME_STRING_LEN);
        let entry_data: String = get_random_string(RAND_NAME_STRING_LEN);
        let entry_address = get_random_number(RAND_ADDRESS_MAX);
        let entry_size = get_random_number(RAND_SIZE_MAX);

        let entry = Entry::new(&entry_name, entry_address, entry_size, &entry_data);

        segment.add_entry(entry.clone());

        test_entries.push(entry);
    }

    let entries = segment.get_entries();

    assert_eq!(entries.len(), ENTRIES_COUNT);
    assert_eq!(entries, test_entries);

    let test_entries_sum = test_entries.iter().fold(0, |acc, e| acc + e.get_size());

    assert_eq!(segment.get_entries_total_size(), test_entries_sum);
}

fn test_xml_output(segment: &Segment, skip_data: bool) {
    let sink = UTSinkSource::new();
    let mut writer = XmlWriter::new_empty(sink.clone());
    if skip_data {
        writer.set_skip_data(true);
    }
    segment.to_xml_writer(&mut writer);

    drop(writer);

    let mut parser = ParserConfig::default()
        .ignore_root_level_whitespace(true)
        .trim_whitespace(true)
        .create_reader(sink);

    check_start_document_event(parser.next().unwrap());

    check_segment_start_element_event(parser.next().unwrap(), segment);

    check_end_element_event(parser.next().unwrap(), "segment");

    check_end_document_event(parser.next().unwrap());
}

#[test]
fn xml_writer_no_size_and_address() {
    let segment_name = get_random_string(RAND_NAME_STRING_LEN);

    let segment = Segment::new(&segment_name);

    test_xml_output(&segment, false);
}

#[test]
fn xml_writer_with_size_and_address() {
    let segment_name = get_random_string(RAND_NAME_STRING_LEN);
    let segment_size = get_random_number(RAND_ADDRESS_MAX);
    let segment_address: u64 = get_random_number(RAND_ADDRESS_MAX);

    let mut segment = Segment::new(&segment_name);
    segment.set_size_and_address(segment_size, segment_address);

    test_xml_output(&segment, false);
}
