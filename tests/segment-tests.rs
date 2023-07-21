use parser::{
    entry::Entry,
    segment::Segment,
    xmlwriter::{ToXmlWriter, XmlWriter},
};

mod uthelper;
use uthelper::*;
use xml::ParserConfig;

#[test]
fn new_no_size_and_address() {
    let name = get_random_string(RAND_STRING_SIZE);

    let segment = Segment::new(&name);

    assert_eq!(segment.get_name(), name);
    assert_eq!(segment.get_address(), None);
    assert_eq!(segment.get_size(), None);
    assert!(segment.get_entries().is_empty());
    assert_eq!(segment.get_entries_total_size(), 0);
}

#[test]
fn new_with_size_and_address() {
    let name = get_random_string(RAND_STRING_SIZE);
    let address = 1;
    let size = 1;

    let mut segment = Segment::new(&name);
    segment.set_size_and_address(size, address);

    assert_eq!(segment.get_name(), name);
    assert_eq!(segment.get_address(), Some(address));
    assert_eq!(segment.get_size(), Some(size));
    assert!(segment.get_entries().is_empty());
    assert_eq!(segment.get_entries_total_size(), 0);
}

#[test]
fn entries_test() {
    let name = get_random_string(RAND_STRING_SIZE);

    let mut segment = Segment::new(&name);

    assert!(segment.get_entries().is_empty());

    let mut test_entries: Vec<Entry> = vec![];
    for _ in 0..ENTRIES_COUNT {
        let name = get_random_string(RAND_STRING_SIZE);
        let data = get_random_string(RAND_STRING_SIZE);
        let address = get_random_number(RAND_NUMBER_MAX);
        let size = get_random_number(RAND_NUMBER_MAX);

        let entry = Entry::new(&name, address, size, &data);

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
    let name = get_random_string(RAND_STRING_SIZE);

    let segment = Segment::new(&name);

    test_xml_output(&segment, false);
}

#[test]
fn xml_writer_with_size_and_address() {
    let name = get_random_string(RAND_STRING_SIZE);
    let size = get_random_number(RAND_NUMBER_MAX);
    let address: u64 = get_random_number(RAND_NUMBER_MAX);

    let mut segment = Segment::new(&name);
    segment.set_size_and_address(size, address);

    test_xml_output(&segment, false);
}
