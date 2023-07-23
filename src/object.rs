//! Object module
//!
//! This module contains the code to process and store object information

use std::{collections::HashMap, io::Write};

use xml::writer::XmlEvent;

use crate::xmlwriter::{ToXmlWriter, XmlWriter};

/// Structure containing object information
pub struct Object {
    /// Object name
    name: String,
    /// Hash map that contains segment name as key and sum of all asociated entry sizes as value
    segment_size: HashMap<String, u64>,
}

impl Object {
    /// Creates a new [Object]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            segment_size: HashMap::new(),
        }
    }

    /// Get object name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the sum of all segments size
    pub fn get_total_size(&self) -> u64 {
        self.segment_size.values().sum()
    }

    /// Update segment size
    pub fn update_segment_size(&mut self, name: &str, size: u64) {
        *self.segment_size.entry(name.to_string()).or_insert(0) += size;
    }

    /// Get size of segment or [None](Option::None) if missing
    pub fn get_segment_size(&self, name: &str) -> Option<u64> {
        self.segment_size.get(name).copied()
    }

    /// Returns a list of all stored segments name
    pub fn get_all_segments(&self) -> Vec<&str> {
        self.segment_size
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    }
}

impl<W: Write> ToXmlWriter<W> for Object {
    fn to_xml_writer(&self, writer: &mut XmlWriter<W>) {
        writer.start_element(XmlEvent::start_element("object").attr("name", &self.name));

        let count = self.segment_size.len();
        if count != 0 {
            writer.start_element(
                XmlEvent::start_element("segments").attr("count", &count.to_string()),
            );

            for (name, size) in &self.segment_size {
                writer.start_element(
                    XmlEvent::start_element("segment")
                        .attr("name", name)
                        .attr("size", &size.to_string()),
                );
                writer.end_element(); // XmlEvent::start_element("segment")
            }

            writer.end_element(); // XmlEvent::start_element("segments")
        }

        writer.end_element(); // XmlEvent::start_element("object")
    }
}
