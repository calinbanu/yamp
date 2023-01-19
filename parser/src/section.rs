//! Section module
//!
//! This module contains the code to process and store information
//! regarding section of the memory map.
use crate::{ParserError, SubSection, HEX_REGEX, NAME_REGEX};
use log::error;
use regex::Regex;
use std::str::FromStr;

/// Structure containing memory map section information
#[derive(Debug, PartialEq)]
pub(crate) struct Section {
    /// Section name
    name: String,
    /// Start address
    address: Option<u64>,
    /// Section size. 0 if [`address`](#structfield.address) is missing
    size: u64,
    /// List of parsed sub-sections
    sub_sections: Vec<SubSection>,
}

impl Section {
    /// Creates a new `Section`
    ///
    /// # Arguments
    ///
    /// * `name` - Section name
    fn new(name: String) -> Self {
        Self {
            name,
            address: None,
            size: 0,
            sub_sections: vec![],
        }
    }

    /// Calculates the sum of all sub-sections
    fn get_sub_sections_total_size(&self) -> u64 {
        if self.sub_sections.is_empty() {
            return 0;
        }

        // We can have multiple consecutive sub-sections that have the same address
        let mut last_address: u64 = self.sub_sections[0].get_address();
        let mut last_size: u64 = self.sub_sections[0].get_size();
        let mut sum = 0;

        for section in self.sub_sections[1..].iter() {
            // If it has the same address
            if last_address == section.get_address() {
                // Keep current size and address
                last_size = section.get_size();
            } else {
                // If not, add to sum the last size
                sum += last_size;
                last_size = section.get_size();
                last_address = section.get_address();
            }
        }
        sum += last_size;
        sum
    }

    /// Consumes next lines and tries to parse them as section information
    ///
    /// # Arguments
    ///
    /// * `iter` - Line by line iterator
    fn parse_section_info<'a>(
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result<Self, ParserError> {
        // Get first line of the section
        let section_info = iter.next();
        if section_info.is_none() {
            return Err(ParserError::InvalidMemoryMapSection);
        }
        // Its valid here so unwrap
        let mut section_info = section_info.unwrap().trim().to_string();
        // If it contains linker directive skip it
        if section_info.contains("LOAD") {
            return Err(ParserError::InvalidMemoryMapSection);
        }
        // If it does not contains any space, it means addr and size should be on the next line
        if !section_info.contains(' ') {
            if let Some(line) = iter.next() {
                if !line.contains("SORT_BY") {
                    section_info.push(' ');
                    section_info += line;
                }
            } else {
                // Section has only one valid line and that contains the section name
                // Address is None and size is 0.
                return Ok(Section::new(section_info));
            }
        }

        Section::from_str(&section_info)
    }

    fn parse_fill_data(data: &str) -> Option<(u64, u64)> {
        // Regex for fill line
        let regex = Regex::new(&format!(r"^\s\*fill\*\s+{HEX_REGEX}\s+{HEX_REGEX}")).unwrap();
        if let Some(cap) = regex.captures(data) {
            // Get fill address
            let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
            // Get fill size
            let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();
            Some((address, size))
        } else {
            None
        }
    }

    fn parse_sub_sections<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) {
        let mut current_sub_section: Option<SubSection> = None;
        let name_regex = Regex::new(&format!(r"^\s{NAME_REGEX}")).unwrap();

        while let Some(line) = iter.next() {
            if line.starts_with(" *fill*") {
                if let Some(sub_section) = &mut current_sub_section {
                    if let Some((address, size)) = Section::parse_fill_data(line) {
                        sub_section.set_fill(address, size);
                    } else {
                        error!("Could not match on fill line: {line}");
                    }
                } else {
                    error!("Fill while current sub-section is not valid");
                }
            } else if name_regex.is_match(line) {
                let mut line = line.trim().to_string();
                if !line.contains(' ') {
                    line.push(' ');
                    line += iter.next().unwrap();
                }

                let sub_section = match SubSection::from_str(&line) {
                    Ok(sub_section) => current_sub_section.replace(sub_section),
                    Err(_) => {
                        error!("Could not parse sub-section information: {line}");
                        current_sub_section.take()
                    }
                };
                sub_section.map(|v| self.sub_sections.push(v));
            }
        }

        current_sub_section.map(|v| self.sub_sections.push(v));
    }

    pub fn parse_section_data(data: &str) -> Result<Self, ParserError> {
        // Split data in lines
        let mut lines = data.lines().filter(|line| !line.is_empty());

        // Get new section from the first lines of data
        let mut section = match Self::parse_section_info(&mut lines) {
            Ok(section) => section,
            Err(e) => {
                error!("Invalid section: \n{data}");
                return Err(e);
            }
        };

        // Skip lines that contain linker symbols
        let mut lines =
            lines.filter(|line| !line.contains("SORT_BY") && !line.contains("FILL mask"));

        section.parse_sub_sections(&mut lines);

        let size = section.get_sub_sections_total_size();
        if size != section.size {
            error!(
                "Section size differs from sub-sections size sum: {} ({} vs {})",
                section.name, section.size, size
            );
        }

        Ok(section)
    }
}

impl FromStr for Section {
    type Err = ParserError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        // Build regex
        let regex =
            Regex::new(&format!(r"^{NAME_REGEX}(?:\s+{HEX_REGEX}\s+{HEX_REGEX})?")).unwrap();
        // Try to match
        if let Some(cap) = regex.captures(data) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let mut section = Section::new(name);

            if let Some(addr) = cap.get(2) {
                section.address = Some(u64::from_str_radix(addr.as_str(), 16).unwrap())
            }

            if let Some(size) = cap.get(3) {
                section.size = u64::from_str_radix(size.as_str(), 16).unwrap();
            }
            Ok(section)
        } else {
            Err(ParserError::InvalidMemoryMapSection)
        }
    }
}

impl PartialOrd for Section {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.address.cmp(&other.address))
    }
}
