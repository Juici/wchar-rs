use wchar::wchar_t;

// Assert our wchar_t matches with the libc wchar_t.
const _: fn(wchar_t) -> libc::wchar_t = |x| x;
