use wchar::wch_c;

fn main() {
    wch_c!("fails \0 to compile");
}
