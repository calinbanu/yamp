mod uthelper;
use parser::xmlwriter::XmlWriter;
use uthelper::*;
use xml::ParserConfig;

#[test]
fn xml_writer_mapfile_event() {
    let mapfile_source = "source";

    let sink = UTSinkSource::new();
    let writer = XmlWriter::new(sink.clone(), mapfile_source);
    drop(writer);

    let mut parser = ParserConfig::default()
        .ignore_root_level_whitespace(true)
        .trim_whitespace(true)
        .create_reader(sink);

    check_start_document_event(parser.next().unwrap());

    check_mapfile_start_element_event(parser.next().unwrap(), mapfile_source);
    check_end_element_event(parser.next().unwrap(), "mapfile");

    check_end_document_event(parser.next().unwrap());
}
