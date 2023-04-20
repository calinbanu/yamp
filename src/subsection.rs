//! Sub-section module
//!
//! This module contains the code to process and store information
//! regarding sub-sections of the memory map.
use crate::{ParserError, HEX_REGEX, NAME_REGEX};
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
        let name_regex = Regex::new(&format!(r"^ {NAME_REGEX}")).unwrap();
        let info_regex = Regex::new(&format!(
            r"{HEX_REGEX}\s+{HEX_REGEX}\s+{NAME_REGEX}\({NAME_REGEX}\)"
        ))
        .unwrap();
        let fill_regex = Regex::new(&format!(r"^ *fill*\s+{HEX_REGEX}\s+{HEX_REGEX}")).unwrap();
        let mut name = None;
        let mut subsection: Option<SubSection> = None;
        for line in data.lines() {
            if let Some(cap) = fill_regex.captures(line) {
                let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
                let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();
                match subsection {
                    Some(ref mut s) => s.set_fill(address, size),
                    None => {
                        return Err(ParserError::InvalidMemoryMapSubSection);
                    }
                }
            }
            if let Some(cap) = name_regex.captures(line) {
                name = Some(cap.get(1).unwrap().as_str().to_string());
            }
            if let Some(cap) = info_regex.captures(line) {
                let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
                let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();
                let lib = cap.get(3).unwrap().as_str().to_string();
                let obj = cap.get(4).unwrap().as_str().to_string();
            }
        }
        Err(ParserError::InvalidMemoryMapSubSection)
        // // Build regex
        // let regex = Regex::new(&format!(
        //     r"^{NAME_REGEX}\s+{HEX_REGEX}\s+{HEX_REGEX}\s+(.*)"
        // ))
        // .unwrap();
        // // Try to match
        // if let Some(cap) = regex.captures(data) {
        //     let name = cap.get(1).unwrap().as_str().to_string();
        //     let address = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();
        //     let size = u64::from_str_radix(cap.get(3).unwrap().as_str(), 16).unwrap();
        //     let sub_section = SubSection::new(name, address, size);
        //     Ok(sub_section)
        // } else {
        //     Err(ParserError::InvalidMemoryMapSubSection)
        // }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn from_str() {
//         let name = "test";
//         let address = 123;
//         let size = 1234;
//         let valid_str = format!("{name} {:#16x} {:#x} TEST", address, size);

//         let str_section = SubSection::from_str(&valid_str);
//         let new_section = SubSection::new(name.to_string(), address, size);
//         assert_eq!(Ok(new_section), str_section);

//         // Without '0x' in front of address and size
//         let invalid_str = format!("{name} 123 1234 TEST");
//         let str_section = SubSection::from_str(&invalid_str);
//         assert_eq!(Err(ParserError::InvalidMemoryMapSubSection), str_section);

//         // Missing size
//         let invalid_str = format!("{name} 1234 TEST");
//         let str_section = SubSection::from_str(&invalid_str);
//         assert_eq!(Err(ParserError::InvalidMemoryMapSubSection), str_section);

//         // Missing address and size
//         let invalid_str = format!("{name} TEST");
//         let str_section = SubSection::from_str(&invalid_str);
//         assert_eq!(Err(ParserError::InvalidMemoryMapSubSection), str_section);

//         // Space at the beginning
//         let invalid_str = format!(" {name} {:#16x} {:#x} TEST", address, size);
//         let str_section = SubSection::from_str(&invalid_str);
//         assert_eq!(Err(ParserError::InvalidMemoryMapSubSection), str_section);
//     }
// }
