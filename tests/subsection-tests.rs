use parser::subsection::SubSection;
use std::str::FromStr;

// struct LoggerDb<'a, 'b> {
//     db: Arc<Mutex<Vec<&'a Record<'b>>>>,
// }
// impl<'a, 'b> LoggerDb<'a, 'b> {
//     fn add_log(&self, record: &'a Record<'b>) {
//         self.db.clone()
//     }
// }
// struct Logger;
// impl log::Log for Logger {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         true
//     }
//     fn log(&self, record: &Record) {
//         println!("{:#?}", record);
//     }
//     fn flush(&self) {}
// }
// static LOGGER: Logger = Logger;
// log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));

#[test]
fn single_line() {
    // valid
    const DATA_0: &str =
        concat!(" .bss.test  0x00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n");
    let subsection = match SubSection::parse(DATA_0) {
        Ok(ss) => ss,
        Err(_) => panic!(),
    };
    assert_eq!(subsection.name(), ".bss.test");
    assert_eq!(subsection.address(), 0x00000000DEADBEEF);
    assert_eq!(subsection.size(), 0xFF);

    // valid without obj
    // TODO(calin) TBD

    // missing space before subsection name
    const DATA_1: &str =
        concat!(".bss.test  0x00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n");
    assert!(SubSection::parse(DATA_1).is_err());

    // missing lib/obj
    const DATA_2: &str = concat!(" .bss.test  0x00000000DEADBEEF       0xFF\n");
    assert!(SubSection::parse(DATA_2).is_err());

    // missing size
    const DATA_3: &str =
        concat!(" .bss.test  0x00000000DEADBEEF        /path/to/lib/libtest.a(test.cc.obj)\n");
    assert!(SubSection::parse(DATA_3).is_err());

    // mssing address
    const DATA_4: &str = concat!(" .bss.test         0xFF /path/to/lib/libtest.a(test.cc.obj)\n");
    assert!(SubSection::parse(DATA_4).is_err());

    // mssing name
    const DATA_5: &str = concat!("          0xFF /path/to/lib/libtest.a(test.cc.obj)\n");
    assert!(SubSection::parse(DATA_5).is_err());

    // invalid address format
    const DATA_6: &str =
        concat!(" .bss.test  00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n");
    assert!(SubSection::parse(DATA_6).is_err());

    // invalid address format
    const DATA_7: &str =
        concat!(" .bss.test  0x00000000DEADBEEF       FF /path/to/lib/libtest.a(test.cc.obj)\n");
    assert!(SubSection::parse(DATA_7).is_err());
}

#[test]
fn single_line_overlapping_fill() {
    // valid
    const DATA_0: &str = concat!(
        " .bss.test      0x000000000002261e       0x4 /path/to/lib/libtest.a(test.cc.obj)\n",
        " *fill*         0x000000000002261e       0x2\n"
    );
    let subsection = match SubSection::parse(DATA_0) {
        Ok(ss) => ss,
        Err(_) => panic!(),
    };
    assert_eq!(subsection.name(), ".bss.test");
    assert_eq!(subsection.address(), 0x000000000002261e);
    assert_eq!(subsection.size(), 0x2);
}

#[test]
fn two_line() {
    const DATA_0: &str = concat!(
        " .bss.test",
        "                0x00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n"
    );
    let subsection = match SubSection::parse(DATA_0) {
        Ok(ss) => ss,
        Err(_) => panic!(),
    };
    assert_eq!(subsection.name(), ".bss.test");
    assert_eq!(subsection.address(), 0x00000000DEADBEEF);
    assert_eq!(subsection.size(), 0xFF);

    // valid without obj
    // TODO(calin) TBD

    // missing space before subsection name
    const DATA_1: &str = concat!(
        ".bss.test",
        "                0x00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n"
    );
    assert!(SubSection::parse(DATA_1).is_err());

    // missing lib/obj
    const DATA_2: &str = concat!(
        " .bss.test",
        "                0x00000000DEADBEEF       0xFF \n"
    );
    assert!(SubSection::parse(DATA_2).is_err());

    // missing size
    const DATA_3: &str = concat!(
        " .bss.test",
        "                0x00000000DEADBEEF        /path/to/lib/libtest.a(test.cc.obj)\n"
    );
    assert!(SubSection::parse(DATA_3).is_err());

    // mssing address
    const DATA_4: &str = concat!(
        " .bss.test",
        "                       0xFF /path/to/lib/libtest.a(test.cc.obj)\n"
    );
    assert!(SubSection::parse(DATA_4).is_err());

    // mssing name
    const DATA_5: &str = concat!(
        "",
        "                0x00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n"
    );
    assert!(SubSection::parse(DATA_5).is_err());

    // invalid address format
    const DATA_6: &str = concat!(
        " .bss.test",
        "                00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n"
    );
    assert!(SubSection::parse(DATA_6).is_err());

    // invalid address format
    const DATA_7: &str = concat!(
        " .bss.test",
        "                0x00000000DEADBEEF       FF /path/to/lib/libtest.a(test.cc.obj)\n"
    );
    assert!(SubSection::parse(DATA_7).is_err());
}

#[test]
fn two_line_w_overlapping_fill() {
    const DATA_0: &str = concat!(
        " .bss.test",
        "                0x00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n",
        " *fill*         0x00000000DEADBEEF       0x01\n"
    );

    assert!(SubSection::parse(DATA_0).is_ok());

    const DATA_1: &str = concat!(
        " .bss.test",
        "                0x00000000DEADBEEF       0xFF /path/to/lib/libtest.a(test.cc.obj)\n",
        "                                         0x70 (size before relaxing)",
        " *fill*         0x00000000DEADBEEF       0x01\n"
    );

    assert!(SubSection::parse(DATA_1).is_ok());
}

#[test]
fn new() {
    let name = "test".to_string();
    let address = 0;
    let size = 0;

    let section = SubSection::new(name.clone(), address, size);

    assert_eq!(section.name(), name);
    assert_eq!(section.address(), address);
    assert_eq!(section.size(), size);
}

#[test]
fn new_size_non_overlaping() {
    let address = 0;
    let size = 1;
    let fill_address = 1;
    let fill_size = 2;

    let mut section = SubSection::new("test".to_string(), address, size);
    section.set_fill(fill_address, fill_size);

    assert_eq!(section.size(), size + fill_size)
}

#[test]
fn new_size_overlaping() {
    let address = 0;
    let size = 1;
    let fill_address = 0;
    let fill_size = 2;

    let mut section = SubSection::new("test".to_string(), address, size);
    section.set_fill(fill_address, fill_size);

    assert_eq!(section.size(), fill_size)
}
