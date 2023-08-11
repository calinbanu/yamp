use log::error;
use rust_xlsxwriter::*;

use crate::{entry::Entry, object::Object, segment::Segment};

pub trait ToExcelWriter {
    fn to_excel_writer<'a, 'b>(&'a self, writer: &mut ExcelWriter<'b>)
    where
        'a: 'b;
}

pub struct ExcelWriter<'a> {
    wb: Workbook,
    file: String,
    current_segment: Option<&'a Segment>,
    segment_count: u32,
    entry_count: u32,
    obj_count: u32,
}

impl<'a> ExcelWriter<'a> {
    fn write_segment_header(ws: &mut Worksheet, format: &Format) -> Result<(), XlsxError> {
        ws.write_with_format(0, 0, "Nr", format)?;
        ws.write_with_format(0, 1, "Segment", format)?;
        ws.write_with_format(0, 2, "Address", format)?;
        ws.write_with_format(0, 3, "Size", format)?;
        Ok(())
    }

    fn write_entry_header(ws: &mut Worksheet, format: &Format) -> Result<(), XlsxError> {
        ws.write_with_format(0, 0, "Nr", format)?;
        ws.write_with_format(0, 1, "Segment", format)?;
        ws.write_with_format(0, 2, "Entry", format)?;
        ws.write_with_format(0, 3, "Address", format)?;
        ws.write_with_format(0, 4, "Size", format)?;
        Ok(())
    }

    fn write_object_header(ws: &mut Worksheet, format: &Format) -> Result<(), XlsxError> {
        ws.write_with_format(0, 0, "Nr", format)?;
        ws.write_with_format(0, 1, "Object", format)?;
        ws.write_with_format(0, 2, "Segment", format)?;
        ws.write_with_format(0, 3, "Size", format)?;
        Ok(())
    }

    pub fn new(file: &str) -> Result<Self, XlsxError> {
        let mut wb = Workbook::new();

        let header_format = Format::new().set_align(FormatAlign::Left);

        let segment_ws = wb.add_worksheet();
        segment_ws.set_name("Segments")?;
        Self::write_segment_header(segment_ws, &header_format)?;

        let entry_ws = wb.add_worksheet();
        entry_ws.set_name("Entries")?;
        Self::write_entry_header(entry_ws, &header_format)?;

        let obj_ws = wb.add_worksheet();
        obj_ws.set_name("Objects")?;
        Self::write_object_header(obj_ws, &header_format)?;

        Ok(Self {
            wb,
            file: file.to_string(),
            current_segment: None,
            segment_count: 0,
            entry_count: 0,
            obj_count: 0,
        })
    }

    pub fn write_segment(&mut self, segment: &'a Segment) {
        let segment_ws = self.wb.worksheet_from_name("Segments").unwrap();

        let row = self.segment_count + 1;
        segment_ws.write(row, 0, self.segment_count as f64).unwrap();
        segment_ws.write(row, 1, segment.get_name()).unwrap();
        if let Some(address) = segment.get_address() {
            let s_address = format!("{:#016x}", address);
            segment_ws.write(row, 2, &s_address).unwrap();
            segment_ws
                .write(row, 3, segment.get_size().unwrap() as f64)
                .unwrap();
        }
        self.segment_count += 1;

        self.current_segment = Some(segment);
    }

    pub fn write_entry(&mut self, entries: &Entry) {
        let entry_ws = self.wb.worksheet_from_name("Entries").unwrap();

        let row = self.entry_count + 1;
        entry_ws.write(row, 0, self.entry_count as f64).unwrap();
        if let Some(segment) = self.current_segment {
            entry_ws.write(row, 1, segment.get_name()).unwrap();
        } else {
            error!("Invalid segment while writing to xlsx!");
        }
        entry_ws.write(row, 2, entries.get_name()).unwrap();
        let s_address = format!("{:#016x}", entries.get_address());
        entry_ws.write(row, 3, &s_address).unwrap();
        entry_ws.write(row, 4, entries.get_size() as f64).unwrap();
        self.entry_count += 1;
    }

    pub fn write_object(&mut self, object: &Object) {
        let obj_ws = self.wb.worksheet_from_name("Objects").unwrap();

        for name in object.get_all_segments() {
            let size = object.get_segment_size(name);
            let row = self.obj_count + 1;
            obj_ws.write(row, 0, self.obj_count as f64).unwrap();
            obj_ws.write(row, 1, object.get_name()).unwrap();
            obj_ws.write(row, 2, name).unwrap();
            if let Some(size) = size {
                obj_ws.write(row, 3, size as f64).unwrap();
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
        self.wb.save(&self.file).unwrap();
    }
}
