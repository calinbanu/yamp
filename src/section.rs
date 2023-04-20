//! Section module
//!
//! This module contains the code to process and store information
//! regarding section of the memory map.
use crate::{ParserError, SubSection, HEX_REGEX, NAME_REGEX};
use log::error;
use regex::Regex;
use std::str::FromStr;

/// Structure containing memory map section information
#[derive(Debug, PartialEq, Eq)]
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

        // If it does not contains any space, it means address and size should be on the next line
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

    /// Parse string and tries to match fill address and size
    ///
    /// # Arguments
    ///
    /// * `data` - Data to parse
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

    /// Consumes next lines and tries to parse them as sub-sections
    ///
    /// # Arguments
    ///
    /// * `iter` - Line by line iterator
    fn parse_sub_sections<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) {
        // Current sub-section that beeing processed
        let mut current_sub_section: Option<SubSection> = None;
        // Sub-section name regex
        let name_regex = Regex::new(&format!(r"^\s{NAME_REGEX}")).unwrap();

        while let Some(line) = iter.next() {
            // Fill line
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
            // Normal sub-section line
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
                // Push last sub-section
                if let Some(v) = sub_section {
                    self.sub_sections.push(v)
                }
            }
        }
        // If it is not None, push it
        if let Some(v) = current_sub_section {
            self.sub_sections.push(v)
        }
    }

    /// Tries to parse data as a section followed by sub-sections
    ///
    /// # Arguments
    ///
    /// * `data` - Data to parse
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

        // Tries to parse next lines as sub-sections
        section.parse_sub_sections(&mut lines);

        // Check to see if the section size is the same as the sum of sub-sections
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn basic_new() {
//     //     let name = "test".to_string();

//     //     let section = Section::new(name.clone());

//     //     assert_eq!(section.name, name);
//     //     assert_eq!(section.address, None);
//     //     assert_eq!(section.size, 0);
//     //     assert!(section.sub_sections.is_empty());
//     //     assert_eq!(section.get_sub_sections_total_size(), 0);
//     // }

//     #[test]
//     fn from_str() {
//         let name = "test";
//         let address = 123;
//         let size = 1234;

//         let mut valid_section = Section::new(name.to_string());

//         let string = format!("{name}");
//         let section = Section::from_str(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         valid_section.address = Some(address);
//         valid_section.size = size;

//         let string = format!("{name} {:#016x} {:#x}", address, size);
//         let section = Section::from_str(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         // Space in front of name
//         let string = format!(" {name}");
//         let section = Section::from_str(&string);
//         assert!(section.is_err());

//         // Size is missing
//         let string = format!(" {name} {:#016x}", address);
//         let section = Section::from_str(&string);
//         assert!(section.is_err());

//         // Address is missing
//         let string = format!(" {name} {:#x}", size);
//         let section = Section::from_str(&string);
//         assert!(section.is_err());

//         // Missing '0x'
//         let string = format!(" {name} 123 1234");
//         let section = Section::from_str(&string);
//         assert!(section.is_err());
//     }

//     #[test]
//     fn parse_section_data_no_sub_section() {
//         let name = "test";
//         let address = 123;
//         let size = 1234;

//         let mut valid_section = Section::new(name.to_string());

//         let string = format!("{name}");
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!("{name}\n *(SORT_BY_ALIGNMENT({name}))\n");
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         valid_section.address = Some(address);
//         valid_section.size = size;

//         let string = format!("{name} {:#016x} {:#x}\n", address, size);
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!("{name}\n {:#016x} {:#x}\n", address, size);
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{name} {:#016x} {:#x}\n *(SORT_BY_ALIGNMENT({name}))\n",
//             address, size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{name}\n {:#016x} {:#x}\n *(SORT_BY_ALIGNMENT({name}))\n",
//             address, size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));
//     }

//     #[test]
//     fn parse_section_data_one_sub_section_no_fill() {
//         let section_name = "test";
//         let section_address = 123;
//         let section_size = 1234;

//         let sub_section_name = ".test.test";
//         let sub_section_address = 123;
//         let sub_section_size = 1234;

//         let mut valid_section = Section::new(section_name.to_string());
//         let valid_sub_section = SubSection::new(
//             sub_section_name.to_string(),
//             sub_section_address,
//             sub_section_size,
//         );
//         valid_section.sub_sections.push(valid_sub_section);

//         let string = format!(
//             "{section_name}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n",
//             sub_section_address, sub_section_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         valid_section.address = Some(section_address);
//         valid_section.size = section_size;

//         let string = format!(
//             "{section_name} {:#016x} {:#x}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n",
//             section_address, section_size, sub_section_address, sub_section_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name}\n {:#016x} {:#x}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n",
//             section_address, section_size, sub_section_address, sub_section_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name} {:#016x} {:#x}\n {sub_section_name} {:#016x} {:#x} TEST\n",
//             section_address, section_size, sub_section_address, sub_section_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name}\n {:#016x} {:#x}\n {sub_section_name} {:#016x} {:#x} TEST\n",
//             section_address, section_size, sub_section_address, sub_section_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));
//     }

//     #[test]
//     fn parse_section_data_one_sub_section_with_fill_no_overlap() {
//         let section_name = "test";
//         let section_address = 123;
//         let section_size = 1234;

//         let sub_section_name = ".test.test";
//         let sub_section_address = 123;
//         let sub_section_size = 1234;
//         let sub_section_fill_address = sub_section_address + 10;
//         let sub_section_fill_size = 12;

//         let mut valid_section = Section::new(section_name.to_string());
//         let mut valid_sub_section = SubSection::new(
//             sub_section_name.to_string(),
//             sub_section_address,
//             sub_section_size,
//         );
//         valid_sub_section.set_fill(sub_section_fill_address, sub_section_fill_size);
//         valid_section.sub_sections.push(valid_sub_section);

//         let string = format!(
//             "{section_name}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         valid_section.address = Some(section_address);
//         valid_section.size = section_size;

//         let string = format!(
//             "{section_name} {:#016x} {:#x}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             section_address, section_size, sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name}\n {:#016x} {:#x}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             section_address, section_size, sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name} {:#016x} {:#x}\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             section_address, section_size, sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name}\n {:#016x} {:#x}\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             section_address, section_size, sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));
//     }

//     #[test]
//     fn parse_section_data_one_sub_section_with_fill_with_overlap() {
//         let section_name = "test";
//         let section_address = 123;
//         let section_size = 1234;

//         let sub_section_name = ".test.test";
//         let sub_section_address = 123;
//         let sub_section_size = 1234;
//         let sub_section_fill_address = sub_section_address;
//         let sub_section_fill_size = 12;

//         let mut valid_section = Section::new(section_name.to_string());
//         let mut valid_sub_section = SubSection::new(
//             sub_section_name.to_string(),
//             sub_section_address,
//             sub_section_size,
//         );
//         valid_sub_section.set_fill(sub_section_fill_address, sub_section_fill_size);
//         valid_section.sub_sections.push(valid_sub_section);

//         let string = format!(
//             "{section_name}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         valid_section.address = Some(section_address);
//         valid_section.size = section_size;

//         let string = format!(
//             "{section_name} {:#016x} {:#x}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             section_address, section_size, sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name}\n {:#016x} {:#x}\n  *(SORT_BY_ALIGNMENT({section_name}))\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             section_address, section_size, sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name} {:#016x} {:#x}\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             section_address, section_size, sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));

//         let string = format!(
//             "{section_name}\n {:#016x} {:#x}\n {sub_section_name} {:#016x} {:#x} TEST\n *fill* {:#016x} {:#x}\n",
//             section_address, section_size, sub_section_address, sub_section_size, sub_section_fill_address, sub_section_fill_size
//         );
//         let section = Section::parse_section_data(&string);
//         assert!(section.is_ok());
//         assert!(section.unwrap().eq(&valid_section));
//     }
// }
