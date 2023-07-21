use parser::{
    entry::Entry,
    segment::Segment,
    xmlwriter::{ToXmlWriter, XmlWriter},
    Parser, Section,
};

mod uthelper;
use uthelper::*;
use xml::{reader::XmlEvent, ParserConfig};

#[test]
fn parse_section_fn() {
    // Random string, should return None
    let line = get_random_string(RAND_STRING_SIZE);
    assert_eq!(Parser::parse_section(&line), None);

    let archive_members_line = "Archive member included to satisfy reference by file (symbol)";
    assert_eq!(
        Parser::parse_section(archive_members_line),
        Some(Section::ArchiveMembers)
    );

    let common_symbols_line = "Allocating common symbols";
    assert_eq!(
        Parser::parse_section(common_symbols_line),
        Some(Section::CommonSymbols)
    );

    let discarded_input_line = "Discarded input sections";
    assert_eq!(
        Parser::parse_section(discarded_input_line),
        Some(Section::DiscardedInput)
    );

    let memory_configuration_line = "Memory Configuration";
    assert_eq!(
        Parser::parse_section(memory_configuration_line),
        Some(Section::MemoryConfiguration)
    );

    let memory_map_line = "Linker script and memory map";
    assert_eq!(
        Parser::parse_section(memory_map_line),
        Some(Section::MemoryMap)
    );
}

fn entry_sub_test(single_line: bool, with_lib: bool, with_fill: bool, fill_overlaps: bool) {
    let entry_name = if single_line {
        get_random_string(14)
    } else {
        get_random_string(18)
    };

    let entry_address = get_random_number(0xFFFFFFFFFFFFFFFF);
    let entry_size = get_random_number(0xFFFFFFFF);
    let entry_lib = get_random_string(10);
    let entry_object = get_random_string(10);
    let entry_fill_size = if with_fill {
        get_random_number(0xFFFFFFFF)
    } else {
        0
    };
    let entry_fill_overlaps = if with_fill { fill_overlaps } else { false };
    let entry_fill_address = if fill_overlaps {
        entry_address
    } else {
        get_random_number(0xFFFFFFFFFFFFFFFF)
    };

    let entry_address_str = format!("{:#016x}", entry_address);
    let entry_size_str = format!("{:#x}", entry_size);
    let mut entry_str = if single_line {
        format!(
            " {:14} {} {:>10}",
            &entry_name, entry_address_str, entry_size_str
        )
    } else {
        format!(
            " {:14}\n                {} {:>10}",
            &entry_name, entry_address_str, entry_size_str
        )
    };
    if with_lib {
        entry_str += &format!(" {}({})", entry_lib, entry_object);
    } else {
        entry_str += &format!(" {}", entry_object);
    }

    if with_fill {
        let entry_fill_size_str = format!("{:#x}", entry_fill_size);
        let entry_fill_address_str = format!("{:#016x}", entry_fill_address);
        entry_str += &format!(
            "\n *fill*         {} {:>10}",
            entry_fill_address_str, entry_fill_size_str
        );
    }

    let entry = Parser::parse_entry_info(&entry_str).unwrap();
    assert_eq!(entry.get_name(), &entry_name);
    assert_eq!(entry.get_original_size(), entry_size);
    assert_eq!(entry.get_address(), entry_address);
    if with_lib {
        assert_eq!(entry.get_library_name(), Some(entry_lib.as_str()));
    } else {
        assert_eq!(entry.get_library_name(), None);
    }

    assert_eq!(entry.get_object_name(), Some(entry_object.as_str()));
    if with_fill {
        if fill_overlaps {
            assert_eq!(entry.get_size(), entry_fill_size);
        } else {
            assert_eq!(entry.get_size(), entry_fill_size + entry_size);
        }
    } else {
        assert_eq!(entry.get_size(), entry_size);
    }
    assert_eq!(entry.get_data(), &entry_str);
    assert_eq!(entry.get_fill_overlaps(), entry_fill_overlaps);
    assert_eq!(entry.get_fill_size(), entry_fill_size);
}

#[test]
fn parse_entry_info_fn() {
    // Test empty string
    let empty = "";
    assert_eq!(Parser::parse_entry_info(empty), None);
    // TODO(calin) add option for SORT_BY_ALIGNMENT line and symbols parsing
    // no fill
    entry_sub_test(true, true, false, false);
    entry_sub_test(true, false, false, false);
    entry_sub_test(false, true, false, false);
    entry_sub_test(false, false, false, false);

    // with fill, no overlap
    entry_sub_test(true, true, true, false);
    entry_sub_test(true, false, true, false);
    entry_sub_test(false, true, true, false);
    entry_sub_test(false, false, true, false);

    // with fill, with overlap
    entry_sub_test(true, true, true, true);
    entry_sub_test(true, false, true, true);
    entry_sub_test(false, true, true, true);
    entry_sub_test(false, false, true, true);
}

fn segment_sub_test(single_line: bool, with_addr_size: bool) {
    let segment_name = if single_line {
        get_random_string(14)
    } else {
        get_random_string(18)
    };

    let segment_address = get_random_number(0xFFFFFFFFFFFFFFFF);
    let segment_size = get_random_number(0xFFFFFFFF);

    let segment_address_str = format!("{:#016x}", segment_address);
    let segment_size_str = format!("{:#x}", segment_size);
    let segment_str = if with_addr_size {
        if single_line {
            format!(
                "{:15} {} {:>10}",
                &segment_name, segment_address_str, segment_size_str
            )
        } else {
            format!(
                "{}\n                {} {:>10}",
                &segment_name, segment_address_str, segment_size_str
            )
        }
    } else {
        segment_name.clone()
    };

    let segment = Parser::parse_segment_info(&segment_str).unwrap();
    assert_eq!(segment.get_name(), &segment_name);
    assert_eq!(segment.get_entries_total_size(), 0);
    assert_eq!(segment.get_entries().len(), 0);
    if with_addr_size {
        assert_eq!(segment.get_address(), Some(segment_address));
        assert_eq!(segment.get_size(), Some(segment_size));
    } else {
        assert_eq!(segment.get_address(), None);
        assert_eq!(segment.get_size(), None);
    }
}

#[test]
fn parse_segment_info_fn() {
    let empty = "";
    assert_eq!(Parser::parse_segment_info(empty), None);

    // Should return None if the first line contains LOAD directive
    let load_directive = format!("    LOAD {}", get_random_string(10));
    assert_eq!(Parser::parse_segment_info(&load_directive), None);

    // Name should not have spaces before
    let space_before_name = format!(" {}", get_random_string(10));
    assert_eq!(Parser::parse_segment_info(&space_before_name), None);

    segment_sub_test(false, false);
    segment_sub_test(false, true);
    segment_sub_test(true, false);
    segment_sub_test(true, true);
}

#[test]
fn split_segment_fn() {
    let mut pos: usize = 0;
    let mut split_vec = vec![];
    let mut test_str = String::new();

    let address_str = format!("{:#016x}", get_random_number(0xFFFFFFFFFFFFFFFF));
    let size_str = format!("{:#x}", get_random_number(0xFFFFFFFF));
    test_str += &format!(
        "{:15} {} {:>10}\n",
        get_random_string(15),
        address_str,
        size_str
    );

    test_str += &format!(" *(SORT_BY_ALIGNMENT({}))\n", get_random_string(15));

    split_vec.push((pos, test_str.len() - 1));
    pos = test_str.len();

    test_str += &format!(
        " {:14} {} {:>10} {}({})\n",
        get_random_string(14),
        address_str,
        size_str,
        get_random_string(14),
        get_random_string(14)
    );

    test_str += &format!(
        "                {}                {}\n",
        address_str,
        get_random_string(14)
    );

    test_str += &format!(" *fill*         {} {:>10}\n", address_str, size_str);

    split_vec.push((pos, test_str.len() - 1));
    pos = test_str.len();

    test_str += &format!(
        " {:14} {} {:>10} {}({})\n",
        get_random_string(14),
        address_str,
        size_str,
        get_random_string(14),
        get_random_string(14)
    );

    test_str += &format!(
        "                {}                {}\n",
        address_str,
        get_random_string(14)
    );

    test_str += " FILL mask 0x00\n";

    split_vec.push((pos, test_str.len() - 1));
    pos = test_str.len();

    test_str += &format!(
        " {:14} {} {:>10} {}({})\n",
        get_random_string(14),
        address_str,
        size_str,
        get_random_string(14),
        get_random_string(14)
    );

    test_str += &format!(
        "                {}                {}\n",
        address_str,
        get_random_string(14)
    );

    test_str += &format!(" *(SORT_BY_ALIGNMENT({}))\n", get_random_string(15));

    split_vec.push((pos, test_str.len() - 1));
    pos = test_str.len();

    assert_eq!(Parser::split_segment(&test_str), split_vec);
}

#[test]
fn add_segment_fn() {
    let mut parser = Parser::new();

    assert!(parser.get_memory_map_segments().is_empty());
    assert!(parser.get_memory_map_objects().is_empty());

    let name = get_random_string(RAND_STRING_SIZE);
    let address = get_random_number(RAND_NUMBER_MAX);
    let size = get_random_number(RAND_NUMBER_MAX);

    let mut segment = Segment::new(&name);
    segment.set_size_and_address(size, address);

    for _ in 0..ENTRIES_COUNT {
        let name = get_random_string(RAND_STRING_SIZE);
        let data = get_random_string(RAND_STRING_SIZE);
        let address = get_random_number(RAND_NUMBER_MAX);
        let size = get_random_number(RAND_NUMBER_MAX);

        let entry = Entry::new(&name, address, size, &data);

        segment.add_entry(entry.clone());
    }

    parser.add_segment(segment.clone());

    assert_eq!(parser.get_memory_map_segments(), &[segment]);
    assert!(parser.get_memory_map_objects().is_empty());

    parser.clear();

    assert!(parser.get_memory_map_segments().is_empty());
    assert!(parser.get_memory_map_objects().is_empty());

    let mut segment = Segment::new(&name);
    segment.set_size_and_address(size, address);

    let object = get_random_string(RAND_STRING_SIZE);
    let mut sum = 0;
    for _ in 0..ENTRIES_COUNT {
        let name = get_random_string(RAND_STRING_SIZE);
        let data = get_random_string(RAND_STRING_SIZE);
        let address = get_random_number(RAND_NUMBER_MAX);
        let size = get_random_number(RAND_NUMBER_MAX);
        sum += size;

        let mut entry = Entry::new(&name, address, size, &data);
        entry.set_object_name(&object);

        segment.add_entry(entry.clone());
    }

    parser.add_segment(segment.clone());

    assert_eq!(parser.get_memory_map_segments(), &[segment]);
    assert_eq!(parser.get_memory_map_objects().len(), 1);
    assert_eq!(
        parser
            .get_memory_map_objects()
            .get(&object)
            .unwrap()
            .get_total_size(),
        sum
    );
}

#[test]
fn test_xml_output() {
    let sink = UTSinkSource::new();
    let mut writer = XmlWriter::new_empty(sink.clone());

    let parser = Parser::new();

    parser.to_xml_writer(&mut writer);

    drop(writer);

    let mut parser = ParserConfig::default()
        .ignore_root_level_whitespace(true)
        .trim_whitespace(true)
        .create_reader(sink);

    check_start_document_event(parser.next().unwrap());

    if let XmlEvent::StartElement {
        name,
        attributes,
        namespace: _,
    } = parser.next().unwrap()
    {
        assert_eq!(name.local_name, "section");
        assert_eq!(name.namespace, Option::None);
        assert_eq!(name.prefix, Option::None);
        assert!(matches!(attributes.len(), 1));

        let attr = &attributes[0];
        assert_eq!(attr.name.local_name, "name");
        assert_eq!(attr.name.namespace, Option::None);
        assert_eq!(attr.name.prefix, Option::None);
        assert_eq!(attr.value, "MemoryMap");
    } else {
        panic!("Expected XmlEvent::StartElement!")
    }

    check_end_element_event(parser.next().unwrap(), "section");

    check_end_document_event(parser.next().unwrap());
}
