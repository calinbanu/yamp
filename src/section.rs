//! Section module
//!
//! This module contains the code to process and store information
//! regarding section of the memory map.
use crate::{ParserError, SubSection, HEX_REGEX, NAME_REGEX};
use log::{error, warn};
use regex::Regex;
use std::{result, str::FromStr};

/// Structure containing memory map section information
#[derive(Debug, PartialEq, Eq)]
pub struct Section {
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
        let mut last_address: u64 = self.sub_sections[0].address();
        let mut last_size: u64 = self.sub_sections[0].size();
        let mut sum = 0;

        for section in self.sub_sections[1..].iter() {
            // If it has the same address
            if last_address == section.address() {
                // Keep current size and address
                last_size = section.size();
            } else {
                // If not, add to sum the last size
                sum += last_size;
                last_size = section.size();
                last_address = section.address();
            }
        }
        sum += last_size;
        sum
    }

    fn get_subsection_start_pos(data: &str) -> Vec<(usize, usize)> {
        let mut pos: usize = 0;
        let mut start: usize = 0;
        let mut result: Vec<(usize, usize)> = Vec::new();
        let subsection_start_regex = Regex::new(r"^ [[[:alnum:]]/.]").unwrap();
        for line in data.lines() {
            if line.contains("SORT_BY_")
                || line.is_empty()
                || line.starts_with(" *fill*")
                || line.starts_with(" FILL")
            {
                pos += line.len() + 1;
                continue;
            }

            if subsection_start_regex.is_match(line) {
                result.push((start, pos - 1));
                start = pos;
            }
            pos += line.len() + 1;
        }
        result.push((start, pos - 1));

        result
    }

    pub fn parser(data: &str) -> Result<Section, ParserError> {
        if data.trim().is_empty() {
            error!("Empty section!");
            return Err(ParserError::InvalidMemoryMapSection);
        }

        let subsections_start_pos = Self::get_subsection_start_pos(data);

        let (start, end) = &subsections_start_pos[0];
        let section_info = &data[*start..*end];

        let name_regex = Regex::new(&format!(r"^{NAME_REGEX}")).unwrap();
        let info_regex = Regex::new(&format!(r"{HEX_REGEX}\s+{HEX_REGEX}")).unwrap();

        let mut lines = section_info
            .lines()
            .filter(|l| !l.is_empty() && !l.contains("SORT_BY"));

        let mut line = lines.next().unwrap().trim_end();

        if line.contains("LOAD") {
            error!("Invalid section:\n{data}");
            return Err(ParserError::InvalidMemoryMapSection);
        }

        let name = match name_regex.captures(line) {
            Some(cap) => cap.get(1).unwrap().as_str().to_string(),
            None => {
                error!("Invalid section name: {line}");
                return Err(ParserError::InvalidMemoryMapSection);
            }
        };

        let mut section: Option<Section> = None;

        // Check if line contains also info or just the name
        if name.len() == line.len() {
            line = match lines.next() {
                Some(l) => l,
                None => {
                    return Ok(Section::new(name));
                }
            };
        }

        if let Some(cap) = info_regex.captures(line) {
            let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
            let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();
            // let lib = cap.get(3).unwrap().as_str().to_string();
            // let obj = cap.get(4).unwrap().as_str().to_string();

            section = Some(Section::new(name));
            section.as_mut().unwrap().address = Some(address);
            section.as_mut().unwrap().size = size;
        } else {
            section = Some(Section::new(name));
        }

        if subsections_start_pos.len() == 1 {
            return section.ok_or(ParserError::InvalidMemoryMapSection);
        }

        for (start, end) in &subsections_start_pos[1..] {
            let section_info = &data[*start..*end];
            match SubSection::from_str(section_info) {
                Ok(ss) => section.as_mut().unwrap().sub_sections.push(ss),
                Err(_) => error!("Could not parse subsection:\n{section_info}"),
            }
        }

        let section = section.unwrap();
        let sum = section.get_sub_sections_total_size();
        if sum != section.size {
            error!(
                "Size mismatch in {}: {} vs {}",
                section.name, sum, section.size
            );
        }
        Ok(section)
    }
}
