use wchar::{include_wch, wchar_t};

const MISSING_FILE: &[wchar_t] = include_wch!("missing_file");

fn main() {}
