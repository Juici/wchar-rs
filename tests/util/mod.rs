use std::convert::TryFrom;
use std::iter::once;

use anyhow::{bail, Result};

pub trait Wide: Copy {
    fn encode_char(c: char) -> Result<Self>;
    fn encode_str(text: &str) -> Vec<Self>;
    fn encode_str_c(text: &str) -> Vec<Self>;
    fn decode_char(c: Self) -> Result<char>;
    fn decode_str<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>;
    fn decode_str_c<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator;
}

impl Wide for u16 {
    fn encode_char(c: char) -> Result<Self> {
        if c.len_utf16() == 1 {
            let mut v = [0];
            c.encode_utf16(&mut v);
            Ok(v[0])
        } else {
            bail!("{:?} does not fit within one UTF-16 character", c)
        }
    }

    fn encode_str(text: &str) -> Vec<Self> {
        text.encode_utf16().collect()
    }

    fn encode_str_c(text: &str) -> Vec<Self> {
        text.encode_utf16().chain(once(0)).collect()
    }

    fn decode_char(c: Self) -> Result<char> {
        match char::decode_utf16([c]).next() {
            Some(r) => Ok(r?),
            None => unreachable!(),
        }
    }

    fn decode_str<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
    {
        let iter = char::decode_utf16(iter);
        let (lower_bound, _) = iter.size_hint();

        let mut s = String::with_capacity(lower_bound);
        for r in iter {
            let c = r?;
            s.push(c);
        }
        Ok(s)
    }

    fn decode_str_c<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        let mut iter = iter.into_iter();

        // Make sure we end in a nul-terminator.
        match iter.next_back() {
            Some(0) => {}
            Some(v) => bail!("invalid terminator for C-style string: {:x}", v),
            None => bail!("missing terminator for C-style string"),
        }

        Self::decode_str(iter)
    }
}

impl Wide for u32 {
    fn encode_char(c: char) -> Result<Self> {
        Ok(c as u32)
    }

    fn encode_str(text: &str) -> Vec<Self> {
        text.chars().map(|c| c as u32).collect()
    }

    fn encode_str_c(text: &str) -> Vec<Self> {
        text.chars().map(|c| c as u32).chain(once(0)).collect()
    }

    fn decode_char(c: Self) -> Result<char> {
        Ok(char::try_from(c)?)
    }

    fn decode_str<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
    {
        let iter = iter.into_iter();
        let (lower_bound, _) = iter.size_hint();

        let mut s = String::with_capacity(lower_bound);
        for c in iter {
            match char::from_u32(c) {
                Some(c) => s.push(c),
                None => bail!("invalid char: {:x}", c),
            }
        }
        Ok(s)
    }

    fn decode_str_c<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        let mut iter = iter.into_iter();

        // Make sure we end in a nul-terminator.
        match iter.next_back() {
            Some(0) => {}
            Some(v) => bail!("invalid terminator for C-style string: {}", v),
            None => bail!("missing terminator for C-style string"),
        }

        Self::decode_str(iter)
    }
}

impl Wide for i16 {
    fn encode_char(c: char) -> Result<Self> {
        u16::encode_char(c).map(|c| c as i16)
    }

    fn encode_str(text: &str) -> Vec<Self> {
        text.encode_utf16().map(|c| c as i16).collect()
    }

    fn encode_str_c(text: &str) -> Vec<Self> {
        text.encode_utf16()
            .map(|c| c as i16)
            .chain(once(0))
            .collect()
    }

    fn decode_char(c: Self) -> Result<char> {
        u16::decode_char(c as u16)
    }

    fn decode_str<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
    {
        u16::decode_str(iter.into_iter().map(|c| c as u16))
    }

    fn decode_str_c<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        u16::decode_str_c(iter.into_iter().map(|c| c as u16))
    }
}

impl Wide for i32 {
    fn encode_char(c: char) -> Result<Self> {
        Ok(c as i32)
    }

    fn encode_str(text: &str) -> Vec<Self> {
        text.chars().map(|c| c as i32).collect()
    }

    fn encode_str_c(text: &str) -> Vec<Self> {
        text.chars().map(|c| c as i32).chain(once(0)).collect()
    }

    fn decode_char(c: Self) -> Result<char> {
        Ok(char::try_from(c as u32)?)
    }

    fn decode_str<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
    {
        u32::decode_str(iter.into_iter().map(|c| c as u32))
    }

    fn decode_str_c<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        u32::decode_str_c(iter.into_iter().map(|c| c as u32))
    }
}
