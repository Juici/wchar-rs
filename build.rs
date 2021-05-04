fn main() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");

    let wchar = get_platform_wchar();
    let value = match wchar {
        WChar::U16 => "u16",
        WChar::U32 => "u32",
        WChar::I32 => "i32",
        WChar::Unknown => {
            println!("cargo:warning=platform wchar_t unknown");

            "unknown"
        }
    };
    println!("cargo:rustc-cfg=wchar_t=\"{}\"", value);
}

enum WChar {
    U16,
    U32,
    I32,
    Unknown,
}

mod cfg {
    use std::env;

    pub fn is_unix() -> bool {
        env::var_os("CARGO_CFG_UNIX").is_some()
    }

    pub fn is_windows() -> bool {
        env::var_os("CARGO_CFG_WINDOWS").is_some()
    }

    pub fn target_os() -> String {
        env::var("CARGO_CFG_TARGET_OS").unwrap()
    }

    pub fn target_arch() -> String {
        env::var("CARGO_CFG_TARGET_ARCH").unwrap()
    }

    pub fn target_env() -> String {
        env::var("CARGO_CFG_TARGET_ENV").unwrap()
    }

    pub fn target_vendor() -> String {
        env::var("CARGO_CFG_TARGET_VENDOR").unwrap()
    }
}

fn get_platform_wchar() -> WChar {
    if cfg::is_windows() {
        return WChar::U16;
    }

    let target_os = cfg::target_os();
    match &target_os[..] {
        "fuchsia" => return get_fuchsia_wchar(),
        "switch" => return WChar::U32,
        "psp" => return WChar::Unknown, // TODO: Can't find info on this in libc.
        "vxworks" => return get_vxworks_wchar(),
        _ => {}
    }

    if cfg::is_unix() {
        return get_unix_wchar();
    }

    if target_os == "hermit" {
        let target_arch = cfg::target_arch();
        return match &target_arch[..] {
            "aarch64" => WChar::U32,
            "x86_64" => WChar::I32,
            _ => WChar::Unknown,
        };
    }

    let target_env = cfg::target_env();
    let target_vendor = cfg::target_vendor();
    if target_env == "sgx" || target_vendor == "fortanix" {
        WChar::Unknown // TODO: Can't find info on this in libc.
    } else if target_env == "wasi" || target_os == "wasi" {
        WChar::I32
    } else {
        WChar::Unknown
    }
}

fn get_fuchsia_wchar() -> WChar {
    let target_arch = cfg::target_arch();
    match &target_arch[..] {
        "aarch64" => WChar::U32,
        "x86_64" => WChar::I32,
        _ => WChar::Unknown,
    }
}

fn get_vxworks_wchar() -> WChar {
    let target_arch = cfg::target_arch();
    match &target_arch[..] {
        "aarch64" => WChar::U32,
        "arm" => WChar::U32,
        "x86" => WChar::I32,
        "x86_64" => WChar::I32,
        "powerpc" => WChar::U32,
        "powerpc64" => WChar::U32,
        _ => WChar::Unknown,
    }
}

fn get_unix_wchar() -> WChar {
    let target_arch = cfg::target_arch();
    let target_env = cfg::target_env();
    if target_env == "new_lib" {
        return match &target_arch[..] {
            "arm" => WChar::U32,
            "aarch64" => WChar::U32,
            "xtensa" => WChar::U32,
            "powerpc" => WChar::I32, // libc::c_int
            _ => WChar::Unknown,
        };
    }

    let target_os = cfg::target_os();
    match &target_os[..] {
        "emscripten" => WChar::I32,
        "linux" | "l4re" => match &target_env[..] {
            "uclibc" => match &target_arch[..] {
                "mips" | "mips64" => WChar::I32,
                "x86_64" => WChar::I32, // libc::c_int
                "arm" => WChar::U32,    // libc::c_uint
                _ => WChar::Unknown,
            },
            "musl" => match &target_arch[..] {
                "x86" | "x86_64" => WChar::I32,
                "aarch64" => WChar::U32,
                "mips64" => WChar::I32,
                "mips" => WChar::I32, // libc::c_int
                "powerpc" | "powerpc64" => WChar::I32,
                "s390x" => WChar::I32,
                "hexagon" => WChar::U32,
                "arm" => WChar::U32,
                _ => WChar::Unknown,
            },
            "gnu" => match &target_arch[..] {
                "x86" | "x86_64" => WChar::I32,
                "arm" => WChar::U32,
                "mips" | "mips64" => WChar::I32,
                "powerpc" | "powerpc64" => WChar::I32,
                "sparc" | "sparc64" => WChar::I32,
                "riscv32" | "riscv64" => WChar::I32, // libc::c_int
                "aarch64" => WChar::U32,
                "s390x" => WChar::I32,
                _ => WChar::Unknown,
            },
            _ => WChar::Unknown,
        },
        "android" => match &target_arch[..] {
            "x86" | "x86_64" => WChar::I32,
            "arm" => WChar::U32,
            "aarch64" => WChar::U32,
            _ => WChar::Unknown,
        },

        "macos" | "ios" => WChar::I32,
        "openbsd" | "netbsd" => WChar::I32,
        "dragonfly" => WChar::I32,
        "freebsd" => match &target_arch[..] {
            "x86" | "x86_64" => WChar::I32,
            "aarch64" => WChar::U32,
            "arm" => WChar::U32,
            "powerpc64" => WChar::I32,
            _ => WChar::Unknown,
        },

        "solaris" | "illumos" => WChar::I32, // libc::c_int,

        "haiku" => WChar::I32,

        "hermit" => match &target_arch[..] {
            "aarch64" => WChar::U32,
            "x86_64" => WChar::I32,
            _ => WChar::Unknown,
        },

        "redox" => WChar::I32,

        _ => WChar::Unknown,
    }
}
