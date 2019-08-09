//! A utility DSL and macro to help deal with paths and pathbufs

#![allow(clippy::cognitive_complexity)]
#![allow(clippy::float_cmp)]
#![deny(nonstandard_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(unused)]

use std::borrow::Cow;
use std::ffi::OsString;
use std::ops::{Deref, DerefMut, Div};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

/// A PathBuf wrapper that has support for a Path DSL.
///
/// It is usable nearly identically to a PathBuf.
/// Supports [`Deref`](https://doc.rust-lang.org/stable/core/ops/trait.Deref.html) to
/// [`PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html) to cover all edge cases.
///
/// See [crate documentation](index.html) for usage examples.
#[derive(Debug, Clone, Default)]
pub struct PathDSL {
    path: PathBuf,
}

impl PathDSL {
    pub fn new() -> Self {
        PathDSL { path: PathBuf::new() }
    }

    pub fn into_os_string(self) -> OsString {
        self.path.into_os_string()
    }

    pub fn into_boxed_path(self) -> Box<Path> {
        self.path.into_boxed_path()
    }
}

//////////////////////////////////
// Pretending to be a Path(Buf) //
//////////////////////////////////

impl AsRef<Path> for PathDSL {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl AsMut<PathBuf> for PathDSL {
    fn as_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }
}

impl Deref for PathDSL {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for PathDSL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

//////////
// From //
//////////

impl<T> From<&T> for PathDSL
where
    T: AsRef<Path> + ?Sized,
{
    fn from(other: &T) -> Self {
        PathDSL {
            path: PathBuf::from(other.as_ref()),
        }
    }
}

impl From<PathBuf> for PathDSL {
    fn from(other: PathBuf) -> Self {
        PathDSL { path: other }
    }
}

impl From<OsString> for PathDSL {
    fn from(other: OsString) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<String> for PathDSL {
    fn from(other: String) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<Box<Path>> for PathDSL {
    fn from(other: Box<Path>) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<Cow<'_, Path>> for PathDSL {
    fn from(other: Cow<'_, Path>) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

//////////
// Into //
//////////
// We can't implement from on these types, so the best we can do is Into.

impl Into<OsString> for PathDSL {
    fn into(self) -> OsString {
        self.into_os_string()
    }
}

impl Into<Box<Path>> for PathDSL {
    fn into(self) -> Box<Path> {
        self.into_boxed_path()
    }
}

impl<'a> Into<Cow<'a, Path>> for PathDSL {
    fn into(self) -> Cow<'a, Path> {
        self.path.into()
    }
}

impl<'a> Into<Cow<'a, Path>> for &'a PathDSL {
    fn into(self) -> Cow<'a, Path> {
        Cow::Borrowed(self.path.as_path())
    }
}

impl<'a> Into<Arc<Path>> for PathDSL {
    fn into(self) -> Arc<Path> {
        self.path.into()
    }
}

impl<'a> Into<Rc<Path>> for PathDSL {
    fn into(self) -> Rc<Path> {
        self.path.into()
    }
}

/////////
// Div //
/////////

impl Div<PathDSL> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: PathDSL) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl<T> Div<&T> for PathDSL
where
    T: AsRef<Path> + ?Sized,
{
    type Output = PathDSL;

    fn div(mut self, rhs: &T) -> Self::Output {
        self.path.push(rhs.as_ref());
        self
    }
}

impl Div<OsString> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: OsString) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Div<String> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: String) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Div<PathBuf> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: PathBuf) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Div<Box<Path>> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: Box<Path>) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Div<Cow<'_, Path>> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: Cow<'_, Path>) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

///////////
// Div & //
///////////

impl Div<PathDSL> for &PathDSL {
    type Output = PathDSL;

    fn div(self, rhs: PathDSL) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl<T> Div<&T> for &PathDSL
where
    T: AsRef<Path> + ?Sized,
{
    type Output = PathDSL;

    fn div(self, rhs: &T) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs.as_ref());
        new_self
    }
}

impl Div<OsString> for &PathDSL {
    type Output = PathDSL;

    fn div(self, rhs: OsString) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<String> for &PathDSL {
    type Output = PathDSL;

    fn div(self, rhs: String) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<PathBuf> for &PathDSL {
    type Output = PathDSL;

    fn div(self, rhs: PathBuf) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Box<Path>> for &PathDSL {
    type Output = PathDSL;

    fn div(self, rhs: Box<Path>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Cow<'_, Path>> for &PathDSL {
    type Output = PathDSL;

    fn div(self, rhs: Cow<'_, Path>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

#[cfg(test)]
mod test {
    use crate::PathDSL;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn dsl_str() {
        let value: &str = "folder";
        let dsl = PathDSL::new() / value;
    }
    fn dsl_string() {
        let value = String::from("folder");
        let dsl = PathDSL::new() / value;
    }
    fn dsl_string_ref() {
        let value = String::from("folder");
        let dsl = PathDSL::new() / &value;
    }
    fn dsl_osstr() {
        let value_owned = OsString::from("folder");
        let value: &OsStr = &value_owned;
        let dsl = PathDSL::new() / value;
    }
    fn dsl_osstring() {
        let value = OsString::from("folder");
        let dsl = PathDSL::new() / value;
    }
    fn dsl_osstring_ref() {
        let value = OsString::from("folder");
        let dsl = &PathDSL::new() / &value;
        let bp: Box<Path> = dsl.into();
    }
}
