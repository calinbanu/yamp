use parser::section::Section;
use std::str::FromStr;

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
