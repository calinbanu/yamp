//! Segment module
//!
//! This module contains the code to process and store segment information

use crate::{
    excelwriter::{ExcelWriter, ToExcelWriter},
    xmlwriter::XmlWriter,
    Entry, ToXmlWriter,
};
use std::io::Write;
use xml::writer::XmlEvent;

/// Structure containing memory map segment information
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Segment {
    /// Name
    name: String,
    /// Address or [None](Option::None) if missing (valid along with [size](#structfield.size))
    address: Option<u64>,
    /// Size or [None](Option::None) if missing (valid along with [address](#structfield.address))
    size: Option<u64>,
    /// List of entries
    entries: Vec<Entry>,
}

impl Segment {
    /// Creates a new [Segment]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            address: None,
            size: None,
            entries: vec![],
        }
    }

    /// Sets segment [size](#structfield.size) and [address](#structfield.address)
    pub fn set_size_and_address(&mut self, size: u64, address: u64) {
        self.address = Some(address);
        self.size = Some(size);
    }

    /// Calculates the sum of all entries
    pub fn get_entries_total_size(&self) -> u64 {
        if self.entries.is_empty() {
            return 0;
        }

        // We can have multiple consecutive entries that have the same address
        let mut last_address: u64 = self.entries[0].get_address();
        let mut last_size: u64 = self.entries[0].get_size();
        let mut sum = 0;

        for entry in self.entries[1..].iter() {
            // If it has the same address
            if last_address == entry.get_address() {
                // Keep current size and address
                last_size = entry.get_size();
            } else {
                // If not, add to sum the last size
                sum += last_size;
                last_size = entry.get_size();
                last_address = entry.get_address();
            }
        }
        sum += last_size;
        sum
    }

    /// Adds new entry
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Gets a slice with stored entries
    pub fn get_entries(&self) -> &[Entry] {
        self.entries.as_slice()
    }

    /// Gets segment [size](#structfield.size) or [None](Option::None)
    pub fn get_size(&self) -> Option<u64> {
        self.size
    }

    /// Gets segment [name](#structfield.name)
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Gets segment [address](#structfield.address) or [None](Option::None)
    pub fn get_address(&self) -> Option<u64> {
        self.address
    }
}

impl<W: Write> ToXmlWriter<W> for Segment {
    fn to_xml_writer(&self, writer: &mut XmlWriter<W>) {
        let mut element = XmlEvent::start_element("segment").attr("name", self.name.as_str());

        if let Some(addr) = self.address {
            let addr = format!("{:#016x}", addr);
            let size = self.size.unwrap().to_string();
            element = element.attr("address", &addr).attr("size", &size);
            writer.start_element(element);
        } else {
            writer.start_element(element);
        }

        self.entries.iter().for_each(|s| s.to_xml_writer(writer));

        writer.end_element();
    }
}

impl ToExcelWriter for Segment {
    fn to_excel_writer<'a, 'b>(&'a self, writer: &mut ExcelWriter<'b>)
    where
        'a: 'b,
    {
        writer.write_segment(self);

        for entry in &self.entries {
            entry.to_excel_writer(writer);
        }
    }
}
