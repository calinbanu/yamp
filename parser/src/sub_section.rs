//! Sub-section module
//!
//! This module contains the code to process and store information
//! regarding sub-sections of the memory map.
use crate::{ParserError, HEX_REGEX, NAME_REGEX};
use regex::Regex;
use std::str::FromStr;

/// Structure containing memory map sub-section information
#[derive(Debug, PartialEq)]
pub(crate) struct SubSection {
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
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get sub-section address
    pub fn get_address(&self) -> u64 {
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
    pub fn get_size(&self) -> u64 {
        match self.fill_overlaps {
            true => self.fill_size,
            false => self.size + self.fill_size,
        }
    }
}

impl FromStr for SubSection {
    type Err = ParserError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        // Build regex
        let regex = Regex::new(&format!(
            r"^{NAME_REGEX}\s+{HEX_REGEX}\s+{HEX_REGEX}\s+(.*)"
        ))
        .unwrap();
        // Try to match
        if let Some(cap) = regex.captures(data) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let address = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();
            let size = u64::from_str_radix(cap.get(3).unwrap().as_str(), 16).unwrap();
            let sub_section = SubSection::new(name, address, size);
            Ok(sub_section)
        } else {
            Err(ParserError::InvalidMemoryMapSubSection)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn basic_new() {
        let name = "test".to_string();
        let address = 0;
        let size = 0;

        let section = SubSection::new(name.clone(), address, size);

        assert_eq!(section.get_name(), name);
        assert_eq!(section.get_address(), address);
        assert_eq!(section.get_size(), size);
    }

    #[test]
    fn test_size_non_overlaping() {
        let address = 0;
        let size = 1;
        let fill_address = 1;
        let fill_size = 2;

        let mut section = SubSection::new("test".to_string(), address, size);
        section.set_fill(fill_address, fill_size);

        assert_eq!(section.get_size(), size + fill_size)
    }

    #[test]
    fn test_size_overlaping() {
        let address = 0;
        let size = 1;
        let fill_address = 0;
        let fill_size = 2;

        let mut section = SubSection::new("test".to_string(), address, size);
        section.set_fill(fill_address, fill_size);

        assert_eq!(section.get_size(), fill_size)
    }

    #[test]
    fn from_str() {
        let name = "test";
        let address = 0;
        let size = 0;
        let valid_str = format!("{name} 0x{address} 0x{size} TEST").to_string();

        let str_section = SubSection::from_str(&valid_str);
        let new_section = SubSection::new(name.to_string(), address, size);
        assert_eq!(Ok(new_section), str_section);

        // Without '0x' in front of address and size
        let invalid_str = format!("{name} {address} {size} TEST").to_string();
        let str_section = SubSection::from_str(&invalid_str);
        assert_eq!(Err(ParserError::InvalidMemoryMapSubSection), str_section);

        // Missing size
        let invalid_str = format!("{name} {address} TEST").to_string();
        let str_section = SubSection::from_str(&invalid_str);
        assert_eq!(Err(ParserError::InvalidMemoryMapSubSection), str_section);

        // Missing address and size
        let invalid_str = format!("{name} TEST").to_string();
        let str_section = SubSection::from_str(&invalid_str);
        assert_eq!(Err(ParserError::InvalidMemoryMapSubSection), str_section);

        // Space at the beginning
        let invalid_str = format!(" {name} 0x{address} 0x{size} TEST").to_string();
        let str_section = SubSection::from_str(&invalid_str);
        assert_eq!(Err(ParserError::InvalidMemoryMapSubSection), str_section);
    }
}
