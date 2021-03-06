// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::default::Default;
use std::vec;
use serialize::{Encodable, Decodable, Encoder, Decoder};

/// A non-growable owned slice. This is a separate type to allow the
/// representation to change.
#[deriving(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnedSlice<T> {
    data: Box<[T]>
}

impl<T:fmt::Show> fmt::Show for OwnedSlice<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(fmt)
    }
}

impl<T> OwnedSlice<T> {
    pub fn empty() -> OwnedSlice<T> {
        OwnedSlice  { data: box [] }
    }

    #[inline(never)]
    pub fn from_vec(v: Vec<T>) -> OwnedSlice<T> {
        OwnedSlice { data: v.into_boxed_slice() }
    }

    #[inline(never)]
    pub fn into_vec(self) -> Vec<T> {
        self.data.into_vec()
    }

    pub fn as_slice<'a>(&'a self) -> &'a [T] {
        &*self.data
    }

    pub fn move_iter(self) -> vec::IntoIter<T> {
        self.into_vec().into_iter()
    }

    pub fn map<U, F: FnMut(&T) -> U>(&self, f: F) -> OwnedSlice<U> {
        self.iter().map(f).collect()
    }
}

impl<T> Deref<[T]> for OwnedSlice<T> {
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> Default for OwnedSlice<T> {
    fn default() -> OwnedSlice<T> {
        OwnedSlice::empty()
    }
}

impl<T: Clone> Clone for OwnedSlice<T> {
    fn clone(&self) -> OwnedSlice<T> {
        OwnedSlice::from_vec(self.as_slice().to_vec())
    }
}

impl<T> FromIterator<T> for OwnedSlice<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> OwnedSlice<T> {
        OwnedSlice::from_vec(iter.collect())
    }
}

impl<S: Encoder<E>, T: Encodable<S, E>, E> Encodable<S, E> for OwnedSlice<T> {
    fn encode(&self, s: &mut S) -> Result<(), E> {
       self.as_slice().encode(s)
    }
}

impl<D: Decoder<E>, T: Decodable<D, E>, E> Decodable<D, E> for OwnedSlice<T> {
    fn decode(d: &mut D) -> Result<OwnedSlice<T>, E> {
        Ok(OwnedSlice::from_vec(match Decodable::decode(d) {
            Ok(t) => t,
            Err(e) => return Err(e)
        }))
    }
}
