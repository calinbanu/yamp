use log::error;
use xlsxwriter::{prelude::FormatAlignment, Format, Workbook, Worksheet, XlsxError};

use crate::{entry::Entry, object::Object, segment::Segment};

pub trait ToExcelWriter {
    fn to_excel_writer<'a, 'b>(&'a self, writer: &mut ExcelWriter<'b>)
    where
        'a: 'b;
}

pub struct ExcelWriter<'a> {
    wb: Option<Workbook>,
    current_segment: Option<&'a Segment>,
    segment_count: u32,
    entry_count: u32,
    obj_count: u32,
}

impl<'a> ExcelWriter<'a> {
    fn write_segment_header(ws: &mut Worksheet, format: &Format) -> Result<(), XlsxError> {
        ws.write_string(0, 0, "Nr", Some(format))?;
        ws.write_string(0, 1, "Segment", Some(format))?;
        ws.write_string(0, 2, "Address", Some(format))?;
        ws.write_string(0, 3, "Size", Some(format))?;
        Ok(())
    }

    fn write_entry_header(ws: &mut Worksheet, format: &Format) -> Result<(), XlsxError> {
        ws.write_string(0, 0, "Nr", Some(format))?;
        ws.write_string(0, 1, "Segment", Some(format))?;
        ws.write_string(0, 2, "Entry", Some(format))?;
        ws.write_string(0, 3, "Address", Some(format))?;
        ws.write_string(0, 4, "Size", Some(format))?;
        Ok(())
    }

    fn write_object_header(ws: &mut Worksheet, format: &Format) -> Result<(), XlsxError> {
        ws.write_string(0, 0, "Nr", Some(format))?;
        ws.write_string(0, 1, "Object", Some(format))?;
        ws.write_string(0, 2, "Segment", Some(format))?;
        ws.write_string(0, 3, "Size", Some(format))?;
        Ok(())
    }

    pub fn new(file: &str) -> Result<Self, XlsxError> {
        let wb: Workbook = Workbook::new(file)?;
        let mut header_format = Format::new();
        header_format.set_align(FormatAlignment::Left);

        let mut segment_ws = wb.add_worksheet(Some("Segments"))?;
        Self::write_segment_header(&mut segment_ws, &header_format)?;

        let mut entry_ws = wb.add_worksheet(Some("Entries"))?;
        Self::write_entry_header(&mut entry_ws, &header_format)?;

        let mut obj_ws = wb.add_worksheet(Some("Objects"))?;
        Self::write_object_header(&mut obj_ws, &header_format)?;

        Ok(Self {
            wb: Some(wb),
            current_segment: None,
            segment_count: 0,
            entry_count: 0,
            obj_count: 0,
        })
    }

    pub fn write_segment(&mut self, segment: &'a Segment) {
        let mut segment_ws = self
            .wb
            .as_ref()
            .unwrap()
            .get_worksheet("Segments")
            .unwrap()
            .unwrap();

        let row = self.segment_count + 1;
        segment_ws
            .write_number(row, 0, self.segment_count as f64, None)
            .unwrap();
        segment_ws
            .write_string(row, 1, segment.get_name(), None)
            .unwrap();
        if let Some(address) = segment.get_address() {
            let s_address = format!("{:#016x}", address);
            segment_ws.write_string(row, 2, &s_address, None).unwrap();
            segment_ws
                .write_number(row, 3, segment.get_size().unwrap() as f64, None)
                .unwrap();
        }
        self.segment_count += 1;

        self.current_segment = Some(segment);
    }

    pub fn write_entry(&mut self, entries: &Entry) {
        let mut entry_ws = self
            .wb
            .as_ref()
            .unwrap()
            .get_worksheet("Entries")
            .unwrap()
            .unwrap();

        let row = self.entry_count + 1;
        entry_ws
            .write_number(row, 0, self.entry_count as f64, None)
            .unwrap();
        if let Some(segment) = self.current_segment {
            entry_ws
                .write_string(row, 1, segment.get_name(), None)
                .unwrap();
        } else {
            error!("Invalid segment while writing to xlsx!");
        }
        entry_ws
            .write_string(row, 2, entries.get_name(), None)
            .unwrap();
        let s_address = format!("{:#016x}", entries.get_address());
        entry_ws.write_string(row, 3, &s_address, None).unwrap();
        entry_ws
            .write_number(row, 4, entries.get_size() as f64, None)
            .unwrap();
        self.entry_count += 1;
    }

    pub fn write_object(&mut self, object: &Object) {
        let mut obj_ws = self
            .wb
            .as_ref()
            .unwrap()
            .get_worksheet("Objects")
            .unwrap()
            .unwrap();

        for name in object.get_all_segments() {
            let size = object.get_segment_size(name);
            let row = self.obj_count + 1;
            obj_ws
                .write_number(row, 0, self.obj_count as f64, None)
                .unwrap();
            obj_ws
                .write_string(row, 1, object.get_name(), None)
                .unwrap();
            obj_ws.write_string(row, 2, name, None).unwrap();
            if let Some(size) = size {
                obj_ws.write_number(row, 3, size as f64, None).unwrap();
            } else {
                error!(
                    "Error occured while trying to write object size: {}",
                    object.get_name()
                );
            }
            self.obj_count += 1;
        }
    }
}

impl<'a> Drop for ExcelWriter<'a> {
    fn drop(&mut self) {
        self.wb.take().unwrap().close().unwrap();
    }
}
