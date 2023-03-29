use std::{collections::HashMap, error::Error};

mod section;
mod sub_section;

use section::Section;
use sub_section::SubSection;

const NAME_REGEX: &str = r#"([[[:alnum:]]./*_"-]+)"#;
const HEX_REGEX: &str = "0x([[:xdigit:]]+)";

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidMemoryMapSection,
    InvalidMemoryMapSubSection,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ParserError::InvalidMemoryMapSection => "Invalid memory map section",
            ParserError::InvalidMemoryMapSubSection => "Invalid memory map sub-section",
        };
        write!(f, "{}", msg)
    }
}

impl Error for ParserError {}

pub struct Parser {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum MapFileSections {
    /// Corresponds to 'Archive member included to satisfy reference by file (symbol)' section
    ArchiveMembers,
    /// Corresponds to 'Allocating common symbols' section
    CommonSymbols,
    /// Corresponds to 'Discarded input sections' sections
    DiscardedInput,
    /// Corresponds to 'Memory Configuration' sections
    MemoryConfiguration,
    /// Corresponds to 'Linker script and memory map' section
    MemoryMap,
    ///
    None,
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self, data: &str) {
        let sections_info = Self::get_sections_pos(data);

        if let Some((start, end)) = sections_info.get(&MapFileSections::MemoryMap) {
            self.parse_memory_map(&data[*start..*end]);
        }
    }

    fn get_sections_pos(data: &str) -> HashMap<MapFileSections, (usize, usize)> {
        let mut pos: usize = 0;
        let mut start: usize = 0;
        let mut result = HashMap::new();
        let mut last_section = MapFileSections::None;
        let mut curr_section = MapFileSections::None;

        for line in data.lines() {
            if line.contains("Archive member included") {
                curr_section = MapFileSections::ArchiveMembers;
            } else if line.contains("Allocating common symbols") {
                curr_section = MapFileSections::CommonSymbols;
            } else if line.contains("Discarded input sections") {
                curr_section = MapFileSections::DiscardedInput;
            } else if line.contains("Memory Configuration") {
                curr_section = MapFileSections::MemoryConfiguration;
            } else if line.contains("Linker script and memory map") {
                curr_section = MapFileSections::MemoryMap;
            }

            if curr_section != last_section {
                result.insert(last_section, (start, pos));
                last_section = curr_section;
                // Skip section title and new line
                start = pos + line.len() + 1;
            }

            pos += line.len() + 1;
        }

        result.insert(last_section, (start, pos));

        result
    }

    /// Parse memory map section
    fn parse_memory_map(&mut self, data: &str) {
        for section in data.split("\n\n") {
            let _err = Section::parse_section_data(section);
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
