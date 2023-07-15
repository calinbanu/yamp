use std::{collections::HashMap, io::Write};

use xml::writer::XmlEvent;

use crate::xmlwriter::{ToXmlWriter, XmlWriter};

#[derive(Debug)]
pub struct Object {
    name: String,
    pub(crate) segment_size: HashMap<String, u64>,
}

impl Object {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            segment_size: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn update_segment_size(&mut self, name: &str, size: u64) {
        *self.segment_size.entry(name.to_string()).or_insert(0) += size;
    }
}

impl<W: Write> ToXmlWriter<W> for Object {
    fn to_xml_writer(&self, writer: &mut XmlWriter<W>) {
        writer.start_element(XmlEvent::start_element("object").attr("name", &self.name));

        let count = self.segment_size.len().to_string();
        writer.start_element(XmlEvent::start_element("segments").attr("count", &count));

        for (name, size) in &self.segment_size {
            writer.start_element(
                XmlEvent::start_element("segment")
                    .attr("name", name)
                    .attr("size", &size.to_string()),
            );
            writer.end_element(); // XmlEvent::start_element("segment")
        }

        writer.end_element(); // XmlEvent::start_element("segments")

        writer.end_element(); // XmlEvent::start_element("object")
    }
}
