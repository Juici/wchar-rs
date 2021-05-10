use std::iter::once;

use anyhow::{bail, Result};

pub trait Wide: Sized {
    fn encode_wide(text: &str) -> Vec<Self>;
    fn encode_wide_c(text: &str) -> Vec<Self>;
    fn decode_wide<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>;
    fn decode_wide_c<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator;
}

impl Wide for u16 {
    fn encode_wide(text: &str) -> Vec<Self> {
        text.encode_utf16().collect()
    }

    fn encode_wide_c(text: &str) -> Vec<Self> {
        text.encode_utf16().chain(once(0)).collect()
    }

    fn decode_wide<I>(iter: I) -> Result<String>
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

    fn decode_wide_c<I>(iter: I) -> Result<String>
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

        Self::decode_wide(iter)
    }
}

impl Wide for u32 {
    fn encode_wide(text: &str) -> Vec<Self> {
        text.chars().map(|c| c as u32).collect()
    }

    fn encode_wide_c(text: &str) -> Vec<Self> {
        text.chars().map(|c| c as u32).chain(once(0)).collect()
    }

    fn decode_wide<I>(iter: I) -> Result<String>
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

    fn decode_wide_c<I>(iter: I) -> Result<String>
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

        Self::decode_wide(iter)
    }
}

impl Wide for i16 {
    fn encode_wide(text: &str) -> Vec<Self> {
        text.encode_utf16().map(|c| c as i16).collect()
    }

    fn encode_wide_c(text: &str) -> Vec<Self> {
        text.encode_utf16()
            .map(|c| c as i16)
            .chain(once(0))
            .collect()
    }

    fn decode_wide<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
    {
        u16::decode_wide(iter.into_iter().map(|c| c as u16))
    }

    fn decode_wide_c<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        u16::decode_wide_c(iter.into_iter().map(|c| c as u16))
    }
}

impl Wide for i32 {
    fn encode_wide(text: &str) -> Vec<Self> {
        text.chars().map(|c| c as i32).collect()
    }

    fn encode_wide_c(text: &str) -> Vec<Self> {
        text.chars().map(|c| c as i32).chain(once(0)).collect()
    }

    fn decode_wide<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
    {
        u32::decode_wide(iter.into_iter().map(|c| c as u32))
    }

    fn decode_wide_c<I>(iter: I) -> Result<String>
    where
        I: IntoIterator<Item = Self>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        u32::decode_wide_c(iter.into_iter().map(|c| c as u32))
    }
}
