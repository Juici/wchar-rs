use wchar::wch_c;

fn main() {
    wch_c!("also fails \0");
}
