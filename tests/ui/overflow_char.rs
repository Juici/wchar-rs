use wchar::wch;

const OVERFLOW_U16: u16 = wch!(u16, '💖');
const OVERFLOW_I16: i16 = wch!(i16, '💖');

fn main() {}
