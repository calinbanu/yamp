use std::io::Write;
use xml::{writer::XmlEvent, EmitterConfig, EventWriter};

pub trait ToXmlWriter<W>
where
    W: Write,
{
    fn to_xml_writer(&self, writer: &mut XmlWriter<W>);
}

pub struct XmlWriter<W>
where
    W: Write,
{
    writer: EventWriter<W>,
    skip_data: bool,
    empty: bool,
}

impl<W> XmlWriter<W>
where
    W: Write,
{
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

    pub fn set_skip_data(&mut self, value: bool) {
        self.skip_data = value;
    }

    pub fn get_skip_data(&self) -> bool {
        self.skip_data
    }

    pub fn start_element<'a, E>(&mut self, event: E)
    where
        E: Into<XmlEvent<'a>>,
    {
        self.writer.write(event).unwrap();
    }

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
