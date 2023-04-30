//! Sub-section module
//!
//! This module contains the code to process and store information
//! regarding sub-sections of the memory map.
use crate::{ParserError, HEX_REGEX, NAME_REGEX};
use log::{error, warn};
use regex::Regex;
use std::str::FromStr;

/// Structure containing memory map sub-section information
#[derive(Debug, PartialEq, Eq)]
pub struct SubSection {
    /// Sub-section name
    name: String,
    /// Start address
    address: u64,
    /// Sub-section Size
    size: u64,
    /// Size of the fill inserted after this section
    fill_size: u64,
    /// If the address of the fill is the same as the sub-section address
    fill_overlaps: bool,
}

impl SubSection {
    /// Creates a new `SubSection`
    ///
    /// # Arguments
    ///
    /// * `name`    - Sub-section name
    /// * `address` - Start address
    /// * `size`    - Size of the sub-section
    pub fn new(name: String, address: u64, size: u64) -> Self {
        Self {
            name,
            address,
            size,
            fill_size: 0,
            fill_overlaps: false,
        }
    }

    /// Get sub-section name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get sub-section address
    pub fn address(&self) -> u64 {
        self.address
    }

    /// Set sub-section fill size
    ///
    /// If fill address is the same as the sub-section, it will set [`fill_overlaps`] to true
    ///
    /// [`fill_overlaps`]: #structfield.fill_overlaps
    ///
    /// # Arguments
    ///
    /// * `address` - Address of the fill
    /// * `size`    - Size of the fill
    pub fn set_fill(&mut self, address: u64, size: u64) {
        self.fill_size = size;
        if self.address == address {
            self.fill_overlaps = true;
        }
    }

    /// Get sub-section size
    ///
    /// If the [`fill_overlaps`](#structfield.fill_overlaps) is true, then the size will be [`fill_size`](#structfield.fill_size)
    /// else it will be the sum of [`fill_size`](#structfield.fill_size) and [`size`](#structfield.size)
    pub fn size(&self) -> u64 {
        match self.fill_overlaps {
            true => self.fill_size,
            false => self.size + self.fill_size,
        }
    }
}

impl FromStr for SubSection {
    type Err = ParserError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        if data.is_empty() {
            error!("Empty sub-section!");
            return Err(ParserError::InvalidMemoryMapSubSection);
        }
        let name_regex = Regex::new(&format!(r"^ {NAME_REGEX}")).unwrap();
        // let info_regex = Regex::new(&format!(
        //     r"{HEX_REGEX}\s+{HEX_REGEX}\s+{NAME_REGEX}\({NAME_REGEX}\)"
        // ))
        // .unwrap();
        let info_regex = Regex::new(&format!(r"\s+{HEX_REGEX}\s+{HEX_REGEX}")).unwrap();
        let fill_regex = Regex::new(&format!(r"^ \*fill\*\s+{HEX_REGEX}\s+{HEX_REGEX}")).unwrap();

        let mut lines = data.lines();
        let mut line = lines.next().unwrap().trim_end();

        let name = match name_regex.captures(line) {
            Some(cap) => cap.get(1).unwrap().as_str().to_string(),
            None => {
                error!("Invalid sub-section name: {line}");
                return Err(ParserError::InvalidMemoryMapSubSection);
            }
        };

        // Check if line contains also info or just the name
        if name.len() == (line.len() - 1) {
            line = match lines.next() {
                Some(l) => l,
                None => {
                    error!("Missing sub-section info line: {line}");
                    return Err(ParserError::InvalidMemoryMapSubSection);
                }
            };
        }

        let mut subsection = None;

        if let Some(cap) = info_regex.captures(line) {
            let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
            let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();
            // let lib = cap.get(3).unwrap().as_str().to_string();
            // let obj = cap.get(4).unwrap().as_str().to_string();

            subsection = Some(SubSection::new(name, address, size));
        } else {
            error!("Invalid sub-section info: {line}");
            return Err(ParserError::InvalidMemoryMapSubSection);
        }

        for line in lines {
            if let Some(cap) = fill_regex.captures(line) {
                let subsection = subsection.as_mut().unwrap();

                let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
                let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();

                if (address != subsection.address)
                    && ((subsection.address + subsection.size) != address)
                {
                    warn!("Sub-section/fill address mismatch: {line}");
                    // return Err(ParserError::InvalidMemoryMapSubSection);
                }

                if size > subsection.size {
                    warn!("Fill size bigger than sub-section size: {line}");
                    // return Err(ParserError::InvalidMemoryMapSubSection);
                }

                subsection.set_fill(address, size);
                continue;
            }

            warn!("Could not parse sub-section line: {line}");
        }

        subsection.ok_or(ParserError::InvalidMemoryMapSubSection)
    }
}