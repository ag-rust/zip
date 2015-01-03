//! Byte container optionally encoded as UTF-8.

use std::{str, fmt};
use std::borrow::Cow;
use std::str::CowString;
use std::default::Default;
use std::path::BytesContainer;

#[deriving(Clone)]
pub enum MaybeUTF8 {
    UTF8(String),
    Bytes(Vec<u8>),
}

impl MaybeUTF8 {
    pub fn new() -> MaybeUTF8 {
        MaybeUTF8::UTF8(String::new())
    }

    pub fn from_str(s: String) -> MaybeUTF8 {
        MaybeUTF8::UTF8(s)
    }

    pub fn from_bytes(v: Vec<u8>) -> MaybeUTF8 {
        MaybeUTF8::Bytes(v)
    }

    pub fn as_bytes<'a>(&'a self) -> &'a [u8] {
        match *self {
            MaybeUTF8::UTF8(ref s) => s.as_bytes(),
            MaybeUTF8::Bytes(ref v) => v.as_slice(),
        }
    }

    pub fn as_str<'a>(&'a self) -> Option<&'a str> {
        match *self {
            MaybeUTF8::UTF8(ref s) => Some(s.as_slice()),
            MaybeUTF8::Bytes(ref v) => str::from_utf8(v.as_slice()).ok(),
        }
    }

    pub fn map_as_cow<'a>(&'a self,
                                  as_cow: |&'a [u8]| -> CowString<'a>) -> CowString<'a> {
        match *self {
            MaybeUTF8::UTF8(ref s) => s.as_slice().into_cow(),
            MaybeUTF8::Bytes(ref v) => as_cow(v.as_slice()),
        }
    }

    pub fn as_cow<'a>(&'a self) -> CowString<'a> {
        self.map_as_cow(String::from_utf8_lossy)
    }

    pub fn into_str(self) -> Result<String, MaybeUTF8> {
        match self {
            MaybeUTF8::UTF8(s) => Ok(s),
            MaybeUTF8::Bytes(v) => match String::from_utf8(v) {
                Ok(s) => Ok(s),
                Err(e) => Err(MaybeUTF8::Bytes(e.into_bytes())),
            },
        }
    }

    pub fn map_into_str(self, into_str: |Vec<u8>| -> String) -> String {
        match self {
            MaybeUTF8::UTF8(s) => s,
            MaybeUTF8::Bytes(v) => into_str(v),
        }
    }

    pub fn into_str_lossy(self) -> String {
        self.map_into_str(|v| match String::from_utf8_lossy(v.as_slice()) {
            // `v` is definitely UTF-8, so do not make a copy!
            Cow::Borrowed(_) => unsafe {String::from_utf8_unchecked(v)},
            Cow::Owned(s) => s,
        })
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            MaybeUTF8::UTF8(s) => s.into_bytes(),
            MaybeUTF8::Bytes(v) => v,
        }
    }

    pub fn len(&self) -> uint {
        match *self {
            MaybeUTF8::UTF8(ref s) => s.len(),
            MaybeUTF8::Bytes(ref v) => v.len(),
        }
    }

    pub fn clear(&mut self) {
        match *self {
            MaybeUTF8::UTF8(ref mut s) => s.clear(),
            MaybeUTF8::Bytes(ref mut v) => v.clear(),
        }
    }
}

impl<S: BytesContainer> PartialEq<S> for MaybeUTF8 {
    fn eq(&self, other: &S) -> bool {
        self.as_bytes().eq(other.container_as_bytes())
    }
}

impl Eq for MaybeUTF8 {
}

impl<S: BytesContainer> PartialOrd<S> for MaybeUTF8 {
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        self.as_bytes().partial_cmp(other.container_as_bytes())
    }
}

impl Ord for MaybeUTF8 {
    fn cmp(&self, other: &MaybeUTF8) -> Ordering {
        self.as_bytes().cmp(other.container_as_bytes())
    }
}

impl BytesContainer for MaybeUTF8 {
    fn container_as_bytes<'a>(&'a self) -> &'a [u8] {
        self.as_bytes()
    }

    fn container_as_str<'a>(&'a self) -> Option<&'a str> {
        self.as_str()
    }

    fn is_str(_: Option<&MaybeUTF8>) -> bool {
        false
    }
}

// a workaround for multiple `FromIterator` implementations with differing type params
trait MaybeUTF8FromIterator {
    fn maybe_utf8_from_iter<I:Iterator<Self>>(iterator: I) -> MaybeUTF8;
}

impl MaybeUTF8FromIterator for char {
    fn maybe_utf8_from_iter<I:Iterator<char>>(iterator: I) -> MaybeUTF8 {
        MaybeUTF8::from_str(FromIterator::from_iter(iterator))
    }
}

impl MaybeUTF8FromIterator for u8 {
    fn maybe_utf8_from_iter<I:Iterator<u8>>(iterator: I) -> MaybeUTF8 {
        MaybeUTF8::from_bytes(FromIterator::from_iter(iterator))
    }
}

impl<T:MaybeUTF8FromIterator> FromIterator<T> for MaybeUTF8 {
    fn from_iter<I:Iterator<T>>(iterator: I) -> MaybeUTF8 {
        MaybeUTF8FromIterator::maybe_utf8_from_iter(iterator)
    }
}

impl Default for MaybeUTF8 {
    fn default() -> MaybeUTF8 {
        MaybeUTF8::new()
    }
}

impl fmt::Show for MaybeUTF8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write(self.as_bytes())
    }
}

