//! Entry module
//!
//! This module contains the code to process and store entry information

use crate::{
    excelwriter::ToExcelWriter,
    xmlwriter::{ToXmlWriter, XmlWriter},
};
use std::io::Write;
use xml::writer::XmlEvent;

/// Structure containing memory map entry information
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    /// Entry `name`
    name: String,
    /// Start `address`
    address: u64,
    /// Entry `size`
    size: u64,
    /// Fill `size`
    fill_size: u64,
    /// If the address of the fill is the same as the entry address
    fill_overlaps: bool,
    /// Data from where information were extracted
    data: String,
    /// Object `name` or None
    object_name: Option<String>,
    /// Library `name` or None
    library_name: Option<String>,
}

impl Entry {
    /// Creates a new `Entry`
    pub fn new(name: &str, address: u64, size: u64, data: &str) -> Self {
        Self {
            name: name.to_string(),
            address,
            size,
            fill_size: 0,
            fill_overlaps: false,
            data: data.to_string(),
            object_name: None,
            library_name: None,
        }
    }

    /// Set object `name`
    pub fn set_object_name(&mut self, name: &str) {
        self.object_name = Some(name.to_string());
    }

    /// Get object name or None
    pub fn get_object_name(&self) -> Option<&str> {
        self.object_name.as_deref()
    }

    /// Set library `name`
    pub fn set_library_name(&mut self, name: &str) {
        self.library_name = Some(name.to_string());
    }

    /// Get library `name` or None
    pub fn get_library_name(&self) -> Option<&str> {
        self.library_name.as_deref()
    }

    /// Set entry fill `address` and `size`
    ///
    /// If fill address is the same as the entry, it will set [`fill_overlaps`] to true
    ///
    /// [`fill_overlaps`]: #structfield.fill_overlaps
    pub fn set_fill(&mut self, address: u64, size: u64) {
        self.fill_size = size;
        if self.address == address {
            self.fill_overlaps = true;
        }
    }

    /// Get entry `name`
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get entry `address`
    pub fn get_address(&self) -> u64 {
        self.address
    }

    /// Get `data` from where the entry was parsed
    pub fn get_data(&self) -> &str {
        &self.data
    }

    /// Get entry fill `size`
    pub fn get_fill_size(&self) -> u64 {
        self.fill_size
    }

    /// Get entry fill `overlap`
    pub fn get_fill_overlaps(&self) -> bool {
        self.fill_overlaps
    }

    /// Get entry `size`
    ///
    /// If the [`fill_overlaps`](#structfield.fill_overlaps) is true, then the size will be [`fill_size`](#structfield.fill_size)
    /// else it will be the sum of [`fill_size`](#structfield.fill_size) and [`size`](#structfield.size)
    pub fn get_size(&self) -> u64 {
        match self.fill_overlaps {
            true => self.fill_size,
            false => self.size + self.fill_size,
        }
    }

    /// Get entry original `size`
    ///
    /// This is the original size from the mapfile without accounting for fill
    pub fn get_original_size(&self) -> u64 {
        self.size
    }
}

impl<W: Write> ToXmlWriter<W> for Entry {
    fn to_xml_writer(&self, writer: &mut XmlWriter<W>) {
        let addr = format!("{:#016x}", self.address);
        let size = self.size.to_string();
        let fill_size = self.fill_size.to_string();
        let fill_overlaps = self.fill_overlaps.to_string();

        let entry_element = XmlEvent::start_element("entry")
            .attr("name", self.name.as_str())
            .attr("address", &addr)
            .attr("size", &size)
            .attr("fill_size", &fill_size)
            .attr("fill_overlaps", &fill_overlaps);

        writer.start_element(entry_element);

        if !writer.get_skip_data() {
            writer.start_element(XmlEvent::start_element("data"));
            writer.start_element(XmlEvent::Characters(&self.data));
            writer.end_element();
        }

        writer.end_element();
    }
}

impl ToExcelWriter for Entry {
    fn to_excel_writer<'a, 'b>(&'a self, writer: &mut crate::excelwriter::ExcelWriter<'b>)
    where
        'a: 'b,
    {
        writer.write_entry(self);
    }
}
