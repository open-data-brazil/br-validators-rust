use std::io::Cursor;

use calamine::{open_workbook_from_rs, Reader, Xlsx};
use chrono::{Days, NaiveDate};

use super::{outcome::SourceError, text::clean};

pub fn read_sheet(bytes: &[u8], sheet: Option<&str>) -> Result<Vec<Vec<String>>, SourceError> {
    let mut workbook: Xlsx<_> = open_workbook_from_rs(Cursor::new(bytes)).map_err(xlsx_error)?;
    let range = match sheet {
        Some(name) => workbook.worksheet_range(name),
        None => workbook
            .worksheet_range_at(0)
            .ok_or_else(|| SourceError::Parse("XLSX sem planilhas".into()))?,
    }
    .map_err(xlsx_error)?;
    Ok(range
        .rows()
        .map(|row| row.iter().map(|cell| clean(&cell.to_string())).collect())
        .collect())
}

pub fn excel_serial_to_date(serial: f64) -> Option<NaiveDate> {
    if !serial.is_finite() || serial <= 0.0 {
        return None;
    }
    NaiveDate::from_ymd_opt(1899, 12, 30)?.checked_add_days(Days::new(serial.floor() as u64))
}

fn xlsx_error(error: calamine::XlsxError) -> SourceError {
    SourceError::Parse(format!("XLSX: {error}"))
}
