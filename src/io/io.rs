use std::fmt::Write;

fn main() {
    io::prelude();
    let mut w = String::new();
    let tc: usize = arg();
    for _ in 0..tc {
        let x: u16 = arg();
        let y: u16 = arg();
        let _ = writeln!(w, "{}", x + y);
    }
    print!("{w}");
}

#[allow(unused)]
mod io {
    use std::convert::AsRef;
    use std::fmt::Debug;
    use std::mem::MaybeUninit;
    use std::str::from_utf8_unchecked;
    use std::str::FromStr;

    static mut GLOBAL_INPUT: MaybeUninit<Tokenizer<'static>> = MaybeUninit::uninit();

    pub fn prelude() {
        unsafe {
            GLOBAL_INPUT = MaybeUninit::new(process_input());
        }
    }

    pub trait FromTokenizer<'a, P> {
        fn parse(input: &mut Tokenizer<'a>, par: P) -> Self;
    }

    #[inline(always)]
    pub fn arg<T>() -> T
    where
        T: FromTokenizer<'static, ()>,
    {
        parse(())
    }

    #[inline(always)]
    pub fn parse<T, P>(par: P) -> T
    where
        T: FromTokenizer<'static, P>,
    {
        unsafe { GLOBAL_INPUT.assume_init_mut() }.parse(par)
    }

    #[cfg(target_family = "windows")]
    fn process_input<'a>() -> Tokenizer<'a> {
        use std::io::{self, Read};
        let mut stdin = io::stdin();
        let mut input = Vec::new();
        let _ = stdin.read_to_end(&mut input);
        Tokenizer::new(input.leak())
    }

    #[cfg(target_family = "unix")]
    fn process_input<'a>() -> Tokenizer<'a> {
        use std::ffi::c_void;
        use std::mem::MaybeUninit;
        use std::ptr::null_mut;
        use std::slice::from_raw_parts;

        const STDIN_FILENO: i32 = 0;
        const PROT_READ: i32 = 0x1;
        const MAP_PRIVATE: i32 = 0x02;
        const NULL: *mut c_void = null_mut();
        const MAP_FAILED: *mut c_void = usize::MAX as _;

        #[repr(C)]
        struct stat64 {
            st_dev: u64,
            st_ino: u64,
            st_nlink: u64,
            st_mode: u32,
            st_uid: u32,
            st_gid: u32,
            __pad0: i32,
            st_rdev: u64,
            st_size: i64,
            st_blksize: i64,
            st_blocks: i64,
            st_atime: i64,
            st_atime_nsec: i64,
            st_mtime: i64,
            st_mtime_nsec: i64,
            st_ctime: i64,
            st_ctime_nsec: i64,
            __reserved: [i64; 3],
        }

        #[link(name = "c", kind = "dylib")]
        extern "C" {
            fn fstat64(fd: i32, buf: *mut stat64) -> i32;
            fn mmap64(
                address: *mut c_void,
                len: usize,
                protect: i32,
                flags: i32,
                fd: i32,
                offset: i64,
            ) -> *mut c_void;
        }

        let input = unsafe {
            let mut buf = MaybeUninit::uninit();
            assert_eq!(fstat64(STDIN_FILENO, buf.as_mut_ptr()), 0);
            let len = buf.assume_init().st_size as usize;
            let address = mmap64(NULL, len, PROT_READ, MAP_PRIVATE, STDIN_FILENO, 0);
            assert!(address != MAP_FAILED);
            from_raw_parts(address.cast(), len)
        };
        Tokenizer::new(input)
    }

    pub struct Tokenizer<'a> {
        pub bytes: &'a [u8],
        pub pos: usize,
    }

    impl<'a> Tokenizer<'a> {
        const fn new(bytes: &'a [u8]) -> Self {
            Self { bytes, pos: 0 }
        }

        const fn state(&self) -> (&'a [u8], usize) {
            (self.bytes, self.pos)
        }

        pub fn arg<T>(&mut self) -> T
        where
            T: FromTokenizer<'a, ()>,
        {
            self.parse(())
        }

        pub fn parse<T, P>(&mut self, par: P) -> T
        where
            T: FromTokenizer<'a, P>,
        {
            T::parse(self, par)
        }
    }

    pub struct Whitespaces;

    impl<'a> FromTokenizer<'a, ()> for Whitespaces {
        fn parse(input: &mut Tokenizer<'a>, _: ()) -> Self {
            let (bytes, mut pos) = input.state();
            while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            input.pos = pos;
            Self
        }
    }

    pub struct Lit;

    impl<'a, T> FromTokenizer<'a, T> for Lit
    where
        T: AsRef<[u8]>,
    {
        fn parse(input: &mut Tokenizer<'a>, lit: T) -> Self {
            let (bytes, mut pos) = input.state();
            let lit = lit.as_ref();
            let mut i = 0;
            while pos < bytes.len() && i < lit.len() && bytes[pos] == lit[i] {
                pos += 1;
                i += 1;
            }
            assert!(i >= lit.len(), "UNMATCHED LITERAL");
            input.pos = pos;
            Self
        }
    }

    impl<'a, T> FromTokenizer<'a, usize> for Vec<T>
    where
        T: FromTokenizer<'a, ()>,
    {
        fn parse(input: &mut Tokenizer<'a>, len: usize) -> Self {
            (0..len).map(|_| input.arg()).collect()
        }
    }

    #[derive(Clone, Copy)]
    pub enum Delimiter {
        Line,
        Word,
    }

    impl Delimiter {
        const fn terminates(self, byte: u8) -> bool {
            match self {
                Delimiter::Line => byte == b'\r' || byte == b'\n',
                Delimiter::Word => byte.is_ascii_whitespace(),
            }
        }
    }

    impl<'a> FromTokenizer<'a, Delimiter> for &'a [u8] {
        fn parse(input: &mut Tokenizer<'a>, lim: Delimiter) -> Self {
            let _: Whitespaces = input.arg();
            let (bytes, pos) = input.state();
            let mut i = pos;
            while i < bytes.len() && !lim.terminates(bytes[i]) {
                i += 1;
            }
            input.pos = i;
            &bytes[pos..i]
        }
    }

    impl<'a> FromTokenizer<'a, Delimiter> for &'a str {
        fn parse(input: &mut Tokenizer<'a>, lim: Delimiter) -> Self {
            let raw: &'a [u8] = input.parse(lim);
            unsafe { from_utf8_unchecked(raw) }
        }
    }

    impl<'a> FromTokenizer<'a, Delimiter> for Vec<u8> {
        fn parse(input: &mut Tokenizer<'a>, lim: Delimiter) -> Self {
            let raw: &'a [u8] = input.parse(lim);
            raw.to_vec()
        }
    }

    #[repr(transparent)]
    struct ParseStr<T>(T);

    impl<'a, T> FromTokenizer<'a, Delimiter> for ParseStr<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        fn parse(input: &mut Tokenizer<'a>, lim: Delimiter) -> Self {
            let src: &'a str = input.parse(lim);
            Self(src.parse().unwrap())
        }
    }

    impl<'a> FromTokenizer<'a, ()> for f64 {
        fn parse(input: &mut Tokenizer<'a>, _: ()) -> Self {
            let ParseStr(n) = input.parse(Delimiter::Word);
            n
        }
    }

    macro_rules! parse_int {
        { $( $Ty:ty )* } => { $(
            impl<'a> FromTokenizer<'a, ()> for $Ty {
                fn parse(input: &mut Tokenizer<'a>, _: ()) -> Self {
                    const SIGNEDNESS: bool = <$Ty>::MIN != 0;
                    let _: Whitespaces = input.arg();
                    let (bytes, mut pos) = input.state();
                    let mut x = 0;
                    if SIGNEDNESS && pos < bytes.len() && bytes[pos] == b'-' {
                        pos += 1;
                        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
                            x = 10 * x - (bytes[pos] & 0xf) as $Ty;
                            pos += 1;
                        }
                    } else {
                        if SIGNEDNESS && pos < bytes.len() && bytes[pos] == b'+' {
                            pos += 1;
                        }
                        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
                            x = 10 * x + (bytes[pos] & 0xf) as $Ty;
                            pos += 1;
                        }
                    }
                    input.pos = pos;
                    x
                }
            }
        )* };
    }
    parse_int! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
}

#[allow(unused)]
use crate::io::{arg, parse};
