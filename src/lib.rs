use std::io::Write;
use std::{collections::HashMap, error::Error};

pub mod entry;
pub mod excelwriter;
pub mod object;
pub mod segment;
pub mod xmlwriter;

use entry::Entry;
use excelwriter::{ExcelWriter, ToExcelWriter};
use log::{error, info, warn};
use object::Object;
use regex::Regex;
use segment::Segment;
use xml::writer::XmlEvent;
use xmlwriter::{ToXmlWriter, XmlWriter};

const NAME_REGEX: &str = r#"([[[:alnum:]]./*_"-//]+)"#;
const HEX_REGEX: &str = "0x([[:xdigit:]]+)";

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidMemoryMapSegment,
    InvalidMemoryMapEntry,
    InvalidSegment,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ParserError::InvalidMemoryMapSegment => "Invalid memory map segment",
            ParserError::InvalidMemoryMapEntry => "Invalid memory map entry",
            ParserError::InvalidSegment => "Could not find segment",
        };
        write!(f, "{}", msg)
    }
}

impl Error for ParserError {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Section {
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
}

pub struct Parser {
    memory_map_segments: Vec<Segment>,
    memory_map_objects: HashMap<String, Object>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            memory_map_segments: vec![],
            memory_map_objects: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.memory_map_objects.clear();
        self.memory_map_segments.clear();
    }

    pub fn get_memory_map_segments(&self) -> &[Segment] {
        &self.memory_map_segments
    }

    pub fn get_memory_map_objects(&self) -> &HashMap<String, Object> {
        &self.memory_map_objects
    }

    pub fn add_segment(&mut self, segment: Segment) {
        for entry in segment.get_entries() {
            if let Some(obj_name) = entry.get_object_name() {
                if !self.memory_map_objects.contains_key(obj_name) {
                    self.memory_map_objects
                        .insert(obj_name.to_string(), Object::new(obj_name));
                }

                let obj = self.memory_map_objects.get_mut(obj_name).unwrap();
                obj.update_segment_size(segment.get_name(), entry.get_size());
            }
        }

        self.memory_map_segments.push(segment);
    }

    pub fn parse_entry_info(data: &str) -> Option<Entry> {
        if data.is_empty() {
            error!("Empty entry!");
            return None;
        }

        let name_regex = Regex::new(&format!(r"^ {NAME_REGEX}")).unwrap();
        // Entry info line can contain object name or lib and object names:
        // <address> <size> <lib name>(<obj name>)
        // <address> <size> <obj name>
        let info_regex = Regex::new(&format!(
            r"\s+{HEX_REGEX}\s+{HEX_REGEX}\s+(?:(?:{NAME_REGEX}\({NAME_REGEX}\))|(?:{NAME_REGEX}))"
        ))
        .unwrap();
        // *fill <address> <size>
        let fill_regex = Regex::new(&format!(r"^ \*fill\*\s+{HEX_REGEX}\s+{HEX_REGEX}")).unwrap();

        let mut iter = data.lines();
        let line = data.lines().next().unwrap();

        let name = match name_regex.captures(line) {
            Some(cap) => cap.get(1).unwrap().as_str(),
            None => {
                error!("Invalid entry name: {data}");
                return None;
            }
        };

        if line.trim().len() == name.len() {
            iter.next();
        }

        let mut entry: Option<Entry> = None;

        while let Some(line) = iter.next() {
            if let Some(cap) = info_regex.captures(line) {
                let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
                let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();

                let mut tmp = Entry::new(name, address, size, data);
                if let Some(object_name) = cap.get(4) {
                    tmp.set_object_name(object_name.as_str());
                    if let Some(library_name) = cap.get(3) {
                        tmp.set_library_name(library_name.as_str())
                    } else {
                        error!("Invalid parsing of object and library name:\n{data}");
                        break;
                    }
                } else if let Some(object_name) = cap.get(5) {
                    tmp.set_object_name(object_name.as_str())
                } else {
                    error!("Invalid parsing of object and library name:\n{data}");
                    break;
                }

                entry = Some(tmp);
                break;
            } else {
                // skip lines that contain linker information
                if !line.contains("*(SORT_BY_ALIGNMENT(") {
                    info!("Skipped line while parsing '{}' entry:\n{line}", name);
                }
            }
        }

        if entry.is_none() {
            error!("Could not parse entry:\n{data}");
            return None;
        }

        while let Some(line) = iter.next() {
            if let Some(cap) = fill_regex.captures(line) {
                let entry = entry.as_mut().unwrap();

                let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
                let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();

                if (address != entry.get_address())
                    && ((entry.get_address() + entry.get_size()) != address)
                {
                    warn!(
                        "Entry fill address mismatch:\n{} -> {line}",
                        entry.get_name()
                    );
                }

                if size > entry.get_size() {
                    warn!(
                        "Fill size bigger than entry size:\n{} -> {line}",
                        entry.get_name()
                    );
                }

                entry.set_fill(address, size);
                break;
            } else {
                // skip lines that contain linker information
                if !line.contains("*(SORT_BY_ALIGNMENT(") {
                    info!("Skipped line while parsing '{}' entry:\n{line}", name);
                }
            }
        }

        // TODO(calin) parse rest of lines

        entry
    }

    pub fn parse_segment_info(data: &str) -> Option<Segment> {
        if data.trim().is_empty() {
            error!("Empty segment!");
            return None;
        }

        let name_regex = Regex::new(&format!(r"^{NAME_REGEX}")).unwrap();
        let info_regex = Regex::new(&format!(r"\s+{HEX_REGEX}\s+{HEX_REGEX}")).unwrap();

        let mut iter = data.lines().peekable();
        let line = iter.peek().unwrap();

        let name = match name_regex.captures(line) {
            Some(cap) => cap.get(1).unwrap().as_str(),
            None => {
                error!("Invalid segment name:\n{data}");
                return None;
            }
        };

        // Check if it is a directive
        if name == "LOAD" {
            error!("Invalid segment name:\n{data}");
            return None;
        }

        let mut segment = None;
        for line in iter {
            if let Some(cap) = info_regex.captures(line) {
                let address = u64::from_str_radix(cap.get(1).unwrap().as_str(), 16).unwrap();
                let size = u64::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap();

                let mut tmp = Segment::new(name);
                tmp.set_size_and_address(size, address);
                segment = Some(tmp);
                break;
            }
        }

        if segment.is_none() {
            segment = Some(Segment::new(name));
        }

        segment
    }

    pub fn split_segment(data: &str) -> Vec<(usize, usize)> {
        let mut pos: usize = 0;
        let mut start: usize = 0;
        let mut result: Vec<(usize, usize)> = Vec::new();
        let entry_start_regex = Regex::new(r"^ [[[:alnum:]]/.]").unwrap();
        for line in data.lines() {
            if line.contains("(SORT_BY_")
                || line.is_empty()
                || line.starts_with(" *fill*")
                || line.starts_with(" FILL")
            {
                pos += line.len() + 1;
                continue;
            }

            if entry_start_regex.is_match(line) {
                result.push((start, pos - 1));
                start = pos;
            }
            pos += line.len() + 1;
        }
        result.push((start, pos - 1));

        result
    }

    fn parse_segment(data: &str) -> Option<Segment> {
        let start_end = Self::split_segment(data);

        let segment_str = &data[start_end[0].0..start_end[0].1];
        let mut segment = Self::parse_segment_info(segment_str)?;

        for (start, end) in &start_end[1..] {
            let entry_str = &data[*start..*end];

            if let Some(entry) = Self::parse_entry_info(entry_str) {
                // self.update_objects(&segment, &entry);
                segment.add_entry(entry);
            } else {
                error!("Could not parse Entry:\n{entry_str}");
                return None;
            }
        }

        if segment.get_address().is_some() {
            let segment_entry_sum = segment.get_entries_total_size();
            let segment_size = segment.get_size().unwrap();
            if segment_entry_sum != segment_size {
                warn!(
                    "Size mismatch in {}: {} vs {}",
                    segment.get_name(),
                    segment_entry_sum,
                    segment_size
                );
            }
        }

        Some(segment)
    }

    pub fn parse_section(line: &str) -> Option<Section> {
        let mut ret = None;

        if line.starts_with("Archive member included") {
            ret = Some(Section::ArchiveMembers);
        } else if line.starts_with("Allocating common symbols") {
            ret = Some(Section::CommonSymbols);
        } else if line.starts_with("Discarded input sections") {
            ret = Some(Section::DiscardedInput);
        } else if line.starts_with("Memory Configuration") {
            ret = Some(Section::MemoryConfiguration);
        } else if line.starts_with("Linker script and memory map") {
            ret = Some(Section::MemoryMap);
        }

        ret
    }

    pub fn parse(data: &str) -> Self {
        let mut current_section = None;

        let mut parser: Parser = Self {
            memory_map_segments: vec![],
            memory_map_objects: HashMap::new(),
        };

        for chunk in data.split("\n\n") {
            if chunk.is_empty() {
                continue;
            }

            let first_line = chunk.lines().next().unwrap();
            if let Some(section) = current_section {
                match section {
                    Section::ArchiveMembers => {
                        if let Some(section) = Self::parse_section(first_line) {
                            current_section = Some(section);
                        }
                    }
                    Section::CommonSymbols => {
                        if let Some(section) = Self::parse_section(first_line) {
                            current_section = Some(section);
                        }
                    }
                    Section::DiscardedInput => {
                        if let Some(section) = Self::parse_section(first_line) {
                            current_section = Some(section);
                        }
                    }
                    Section::MemoryConfiguration => {
                        if let Some(section) = Self::parse_section(first_line) {
                            current_section = Some(section);
                        }
                    }
                    Section::MemoryMap => {
                        if let Some(segment) = Self::parse_segment(chunk) {
                            parser.add_segment(segment);
                        } else if let Some(section) = Self::parse_section(first_line) {
                            current_section = Some(section);
                        }
                    }
                }
            } else {
                current_section = Self::parse_section(first_line);
            }
        }
        parser
    }
}

// Helper functions for ToXmlWriter::to_xml_writer trait implementation
impl Parser {
    fn write_segments<W: Write>(&self, writer: &mut XmlWriter<W>) {
        let count = self.memory_map_segments.len();
        if count > 0 {
            writer.start_element(
                XmlEvent::start_element("segments").attr("count", &count.to_string()),
            );
            self.memory_map_segments
                .iter()
                .for_each(|s| s.to_xml_writer(writer));
            writer.end_element();
        }
    }

    fn write_objects<W: Write>(&self, writer: &mut XmlWriter<W>) {
        let count = self.memory_map_objects.len();
        if count > 0 {
            writer.start_element(
                XmlEvent::start_element("objects").attr("count", &count.to_string()),
            );
            self.memory_map_objects
                .values()
                .for_each(|o| o.to_xml_writer(writer));
            writer.end_element();
        }
    }
}

impl<W: Write> ToXmlWriter<W> for Parser {
    fn to_xml_writer(&self, writer: &mut XmlWriter<W>) {
        // writer.start_element(XmlEvent::start_element("section").attr("name", "ArchiveMembers"));
        // writer.end_element();

        // writer.start_element(XmlEvent::start_element("section").attr("name", "CommonSymbols"));
        // writer.end_element();

        // writer.start_element(XmlEvent::start_element("section").attr("name", "DiscardedInput"));
        // writer.end_element();

        // writer
        //     .start_element(XmlEvent::start_element("section").attr("name", "MemoryConfiguration"));
        // writer.end_element();

        writer.start_element(XmlEvent::start_element("section").attr("name", "MemoryMap"));

        self.write_segments(writer);

        self.write_objects(writer);

        writer.end_element();
    }
}

impl ToExcelWriter for Parser {
    fn to_excel_writer<'a, 'b>(&'a self, writer: &mut ExcelWriter<'b>)
    where
        'a: 'b,
    {
        for segment in self.memory_map_segments.iter() {
            segment.to_excel_writer(writer);
        }

        for object in self.memory_map_objects.values() {
            writer.write_object(object);
        }
    }
}
