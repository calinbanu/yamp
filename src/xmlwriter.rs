//! XML Writer module
//!
//! This module contains the code for XML Writer

use std::io::Write;
use xml::{writer::XmlEvent, EmitterConfig, EventWriter};

/// This trait must be implemented in order to convert into an xml format for XmlWriter
pub trait ToXmlWriter<W>
where
    W: Write,
{
    fn to_xml_writer(&self, writer: &mut XmlWriter<W>);
}

/// XML Writer structure
pub struct XmlWriter<W>
where
    W: Write,
{
    /// Event Writer
    writer: EventWriter<W>,
    /// If [true], do not include data from [Entry](crate::entry::Entry)
    skip_data: bool,
    /// If [true], the mapfile elements does not get written. Valid only for [new_empty](#method.new_empty), in UT.
    empty: bool,
}

impl<W> XmlWriter<W>
where
    W: Write,
{
    /// Creates a new [XmlWriter]
    pub fn new(sink: W, source: &str) -> Self {
        let mut writer = Self {
            writer: EmitterConfig::new()
                .perform_indent(true)
                .indent_string("    ")
                .write_document_declaration(false)
                .create_writer(sink),
            skip_data: false,
            empty: false,
        };
        let datetime: chrono::DateTime<chrono::offset::Utc> = std::time::SystemTime::now().into();
        writer.start_element(
            XmlEvent::start_element("mapfile")
                .attr(
                    "datetime",
                    datetime.format("%d/%m/%Y %T").to_string().as_str(),
                )
                .attr("source", source),
        );
        writer
    }

    /// Creates a new [XmlWriter] but without `<mapfile...>` element
    pub fn new_empty(sink: W) -> Self {
        Self {
            writer: EmitterConfig::new()
                .perform_indent(true)
                .indent_string("    ")
                .write_document_declaration(false)
                .create_writer(sink),
            skip_data: false,
            empty: true,
        }
    }

    /// Set skip data
    pub fn set_skip_data(&mut self, value: bool) {
        self.skip_data = value;
    }

    /// Get skip data state
    pub fn get_skip_data(&self) -> bool {
        self.skip_data
    }

    /// Start a new element with given `event`. Make sure it has an equivalent [end_element](#method.end_element)
    pub fn start_element<'a, E>(&mut self, event: E)
    where
        E: Into<XmlEvent<'a>>,
    {
        self.writer.write(event).unwrap();
    }

    /// End element. Make sure it has an equivalent [start_element](#method.start_element)
    pub fn end_element(&mut self) {
        self.writer.write(XmlEvent::end_element()).unwrap();
    }
}

impl<W> Drop for XmlWriter<W>
where
    W: Write,
{
    fn drop(&mut self) {
        if !self.empty {
            self.end_element();
        }
    }
}
