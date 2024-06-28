use std::collections::HashMap;

macro_rules! consts {
    ($($arch:ident),* $(,)?) => {
        $(
            #[allow(non_upper_case_globals, dead_code)]
            pub const $arch: &str = stringify!($arch);
        )*
    };
}

mod arch {
    consts! {
        aarch64,
        arm,
        hexagon,
        mips,
        mips64,
        powerpc,
        powerpc64,
        riscv32,
        riscv64,
        sparc,
        sparc64,
        s390x,
        xtensa,
        x86,
        x86_64,
    }
}

mod os {
    consts! {
        android,
        dragonfly,
        emscripten,
        freebsd,
        fushia,
        haiku,
        hermit,
        illumos,
        ios,
        linux,
        l4re,
        macos,
        netbsd,
        openbsd,
        psp,
        redox,
        solaris,
        switch,
        vxworks,
        wasi,
        windows,
    }
}

mod family {
    consts! {
        unix,
        windows,
        wasm,
    }
}

mod env {
    consts! {
        gnu,
        musl,
        mvsc,
        new_lib,
        sgx,
        uclibc,
        wasi,
    }
}

mod vendor {
    consts! {
        fortanix,
    }
}

struct Cfg {
    vars: HashMap<String, String>,
}

impl Cfg {
    fn from_env() -> Cfg {
        let vars: HashMap<String, String> = std::env::vars()
            .filter(|(key, _)| key.starts_with("CARGO_CFG"))
            .collect();

        Cfg { vars }
    }

    fn cfg(&self, cfg: &str) -> Option<&str> {
        let mut cfg = format!("CARGO_CFG_{}", cfg);
        cfg["CARGO_CFG_".len()..].make_ascii_uppercase();

        self.vars.get(&cfg).map(|s| s.as_str())
    }

    fn unix(&self) -> bool {
        self.family().any(|family| family::unix == family)
    }

    fn windows(&self) -> bool {
        self.family().any(|family| family::windows == family)
    }

    fn family(&self) -> impl Iterator<Item = &str> {
        self.cfg("target_family").unwrap_or_default().split(',')
    }

    fn os(&self) -> Option<&str> {
        self.cfg("target_os")
    }

    fn arch(&self) -> Option<&str> {
        self.cfg("target_arch")
    }

    fn vendor(&self) -> Option<&str> {
        self.cfg("target_vendor")
    }

    fn env(&self) -> Option<&str> {
        self.cfg("target_env")
    }

    // fn pointer_width(&self) -> Option<&str> {
    //     self.cfg("target_pointer_width")
    // }
    //
    // fn endian(&self) -> Option<&str> {
    //     self.cfg("target_endian")
    // }
    //
    // fn feature(&self, value: &str) -> bool {
    //     self.features().any(|feature| feature == value)
    // }
    //
    // fn features(&self) -> impl Iterator<Item = &str> {
    //     self.cfg("target_feature").unwrap_or_default().split(',')
    // }
}

enum WChar {
    U16,
    U32,
    I32,
    Unknown,
}

fn main() {
    let wchar_t = match get_platform_wchar() {
        WChar::U16 => Some("u16"),
        WChar::U32 => Some("u32"),
        WChar::I32 => Some("i32"),
        WChar::Unknown => None,
    };

    match wchar_t {
        Some(wchar_t) => println!("cargo:rustc-cfg=wchar_t=\"{}\"", wchar_t),
        None => println!("cargo:warning=unknown platform wchar_t"),
    }
}

fn get_platform_wchar() -> WChar {
    let cfg = Cfg::from_env();

    if cfg.windows() {
        return WChar::U16;
    }

    match cfg.os() {
        Some(os::fushia) => match cfg.arch() {
            Some(arch::aarch64) => WChar::U32,
            Some(arch::x86_64) => WChar::I32,
            _ => WChar::Unknown,
        },
        Some(os::switch) => WChar::U32,
        Some(os::psp) => WChar::Unknown, // TODO: No info in libc.
        Some(os::vxworks) => match cfg.arch() {
            Some(arch::aarch64) => WChar::U32,
            Some(arch::arm) => WChar::U32,
            Some(arch::x86 | arch::x86_64) => WChar::I32,
            Some(arch::powerpc | arch::powerpc64) => WChar::U32,
            _ => WChar::Unknown,
        },
        os if cfg.unix() => get_unix_wchar(&cfg, os),
        Some(os::hermit) => get_hermit_wchar(&cfg),
        os => {
            let env = cfg.env();
            if env == Some(env::sgx) || cfg.vendor() == Some(vendor::fortanix) {
                WChar::Unknown // TODO: No info in libc.
            } else if env == Some(env::wasi) || os == Some(os::wasi) {
                WChar::I32
            } else {
                WChar::Unknown
            }
        }
    }
}

fn get_unix_wchar(cfg: &Cfg, os: Option<&str>) -> WChar {
    #[allow(non_upper_case_globals)]
    const c_int: WChar = WChar::I32;
    #[allow(non_upper_case_globals)]
    const c_uint: WChar = WChar::U32;

    let env = cfg.env();

    if let Some(env::new_lib) = env {
        return match cfg.arch() {
            Some(arch::aarch64) => WChar::U32,
            Some(arch::arm) => WChar::U32,
            Some(arch::powerpc) => c_int,
            Some(arch::xtensa) => WChar::U32,
            _ => WChar::Unknown,
        };
    }

    match os {
        Some(os::emscripten) => WChar::I32,
        Some(os::linux | os::l4re) => match env {
            Some(env::uclibc) => match cfg.arch() {
                Some(arch::arm) => c_uint,
                Some(arch::mips | arch::mips64) => WChar::I32,
                Some(arch::x86_64) => c_int,
                _ => WChar::Unknown,
            },
            Some(env::musl) => match cfg.arch() {
                Some(arch::aarch64) => WChar::U32,
                Some(arch::arm) => WChar::U32,
                Some(arch::hexagon) => WChar::U32,
                Some(arch::mips) => c_int,
                Some(arch::mips64) => WChar::I32,
                Some(arch::powerpc | arch::powerpc64) => WChar::I32,
                Some(arch::s390x) => WChar::I32,
                Some(arch::x86 | arch::x86_64) => WChar::I32,
                _ => WChar::Unknown,
            },
            Some(env::gnu) => match cfg.arch() {
                Some(arch::aarch64) => WChar::U32,
                Some(arch::arm) => WChar::U32,
                Some(arch::mips | arch::mips64) => WChar::I32,
                Some(arch::powerpc | arch::powerpc64) => WChar::I32,
                Some(arch::riscv32 | arch::riscv64) => c_int,
                Some(arch::sparc | arch::sparc64) => WChar::I32,
                Some(arch::s390x) => WChar::I32,
                Some(arch::x86 | arch::x86_64) => WChar::I32,
                _ => WChar::Unknown,
            },
            _ => WChar::Unknown,
        },
        Some(os::android) => match cfg.arch() {
            Some(arch::aarch64) => WChar::U32,
            Some(arch::arm) => WChar::U32,
            Some(arch::x86 | arch::x86_64) => WChar::I32,
            _ => WChar::Unknown,
        },

        Some(os::ios | os::macos) => WChar::I32,
        Some(os::openbsd | os::netbsd) => WChar::I32,
        Some(os::dragonfly) => WChar::I32,
        Some(os::freebsd) => match cfg.arch() {
            Some(arch::aarch64) => WChar::U32,
            Some(arch::arm) => WChar::U32,
            Some(arch::powerpc64) => WChar::I32,
            Some(arch::x86 | arch::x86_64) => WChar::I32,
            _ => WChar::Unknown,
        },

        Some(os::solaris | os::illumos) => c_int,

        Some(os::haiku) => WChar::I32,

        Some(os::hermit) => get_hermit_wchar(cfg),

        Some(os::redox) => WChar::I32,

        _ => WChar::Unknown,
    }
}

fn get_hermit_wchar(cfg: &Cfg) -> WChar {
    match cfg.arch() {
        Some(arch::aarch64) => WChar::U32,
        Some(arch::x86_64) => WChar::I32,
        _ => WChar::Unknown,
    }
}
