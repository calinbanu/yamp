mod uthelper;
use parser::{
    object::Object,
    xmlwriter::{ToXmlWriter, XmlWriter},
};
use uthelper::*;
use xml::ParserConfig;

#[test]
fn new_empty() {
    let name = get_random_string(RAND_STRING_SIZE);
    let object = Object::new(&name);

    assert_eq!(object.get_name(), &name);
    assert_eq!(object.get_total_size(), 0);
}

#[test]
fn new_distinct_sections() {
    let object_name = get_random_string(RAND_STRING_SIZE);
    let mut object = Object::new(&object_name);

    let mut sum = 0;
    for _ in 0..10 {
        let segment_name = get_random_string(RAND_STRING_SIZE);
        let segment_size = get_random_number(RAND_NUMBER_MAX);
        sum += segment_size;

        object.update_segment_size(&segment_name, segment_size)
    }

    assert_eq!(object.get_total_size(), sum);
}

#[test]
fn new_same_section() {
    let object_name = get_random_string(RAND_STRING_SIZE);
    let mut object = Object::new(&object_name);
    let segment_name = get_random_string(RAND_STRING_SIZE);

    let mut sum = 0;
    for _ in 0..10 {
        let segment_size = get_random_number(RAND_NUMBER_MAX);
        sum += segment_size;

        object.update_segment_size(&segment_name, segment_size)
    }

    assert_eq!(object.get_total_size(), sum);
}

#[test]
fn xml_writer_empty() {
    let name = get_random_string(RAND_STRING_SIZE);
    let object = Object::new(&name);

    let sink = UTSinkSource::new();
    let mut writer = XmlWriter::new_empty(sink.clone());

    object.to_xml_writer(&mut writer);

    drop(writer);

    let mut parser = ParserConfig::default()
        .ignore_root_level_whitespace(true)
        .trim_whitespace(true)
        .create_reader(sink);

    check_start_document_event(parser.next().unwrap());

    check_object_start_element_event(parser.next().unwrap(), &object);

    check_end_element_event(parser.next().unwrap(), "object");

    check_end_document_event(parser.next().unwrap());
}

#[test]
fn xml_writer_distinct_sections() {
    let name = get_random_string(RAND_STRING_SIZE);
    let mut object = Object::new(&name);

    let sink = UTSinkSource::new();
    let mut writer = XmlWriter::new_empty(sink.clone());

    for _ in 0..10 {
        let segment_name = get_random_string(RAND_STRING_SIZE);
        let segment_size = get_random_number(RAND_NUMBER_MAX);

        object.update_segment_size(&segment_name, segment_size);
    }

    object.to_xml_writer(&mut writer);

    drop(writer);

    let mut parser = ParserConfig::default()
        .ignore_root_level_whitespace(true)
        .trim_whitespace(true)
        .create_reader(sink);

    check_start_document_event(parser.next().unwrap());

    check_object_start_element_event(parser.next().unwrap(), &object);

    let count = check_count_start_element_event(parser.next().unwrap(), "segments");

    for _ in 0..count {
        check_object_segment_start_element_event(parser.next().unwrap(), &object);
        check_end_element_event(parser.next().unwrap(), "segment");
    }

    check_end_element_event(parser.next().unwrap(), "segments");

    check_end_element_event(parser.next().unwrap(), "object");

    check_end_document_event(parser.next().unwrap());
}

#[test]
fn xml_writer_same_section() {
    let name = get_random_string(RAND_STRING_SIZE);
    let mut object = Object::new(&name);

    let sink = UTSinkSource::new();
    let mut writer = XmlWriter::new_empty(sink.clone());
    let segment_name = get_random_string(RAND_STRING_SIZE);

    for _ in 0..10 {
        let segment_size = get_random_number(RAND_NUMBER_MAX);

        object.update_segment_size(&segment_name, segment_size);
    }

    object.to_xml_writer(&mut writer);

    drop(writer);

    let mut parser = ParserConfig::default()
        .ignore_root_level_whitespace(true)
        .trim_whitespace(true)
        .create_reader(sink);

    check_start_document_event(parser.next().unwrap());

    check_object_start_element_event(parser.next().unwrap(), &object);

    let count = check_count_start_element_event(parser.next().unwrap(), "segments");

    for _ in 0..count {
        check_object_segment_start_element_event(parser.next().unwrap(), &object);
        check_end_element_event(parser.next().unwrap(), "segment");
    }

    check_end_element_event(parser.next().unwrap(), "segments");

    check_end_element_event(parser.next().unwrap(), "object");

    check_end_document_event(parser.next().unwrap());
}
