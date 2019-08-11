//! DSL PathBuf Wrapper and Macro for easy creation of paths.
//!
//! PathBuf (and Path) give us a cross platform way to handle paths,
//! but when you are creating part a path, you often want to
//! use a raw string or a formatted string to express that. While
//! this is significantly more terse, it has cross platform issues
//! because of the slashes use in the string. Enter PathDSL.
//!
//! # **Incorrect:** String
//!
//! This is an easy but incorrect way of creating a path.
//!
//! ```rust,no_run
//! use std::path::PathBuf;
//! // Fails on windows when put onto the end of an absolute path
//! let path = PathBuf::from("dir1/dir2/dir3/file.txt");
//!
//! # let mut path2 = PathBuf::new();
//! # path2.push("dir1");
//! # path2.push("dir2");
//! # path2.push("dir3");
//! # path2.push("file.txt");
//! # assert_eq!(path, path2);
//! ```
//!
//! # PathBuf API
//!
//! This is a correct but extremely verbose and mutable way of creating a path.
//! It is possible to mitigate the mutability by making a mutable path inside a block
//! and then assing the result of the block to an immutable variable, but that increases
//! the amount of code.
//!
//! ```rust
//! use std::path::PathBuf;
//! let mut path = PathBuf::new();
//! path.push("dir1");
//! path.push("dir2");
//! path.push("dir3");
//! path.push("file.txt");
//! ```
//!
//! # PathDSL Macro
//!
//! Compare with PathDSL's `path!` macro (note the use of `|` instead of `/` due to rust's macro rules).
//! PathDSL is a drop-in replacement for PathBuf and is easily and cheeply convertable back and forth. This
//! macro has a couple optimizations over just using the PathDSL class manually, described later.
//!
//! ```rust
//! use path_dsl::{path, PathDSL};
//! // Type annotation for illustration only, not needed
//! let path: PathDSL = path!("dir1" | "dir2" | "dir3" | "file.txt");
//!
//! # use std::path::PathBuf;
//! # let mut path2 = PathBuf::new();
//! # path2.push("dir1");
//! # path2.push("dir2");
//! # path2.push("dir3");
//! # path2.push("file.txt");
//! # assert_eq!(path, path2);
//! ```
//!
//! # PathDSL
//!
//! You can also generate a PathDSL directly:
//!
//! ```rust
//! use path_dsl::PathDSL;
//! let path = PathDSL::from("dir1") / "dir2" / "dir3" / "file.txt";
//!
//! # use std::path::PathBuf;
//! # let mut path2 = PathBuf::new();
//! # path2.push("dir1");
//! # path2.push("dir2");
//! # path2.push("dir3");
//! # path2.push("file.txt");
//! # assert_eq!(path, path2);
//! ```
//!
//! # Adding Path-Like Structures
//!
//! As well as using regular string literals, you can use anything that can be passed to `PathBuf::push`
//! as a part of the DSL.
//!
//! Note the borrow on `other`: as these types are not `Copy`, they will be moved
//! into the path unless you borrow them. This matches behavior with `PathBuf::push`, but can be suprising
//! when used in a infix expression.
//!
//! ```rust
//! use path_dsl::{path, PathDSL};
//!
//! let other = PathBuf::from("some_dir");
//! let filename: &str = "my_file.txt";
//!
//! let path = PathDSL::from("dir1") / "dir2" / &other / filename;
//! let mac  = path!("dir1" | "dir2" | other | filename);
//!
//! # use std::path::PathBuf;
//! # let mut path2 = PathBuf::new();
//! # path2.push("dir1");
//! # path2.push("dir2");
//! # path2.push("some_dir");
//! # path2.push("my_file.txt");
//! # assert_eq!(path, path2);
//! # assert_eq!(mac, path2);
//! ```

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
    #[inline]
    pub fn new() -> Self {
        PathDSL { path: PathBuf::new() }
    }

    #[inline]
    pub fn into_os_string(self) -> OsString {
        self.path.into_os_string()
    }

    #[inline]
    pub fn into_boxed_path(self) -> Box<Path> {
        self.path.into_boxed_path()
    }
}

//////////////////////////////////
// Pretending to be a Path(Buf) //
//////////////////////////////////

impl AsRef<Path> for PathDSL {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl AsMut<PathBuf> for PathDSL {
    #[inline]
    fn as_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }
}

impl AsRef<OsStr> for PathDSL {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.path.as_ref()
    }
}

impl Deref for PathDSL {
    type Target = PathBuf;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for PathDSL {
    #[inline]
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
    #[inline]
    fn from(other: &T) -> Self {
        PathDSL {
            path: PathBuf::from(other.as_ref()),
        }
    }
}

impl From<PathBuf> for PathDSL {
    #[inline]
    fn from(other: PathBuf) -> Self {
        PathDSL { path: other }
    }
}

impl From<OsString> for PathDSL {
    #[inline]
    fn from(other: OsString) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<String> for PathDSL {
    #[inline]
    fn from(other: String) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<Box<Path>> for PathDSL {
    #[inline]
    fn from(other: Box<Path>) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<Cow<'_, Path>> for PathDSL {
    #[inline]
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
    #[inline]
    fn into(self) -> OsString {
        self.into_os_string()
    }
}

impl Into<Box<Path>> for PathDSL {
    #[inline]
    fn into(self) -> Box<Path> {
        self.into_boxed_path()
    }
}

impl<'a> Into<Cow<'a, Path>> for PathDSL {
    #[inline]
    fn into(self) -> Cow<'a, Path> {
        self.path.into()
    }
}

impl<'a> Into<Cow<'a, Path>> for &'a PathDSL {
    #[inline]
    fn into(self) -> Cow<'a, Path> {
        Cow::Borrowed(self.path.as_path())
    }
}

impl<'a> Into<Cow<'a, OsStr>> for &'a PathDSL {
    #[inline]
    fn into(self) -> Cow<'a, OsStr> {
        Cow::Borrowed(self.path.as_os_str())
    }
}

impl<'a> Into<Arc<Path>> for PathDSL {
    #[inline]
    fn into(self) -> Arc<Path> {
        self.path.into()
    }
}

impl<'a> Into<Rc<Path>> for PathDSL {
    #[inline]
    fn into(self) -> Rc<Path> {
        self.path.into()
    }
}

////////////////
// Partial Eq //
////////////////

impl PartialEq<PathDSL> for PathDSL {
    #[inline]
    fn eq(&self, other: &PathDSL) -> bool {
        self.path == other.path
    }
}

impl PartialEq<PathBuf> for PathDSL {
    #[inline]
    fn eq(&self, other: &PathBuf) -> bool {
        self.path == *other
    }
}

impl PartialEq<Path> for PathDSL {
    #[inline]
    fn eq(&self, other: &Path) -> bool {
        self.path.as_path() == other
    }
}

impl PartialEq<OsStr> for PathDSL {
    #[inline]
    fn eq(&self, other: &OsStr) -> bool {
        self.path.as_path() == other
    }
}

impl PartialEq<OsString> for PathDSL {
    #[inline]
    fn eq(&self, other: &OsString) -> bool {
        self.path.as_path() == other
    }
}

impl<'a> PartialEq<Cow<'a, Path>> for PathDSL {
    #[inline]
    fn eq(&self, other: &Cow<'a, Path>) -> bool {
        self.path.as_path() == other
    }
}

impl<'a> PartialEq<Cow<'a, OsStr>> for PathDSL {
    #[inline]
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
    #[inline]
    fn partial_cmp(&self, other: &PathDSL) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl PartialOrd<PathBuf> for PathDSL {
    #[inline]
    fn partial_cmp(&self, other: &PathBuf) -> Option<Ordering> {
        self.path.partial_cmp(other)
    }
}

impl PartialOrd<Path> for PathDSL {
    #[inline]
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other)
    }
}

impl<'a> PartialOrd<Cow<'a, Path>> for PathDSL {
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, Path>) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other)
    }
}

impl<'a> PartialOrd<Cow<'a, OsStr>> for PathDSL {
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, OsStr>) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other.into())
    }
}

impl PartialOrd<OsStr> for PathDSL {
    #[inline]
    fn partial_cmp(&self, other: &OsStr) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other.into())
    }
}

impl PartialOrd<OsString> for PathDSL {
    #[inline]
    fn partial_cmp(&self, other: &OsString) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other.into())
    }
}

/////////
// Ord //
/////////

impl Ord for PathDSL {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

/////////////
// FromStr //
/////////////

impl FromStr for PathDSL {
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s).map(|path| PathDSL { path })
    }
}

//////////
// Hash //
//////////

impl Hash for PathDSL {
    #[inline]
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
    #[inline]
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

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.path.as_path().into_iter()
    }
}

/////////////
// Default //
/////////////

impl Borrow<Path> for PathDSL {
    #[inline]
    fn borrow(&self) -> &Path {
        self.path.borrow()
    }
}

/////////
// Div //
/////////

impl Div<PathDSL> for PathDSL {
    type Output = PathDSL;

    #[inline]
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

    #[inline]
    fn div(mut self, rhs: &T) -> Self::Output {
        self.path.push(rhs.as_ref());
        self
    }
}

impl Div<OsString> for PathDSL {
    type Output = PathDSL;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn div(mut self, rhs: Box<Path>) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Div<Cow<'_, Path>> for PathDSL {
    type Output = PathDSL;

    #[inline]
    fn div(mut self, rhs: Cow<'_, Path>) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Div<Cow<'_, OsStr>> for PathDSL {
    type Output = PathDSL;

    #[inline]
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

    #[inline]
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

    #[inline]
    fn div(self, rhs: &T) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs.as_ref());
        new_self
    }
}

impl Div<OsString> for &PathDSL {
    type Output = PathDSL;

    #[inline]
    fn div(self, rhs: OsString) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<String> for &PathDSL {
    type Output = PathDSL;

    #[inline]
    fn div(self, rhs: String) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<PathBuf> for &PathDSL {
    type Output = PathDSL;

    #[inline]
    fn div(self, rhs: PathBuf) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Box<Path>> for &PathDSL {
    type Output = PathDSL;

    #[inline]
    fn div(self, rhs: Box<Path>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Cow<'_, Path>> for &PathDSL {
    type Output = PathDSL;

    #[inline]
    fn div(self, rhs: Cow<'_, Path>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Cow<'_, OsStr>> for &PathDSL {
    type Output = PathDSL;

    #[inline]
    fn div(self, rhs: Cow<'_, OsStr>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

#[cfg(windows)]
#[doc(hidden)]
#[macro_export]
macro_rules! separator {
    () => { "\\" };
}

#[cfg(not(windows))]
#[doc(hidden)]
#[macro_export]
macro_rules! separator {
    () => { "/" };
}

#[doc(hidden)]
#[macro_export]
macro_rules! concat_separator {
    ( $e:literal, $($other:literal),+ ) => {
        concat!($e, path_dsl::separator!(), path_dsl::concat_separator!($($other),+))
    };
    ( $e:literal ) => {
        $e
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! path_impl {
    ( @($($stack:expr),*)@ ($exp:expr) | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $exp)@ $($other)+ )
    };
    ( @($($stack:expr),*)@ ($exp:expr) ) => {
        $($stack),* / $exp
    };
    ( @($($stack:expr),*)@ $blk:block | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $blk)@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $blk:block ) => {
        $($stack),* / $blk
    };
    ( @($($stack:expr),*)@ $name:path | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $name)@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $name:path ) => {
        $($stack),* / $name
    };
    ( @($($stack:expr),*)@ &$name:path | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / &$name)@ $($other)+ )
    };
    ( @($($stack:expr),*)@ &$name:path ) => {
        $($stack),* / &$name
    };
    ( @($($stack:expr),*)@ &mut $name:path | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / &mut $name)@ $($other)+ )
    };
    ( @($($stack:expr),*)@ &mut $name:path ) => {
        $($stack),* / &mut $name
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $lit9:literal | $lit10:literal | $lit11:literal | $lit12:literal | $lit13:literal | $lit14:literal | $lit15:literal | $lit16:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8, $lit9, $lit10, $lit11, $lit12, $lit13, $lit14, $lit15, $lit16))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $lit9:literal | $lit10:literal | $lit11:literal | $lit12:literal | $lit13:literal | $lit14:literal | $lit15:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8, $lit9, $lit10, $lit11, $lit12, $lit13, $lit14, $lit15))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $lit9:literal | $lit10:literal | $lit11:literal | $lit12:literal | $lit13:literal | $lit14:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8, $lit9, $lit10, $lit11, $lit12, $lit13, $lit14))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $lit9:literal | $lit10:literal | $lit11:literal | $lit12:literal | $lit13:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8, $lit9, $lit10, $lit11, $lit12, $lit13))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $lit9:literal | $lit10:literal | $lit11:literal | $lit12:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8, $lit9, $lit10, $lit11, $lit12))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $lit9:literal | $lit10:literal | $lit11:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8, $lit9, $lit10, $lit11))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $lit9:literal | $lit10:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8, $lit9, $lit10))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $lit9:literal| $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8, $lit9))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $lit8:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7, $lit8))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $lit7:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6, $lit7))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $lit6:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5, $lit6))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $lit5:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4, $lit5))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $lit4:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3, $lit4))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $lit3:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2, $lit3))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $lit2:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $crate::concat_separator!($lit, $lit2))@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal | $($other:tt)+ ) => {
        $crate::path_impl!( @($($stack),* / $lit)@ $($other)+ )
    };
    ( @($($stack:expr),*)@ $lit:literal ) => {
        $($stack),* / $lit
    };
    ( @($($stack:expr),*)@ ) => {
        $($stack),*
    };
}

#[macro_export]
macro_rules! path {
    ( $($other:tt)* ) => {
         $crate::path_impl!( @($crate::PathDSL::new())@ $($other)* )
    };
    () => {  $crate::PathDSL::new() };
}

#[cfg(test)]
mod test {
    use crate::PathDSL;
    use crate::path;
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
    #[test]
    fn macro_test() {
        let b = "blah";
//        let p = path!("blah" | "hi" | "hi2" | "hi3" | "hi4" | "hi5" | "hi6" | "hi7" | "hi8" | "hi9" | "hi10" | "hi11" | "hi12" | "hi13" | "hi14" | "hi15" | "hi16"| b);
    }
}
