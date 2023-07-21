use std::{
    cell::RefCell,
    io::{Read, Write},
    ops::Deref,
    rc::Rc,
};

use parser::{entry::Entry, segment::Segment};
use rand::{distributions::Alphanumeric, Rng};
use xml::{common::XmlVersion, reader::XmlEvent};

#[allow(dead_code)]
pub const ENTRIES_COUNT: usize = 10;
#[allow(dead_code)]
pub const RAND_STRING_SIZE: usize = 10;
#[allow(dead_code)]
pub const RAND_NUMBER_MAX: u64 = u32::MAX as u64;

pub struct UTSinkSource {
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl UTSinkSource {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            buffer: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl ToString for UTSinkSource {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(self.buffer.deref().borrow().as_slice()).to_string()
    }
}

impl Clone for UTSinkSource {
    fn clone(&self) -> Self {
        Self {
            buffer: Rc::clone(&self.buffer),
        }
    }
}

impl Write for UTSinkSource {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.deref().borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for UTSinkSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.deref().borrow_mut();
        let limit = if buf.len() > buffer.len() {
            buffer.len()
        } else {
            buf.len()
        };
        buffer
            .drain(0..limit)
            .enumerate()
            .for_each(|(i, e)| buf[i] = e);
        Ok(limit)
    }
}

#[allow(dead_code)]
pub fn check_start_document_event(event: XmlEvent) {
    if let XmlEvent::StartDocument {
        version,
        encoding,
        standalone,
    } = event
    {
        assert_eq!(version, XmlVersion::Version10);
        assert_eq!(encoding, "UTF-8");
        assert_eq!(standalone, Option::None);
    } else {
        panic!("Expected XmlEvent::StartDocument!")
    }
}

#[allow(dead_code)]
pub fn check_mapfile_start_element_event(event: XmlEvent, mapfile_source: &str) {
    if let XmlEvent::StartElement {
        name,
        attributes,
        namespace: _,
    } = event
    {
        assert_eq!(name.local_name, "mapfile");
        assert_eq!(name.namespace, Option::None);
        assert_eq!(name.prefix, Option::None);

        let attr = &attributes[0];
        assert_eq!(attr.name.local_name, "datetime");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        // TODO(calin) check datetime value
        // assert_eq!(attr.value, entry_name);

        let attr = &attributes[1];
        assert_eq!(attr.name.local_name, "source");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        assert_eq!(attr.value, mapfile_source);

        // TODO(calin) check namespace ?
    } else {
        panic!("Expected XmlEvent::StartElement!")
    }
}

#[allow(dead_code)]
pub fn check_entry_start_element_event(event: XmlEvent, entry: &Entry) {
    if let XmlEvent::StartElement {
        name,
        attributes,
        namespace: _,
    } = event
    {
        assert_eq!(name.local_name, "entry");
        assert_eq!(name.namespace, Option::None);
        assert_eq!(name.prefix, Option::None);

        let attr = &attributes[0];
        assert_eq!(attr.name.local_name, "name");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        assert_eq!(attr.value, entry.get_name());

        let attr = &attributes[1];
        assert_eq!(attr.name.local_name, "address");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        let address = format!("{:#016x}", entry.get_address());
        assert_eq!(attr.value, address);

        let attr = &attributes[2];
        assert_eq!(attr.name.local_name, "size");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        let size = format!("{}", entry.get_original_size());
        assert_eq!(attr.value, size);

        let attr = &attributes[3];
        assert_eq!(attr.name.local_name, "fill_size");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        let fill_size = format!("{}", entry.get_fill_size());
        assert_eq!(attr.value, fill_size);

        let attr = &attributes[4];
        assert_eq!(attr.name.local_name, "fill_overlaps");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        let fill_overlaps = format!("{}", entry.get_fill_overlaps());
        assert_eq!(attr.value, fill_overlaps);

        // TODO(calin) check namespace ?
    } else {
        panic!("Expected XmlEvent::StartElement!")
    }
}

#[allow(dead_code)]
pub fn check_start_element_event(event: XmlEvent, event_name: &str) {
    if let XmlEvent::StartElement {
        name,
        attributes,
        namespace: _,
    } = event
    {
        assert_eq!(name.local_name, event_name);
        assert_eq!(name.namespace, Option::None);
        assert_eq!(name.prefix, Option::None);
        assert_eq!(attributes.len(), 0);

        // TODO(calin) check namespace ?
    } else {
        panic!("Expected XmlEvent::StartElement!")
    }
}

#[allow(dead_code)]
pub fn check_end_element_event(event: XmlEvent, event_name: &str) {
    if let XmlEvent::EndElement { name } = event {
        assert_eq!(name.local_name, event_name);
        assert_eq!(name.namespace, Option::None);
        assert_eq!(name.prefix, Option::None);
    } else {
        panic!("Expected XmlEvent::EndElement!")
    }
}

#[allow(dead_code)]
pub fn check_characters_event(event: XmlEvent, characters: &str) {
    if let XmlEvent::Characters { 0: data } = event {
        assert_eq!(data, characters);
    } else {
        panic!("Expected XmlEvent::Characters!")
    }
}

#[allow(dead_code)]
pub fn check_end_document_event(event: XmlEvent) {
    if XmlEvent::EndDocument != event {
        panic!("Expected XmlEvent::Characters!")
    }
}

#[allow(dead_code)]
pub fn check_segment_start_element_event(event: XmlEvent, segment: &Segment) {
    if let XmlEvent::StartElement {
        name,
        attributes,
        namespace: _,
    } = event
    {
        assert_eq!(name.local_name, "segment");
        assert_eq!(name.namespace, Option::None);
        assert_eq!(name.prefix, Option::None);
        assert!(matches!(attributes.len(), 1 | 3));

        let attr = &attributes[0];
        assert_eq!(attr.name.local_name, "name");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        assert_eq!(attr.value, segment.get_name());

        if attributes.len() == 3 {
            let attr = &attributes[1];
            assert_eq!(attr.name.local_name, "address");
            assert_eq!(attr.name.namespace, Option::None);
            assert_eq!(attr.name.prefix, Option::None);
            let address = format!("{:#016x}", segment.get_address().unwrap());
            assert_eq!(attr.value, address);

            let attr = &attributes[2];
            assert_eq!(attr.name.local_name, "size");
            assert_eq!(attr.name.namespace, Option::None);
            assert_eq!(attr.name.prefix, Option::None);
            let size = format!("{}", segment.get_size().unwrap());
            assert_eq!(attr.value, size);
        }
        // TODO(calin) check namespace ?
    } else {
        panic!("Expected XmlEvent::StartElement!")
    }
}

#[allow(dead_code)]
pub fn get_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[allow(dead_code)]
pub fn get_random_number(max: u64) -> u64 {
    rand::thread_rng().gen_range(0..max)
}
