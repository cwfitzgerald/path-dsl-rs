//! A utility DSL and macro to help deal with paths and pathbufs

#![allow(clippy::cognitive_complexity)]
#![allow(clippy::float_cmp)]
#![deny(nonstandard_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_idioms)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(unused)]

use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::convert::Infallible;
use std::ffi::{OsStr, OsString};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut, Div};
use std::path::{Iter, Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
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

impl AsRef<OsStr> for PathDSL {
    fn as_ref(&self) -> &OsStr {
        self.path.as_ref()
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

impl<'a> Into<Cow<'a, OsStr>> for &'a PathDSL {
    fn into(self) -> Cow<'a, OsStr> {
        Cow::Borrowed(self.path.as_os_str())
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

////////////////
// Partial Eq //
////////////////

impl PartialEq<PathDSL> for PathDSL {
    fn eq(&self, other: &PathDSL) -> bool {
        self.path == other.path
    }
}

impl PartialEq<PathBuf> for PathDSL {
    fn eq(&self, other: &PathBuf) -> bool {
        self.path == *other
    }
}

impl PartialEq<Path> for PathDSL {
    fn eq(&self, other: &Path) -> bool {
        self.path.as_path() == other
    }
}

impl PartialEq<OsStr> for PathDSL {
    fn eq(&self, other: &OsStr) -> bool {
        self.path.as_path() == other
    }
}

impl PartialEq<OsString> for PathDSL {
    fn eq(&self, other: &OsString) -> bool {
        self.path.as_path() == other
    }
}

impl<'a> PartialEq<Cow<'a, Path>> for PathDSL {
    fn eq(&self, other: &Cow<'a, Path>) -> bool {
        self.path.as_path() == other
    }
}

impl<'a> PartialEq<Cow<'a, OsStr>> for PathDSL {
    fn eq(&self, other: &Cow<'a, OsStr>) -> bool {
        self.path.as_path() == other
    }
}

////////
// Eq //
////////

impl Eq for PathDSL {}

/////////////////
// Partial Ord //
/////////////////

impl PartialOrd<PathDSL> for PathDSL {
    fn partial_cmp(&self, other: &PathDSL) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl PartialOrd<PathBuf> for PathDSL {
    fn partial_cmp(&self, other: &PathBuf) -> Option<Ordering> {
        self.path.partial_cmp(other)
    }
}

impl PartialOrd<Path> for PathDSL {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other)
    }
}

impl<'a> PartialOrd<Cow<'a, Path>> for PathDSL {
    fn partial_cmp(&self, other: &Cow<'a, Path>) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other)
    }
}

impl<'a> PartialOrd<Cow<'a, OsStr>> for PathDSL {
    fn partial_cmp(&self, other: &Cow<'a, OsStr>) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other.into())
    }
}

impl PartialOrd<OsStr> for PathDSL {
    fn partial_cmp(&self, other: &OsStr) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other.into())
    }
}

impl PartialOrd<OsString> for PathDSL {
    fn partial_cmp(&self, other: &OsString) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other.into())
    }
}

/////////
// Ord //
/////////

impl Ord for PathDSL {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

/////////////
// FromStr //
/////////////

impl FromStr for PathDSL {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s).map(|path| PathDSL { path })
    }
}

//////////
// Hash //
//////////

impl Hash for PathDSL {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state)
    }
}

////////////
// Extend //
////////////

impl<P> Extend<P> for PathDSL
where
    P: AsRef<Path>,
{
    fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
        self.path.extend(iter)
    }
}

//////////////////
// FromIterator //
//////////////////

impl<'a> IntoIterator for &'a PathDSL {
    type Item = &'a OsStr;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.as_path().into_iter()
    }
}

/////////////
// Default //
/////////////

impl Borrow<Path> for PathDSL {
    fn borrow(&self) -> &Path {
        self.path.borrow()
    }
}

/////////
// Div //
/////////

impl Div<PathDSL> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: PathDSL) -> Self::Output {
        if self.path.as_os_str().is_empty() {
            Self::from(rhs)
        } else {
            self.path.push(rhs);
            self
        }
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
        if self.path.as_os_str().is_empty() {
            Self::from(rhs)
        } else {
            self.path.push(rhs);
            self
        }
    }
}

impl Div<String> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: String) -> Self::Output {
        if self.path.as_os_str().is_empty() {
            Self::from(rhs)
        } else {
            self.path.push(rhs);
            self
        }
    }
}

impl Div<PathBuf> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: PathBuf) -> Self::Output {
        if self.path.as_os_str().is_empty() {
            Self::from(rhs)
        } else {
            self.path.push(rhs);
            self
        }
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

impl Div<Cow<'_, OsStr>> for PathDSL {
    type Output = PathDSL;

    fn div(mut self, rhs: Cow<'_, OsStr>) -> Self::Output {
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

impl Div<Cow<'_, OsStr>> for &PathDSL {
    type Output = PathDSL;

    fn div(self, rhs: Cow<'_, OsStr>) -> Self::Output {
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
    #[test]
    fn dsl_string() {
        let value = String::from("folder");
        let dsl = PathDSL::new() / value;
    }
    #[test]
    fn dsl_string_ref() {
        let value = String::from("folder");
        let dsl = PathDSL::new() / &value;
    }
    #[test]
    fn dsl_osstr() {
        let value_owned = OsString::from("folder");
        let value: &OsStr = &value_owned;
        let dsl = PathDSL::new() / value;
    }
    #[test]
    fn dsl_osstring() {
        let value = OsString::from("folder");
        let dsl = PathDSL::new() / value;
    }
    #[test]
    fn dsl_osstring_ref() {
        let value = OsString::from("folder");
        let dsl = &PathDSL::new() / &value;
        let bp: Box<Path> = dsl.into();
    }
}
