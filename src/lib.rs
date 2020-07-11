//! Utility DSL and macro to help deal with Paths.
//!
//! PathDSL provides a simple and zero-overhead abstraction for creating
//! paths and appending to existing `Path`-like things.
//!
//! # Overview
//!
//! ```rust
//! use path_dsl::path;
//! # use std::path::{PathBuf, Path};
//!
//! // PathBuf::push() only called once with consecutive literals:
//! let literals: PathBuf = path!("dir1" | "dir2" | "dir3");
//! // Type annotation for illustration purposes; not needed
//!
//! // Does not copy data if first path segment is a owning value:
//! let moving = path!(literals | "dir4");
//! # let mut moving_pb = PathBuf::new();
//! # moving_pb.push("dir1");
//! # moving_pb.push("dir2");
//! # moving_pb.push("dir3");
//! # moving_pb.push("dir4");
//! # assert_eq!(moving, moving_pb);
//!
//! // Mixing and matching is easy:
//! let start = path!("some" | "dir");
//! let end = path!("my_folder" | "my_file.txt");
//! # let mut end_pb = PathBuf::new();
//! # end_pb.push("my_folder");
//! # end_pb.push("my_file.txt");
//! # assert_eq!(end, end_pb);
//! // Can borrow as normal
//! let result = path!(start | "middle_folder" | &end);
//! # let mut result_pb = PathBuf::new();
//! # result_pb.push("some");
//! # result_pb.push("dir");
//! # result_pb.push("middle_folder");
//! # result_pb.push(end_pb);
//! # assert_eq!(result, result_pb);
//!
//! // Works with PathBuf, Path, and String-likes
//! let file = Path::new("file.txt");
//! let folder = PathBuf::from("folder");
//! let middle: &str = "other_middle";
//! let combined = path!(folder | middle | "middle_folder" | file);
//! # let mut combined_pb = PathBuf::new();
//! # combined_pb.push("folder");
//! # combined_pb.push("other_middle");
//! # combined_pb.push("middle_folder");
//! # combined_pb.push("file.txt");
//! # assert_eq!(combined, combined_pb);
//! ```
//!
//! # PathDSL Macro and Type
//!
//! PathDSL's [`path!`](macro.path.html) macro allows for the creation of a `PathBuf` in the most efficent way possible in the situation.
//!
//! note the use of `|` instead of `/` due to rust's macro rules
//!
//! ```rust
//! use path_dsl::path;
//! // Type annotation for illustration only, not needed
//! let path: PathBuf = path!("dir1" | "dir2" | "dir3" | "file.txt");
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
//! ### PathDSL
//!
//! You can also generate a PathDSL directly, though this is discouraged. PathDSL will pretend to be
//! a `PathBuf` as best it can, but it is almost always more efficent to use the `path!` macro to generate
//! a `PathBuf` directly.
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
//! ### Adding Path-Like Structures
//!
//! As well as using regular string literals, you can use anything that can be passed to `PathBuf::push`
//! as a part of the DSL.
//!
//! Note the borrow on `other`: as these types are not `Copy`, they will be moved
//! into the path unless you borrow them. This matches behavior with `PathBuf::push`, but can be surprising
//! when used in a infix expression.
//!
//! ```rust
//! use path_dsl::{path, PathDSL};
//!
//! let other = PathBuf::from("some_dir");
//! let filename: &str = "my_file.txt";
//!
//! let mac: PathBuf  = path!("dir1" | "dir2" | &other | filename); // Preferred
//! let path: PathDSL = PathDSL::from("dir1") / "dir2" / other / filename; // Also works
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
//!
//! ### Moving vs Borrowing
//!
//! Both the macro and the DSL type behave the same with regard to borrowing vs moving. If a
//! reference is provided, it will borrow the provided value. However, if a value is provided
//! **it will move it**, making the value unusable afterwards. While these are the normal rules
//! for rust, infix operators are normally used with `Copy` types, so this may be **surprising**.
//!
//! Both mutable and immutable borrows are supported, though they will never actually mutate anything.
//!
//! ```rust,compile_fail
//! use path_dsl::path;
//! # use std::path::PathBuf;
//!
//! let value = PathBuf::from("some_dir");
//! let borrow: &str = "my_file.txt";
//!
//! let mac  = path!(value | borrow);
//! let path = path!(value | borrow); // Will not compile because `value` was moved
//! ```
//!
//! You must manually borrow it:
//!
//! ```rust
//! # use path_dsl::{path, PathDSL};
//! #
//! # let value = PathBuf::from("some_dir");
//! # let borrow: &str = "my_file.txt";
//! #
//! let mac  = path!(&value | borrow); // Borrow value so it can be used later
//! let path = PathDSL::new() / value / borrow; // Not used afterwards, so doesn't need a borrow
//!
//! # use std::path::PathBuf;
//! # let mut path2 = PathBuf::new();
//! # path2.push("some_dir");
//! # path2.push("my_file.txt");
//! # assert_eq!(path, path2);
//! # assert_eq!(mac, path2);
//! ```
//!
//! ### PathDSL <=> PathBuf
//!
//! **The PathDSL type is not meant to be used directly, but exists to allow the macro to work.
//! It was once intended to be directly used, so it ratains this functionality.**
//!
//! `PathDSL` is designed to be a drop-in replacement for `PathBuf`, including trivial conversions
//! between the two. In any situation where you would be able to use `PathBuf` you can use
//! `PathDSL`. `PathDSL` includes an implementation of `Deref` to a `PathBuf` (and by proxy `Path`) and re-implements all functions that take `self`, so is fully api compatable.
//! However there are some situations where you must have a `PathBuf`.
//! Obtaining a `&PathBuf` is trivial through dereferencing and obtaining a `PathBuf` is possible through the [`PathDSL::into_pathbuf`](struct.PathDSL.html#method.into_pathbuf) function.
//!
//! PathDSL is `#[repr(transparent)]` over `PathBuf` and all functions are force-inlined so
//! conversions and operations should be cost-free compared to the equivalent `PathBuf` operation.
//! If they aren't, please file a bug.
//!
//! Some known issues are:
//!
//! **Equality**
//!
//! ```rust
//! use path_dsl::path;
//! # use std::path::PathBuf;
//!
//! let dsl = path!("file.txt");
//! let buf = PathBuf::from("file.txt");
//!
//! assert!(dsl == buf);
//! // Must de-reference to PathBuf can't implement `Eq` for `PathBuf`
//! assert!(buf == *dsl);
//! ```
//!
//! **Function Calls**
//!
//! ```rust
//! use path_dsl::path;
//! # use std::path::PathBuf;
//!
//! fn func(p: PathBuf) {
//! # assert_eq!(p, PathBuf::from("file.txt"));
//! }
//!
//! let dsl = path!("file.txt");
//! let buf = PathBuf::from("file.txt");
//!
//! func(buf);
//! // Must convert into `PathBuf`
//! // Dereferencing doesn't work because `func` moves.
//! func(dsl.to_path_buf());
//! # let dsl = path!("file.txt");
//! func(dsl.into()) // also works
//! ```
//!
//! ### Macro Optimizations
//!
//! As previously mentioned, the macro contains some optimizations over using raw `PathDSL` and should always
//! be used over manually using PathDSL. These optimizations happen at compile time, and are guaranteed.
//! Further details on these can be found on the [`path!`](macro.path.html) macro documentation.
//!
//! **String Literal Concatenation:**
//!
//! While it is ill-advised to use string literals with slashes in a `Path`, The [`path!`](macro.path.html) macro
//! takes slashes into account, and automatically constructs a single string literal from multiple
//! consecutive string literals. This can potentially save an allocation or two in the underlying
//! `OsString`.
//!
//! ```rust
//! use path_dsl::path;
//! # use std::path::PathBuf;
//!
//! let p = path!("this" | "is" | "combined");
//! if cfg!(windows) {
//!     assert_eq!(p, PathBuf::from("this\\is\\combined"));
//! } else {
//!     assert_eq!(p, PathBuf::from("this/is/combined"));
//! }
//! ```
//!
//! **First-Argument Optimization:**
//!
//! When the very first argument of the [`path!`](macro.path.html) macro is a owning `PathBuf`, `OsString` or `PathDSL`
//! passed by value (moved), instead of copying everything into a new `PathDSL`, it will just steal the
//! buffer from that moved-in value. This allows you to use the [`path!`](macro.path.html) macro fearlessly when
//! appending to already existing variables.
//!
//! ```rust
//! use path_dsl::path;
//! # use std::path::PathBuf;
//!
//! let first = PathBuf::from("a_very_long_folder_name");
//! # let dup = first.clone();
//! let p = path!(first); // Does not copy anything.
//!
//! # assert_eq!(p, dup);
//! ```
//!
//! # Why Use A Crate?
//!
//! You may be wondering why you should use a crate for this when you can easily wrap `PathBuf` and
//! add some `Div` implementations. This is basically what I thought as well until I actually went
//! to go implement this crate. There is a surprising amount of very tedious and particular code to try to emulate
//! `PathBuf` directly, as well as to test the functionality.
//!
//! With this in mind, I have made `path_dsl` completely dependency free, choosing to lean on declarative
//! macros over proc macros as to not depend on things like `syn`. Additionally, everything is contained within
//! this one file, I have thorough tests, and I have added `#[deny(unsafe_code)]` for good measure.
//! Hopefully this makes this crate light enough and easily-auditable enough to be an acceptable dependency.

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

#[cfg(test)]
mod tests;

/// A PathBuf wrapper that has support for a Path DSL.
///
/// It is usable nearly identically to a PathBuf.
/// Supports [`Deref`](https://doc.rust-lang.org/stable/core/ops/trait.Deref.html) to
/// [`PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html) to cover all edge cases.
///
/// Prefer using the [`path!`](macro.path.html) macro.
///
/// See [crate documentation](index.html) for usage examples.
#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct PathDSL {
    path: PathBuf,
}

impl PathDSL {
    /// Creates a new PathDSL with a new empty PathBuf inside
    #[inline(always)]
    pub fn new() -> Self {
        PathDSL { path: PathBuf::new() }
    }

    /// Forwarder function for [`PathBuf::into_os_string`](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.into_os_string)
    #[inline(always)]
    pub fn into_os_string(self) -> OsString {
        self.path.into_os_string()
    }

    /// Forwarder function for [`PathBuf::into_boxed_path`](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.into_boxed_path)
    #[inline(always)]
    pub fn into_boxed_path(self) -> Box<Path> {
        self.path.into_boxed_path()
    }

    /// Converts this PathDSL into the underlying PathBuf
    #[inline(always)]
    pub fn into_pathbuf(self) -> PathBuf {
        self.into()
    }
}

//////////////////////////////////
// Pretending to be a Path(Buf) //
//////////////////////////////////

impl AsRef<Path> for PathDSL {
    #[inline(always)]
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl AsMut<PathBuf> for PathDSL {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }
}

impl AsRef<OsStr> for PathDSL {
    #[inline(always)]
    fn as_ref(&self) -> &OsStr {
        self.path.as_ref()
    }
}

impl Deref for PathDSL {
    type Target = PathBuf;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for PathDSL {
    #[inline(always)]
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
    #[inline(always)]
    fn from(other: &T) -> Self {
        PathDSL {
            path: PathBuf::from(other.as_ref()),
        }
    }
}

impl<T> From<&mut T> for PathDSL
where
    T: AsRef<Path> + ?Sized,
{
    #[inline(always)]
    fn from(other: &mut T) -> Self {
        PathDSL {
            path: PathBuf::from(other.as_ref()),
        }
    }
}

impl From<PathBuf> for PathDSL {
    #[inline(always)]
    fn from(other: PathBuf) -> Self {
        PathDSL { path: other }
    }
}

impl From<OsString> for PathDSL {
    #[inline(always)]
    fn from(other: OsString) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<String> for PathDSL {
    #[inline(always)]
    fn from(other: String) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<Box<Path>> for PathDSL {
    #[inline(always)]
    fn from(other: Box<Path>) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<Cow<'_, Path>> for PathDSL {
    #[inline(always)]
    fn from(other: Cow<'_, Path>) -> Self {
        PathDSL {
            path: PathBuf::from(other),
        }
    }
}

impl From<Cow<'_, OsStr>> for PathDSL {
    #[inline(always)]
    fn from(other: Cow<'_, OsStr>) -> Self {
        PathDSL {
            path: PathBuf::from(&*other),
        }
    }
}

//////////
// Into //
//////////
// We can't implement from on these types, so the best we can do is Into.

impl Into<PathBuf> for PathDSL {
    #[inline(always)]
    fn into(self) -> PathBuf {
        self.path
    }
}

impl Into<OsString> for PathDSL {
    #[inline(always)]
    fn into(self) -> OsString {
        self.into_os_string()
    }
}

impl Into<Box<Path>> for PathDSL {
    #[inline(always)]
    fn into(self) -> Box<Path> {
        self.into_boxed_path()
    }
}

impl<'a> Into<Cow<'a, Path>> for PathDSL {
    #[inline(always)]
    fn into(self) -> Cow<'a, Path> {
        self.path.into()
    }
}

impl<'a> Into<Cow<'a, Path>> for &'a PathDSL {
    #[inline(always)]
    fn into(self) -> Cow<'a, Path> {
        Cow::Borrowed(self.path.as_path())
    }
}

impl<'a> Into<Cow<'a, OsStr>> for &'a PathDSL {
    #[inline(always)]
    fn into(self) -> Cow<'a, OsStr> {
        Cow::Borrowed(self.path.as_os_str())
    }
}

impl<'a> Into<Arc<Path>> for PathDSL {
    #[inline(always)]
    fn into(self) -> Arc<Path> {
        self.path.into()
    }
}

impl<'a> Into<Rc<Path>> for PathDSL {
    #[inline(always)]
    fn into(self) -> Rc<Path> {
        self.path.into()
    }
}

////////////////
// Partial Eq //
////////////////

impl PartialEq<PathDSL> for PathDSL {
    #[inline(always)]
    fn eq(&self, other: &PathDSL) -> bool {
        self.path == other.path
    }
}

impl PartialEq<PathBuf> for PathDSL {
    #[inline(always)]
    fn eq(&self, other: &PathBuf) -> bool {
        self.path == *other
    }
}

impl PartialEq<Path> for PathDSL {
    #[inline(always)]
    fn eq(&self, other: &Path) -> bool {
        self.path.as_path() == other
    }
}

impl PartialEq<OsStr> for PathDSL {
    #[inline(always)]
    fn eq(&self, other: &OsStr) -> bool {
        self.path.as_path() == other
    }
}

impl PartialEq<OsString> for PathDSL {
    #[inline(always)]
    fn eq(&self, other: &OsString) -> bool {
        self.path.as_path() == other
    }
}

impl<'a> PartialEq<Cow<'a, Path>> for PathDSL {
    #[inline(always)]
    fn eq(&self, other: &Cow<'a, Path>) -> bool {
        self.path.as_path() == other
    }
}

impl<'a> PartialEq<Cow<'a, OsStr>> for PathDSL {
    #[inline(always)]
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
    #[inline(always)]
    fn partial_cmp(&self, other: &PathDSL) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl PartialOrd<PathBuf> for PathDSL {
    #[inline(always)]
    fn partial_cmp(&self, other: &PathBuf) -> Option<Ordering> {
        self.path.partial_cmp(other)
    }
}

impl PartialOrd<Path> for PathDSL {
    #[inline(always)]
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other)
    }
}

impl<'a> PartialOrd<Cow<'a, Path>> for PathDSL {
    #[inline(always)]
    fn partial_cmp(&self, other: &Cow<'a, Path>) -> Option<Ordering> {
        self.path.as_path().partial_cmp(other)
    }
}

impl<'a> PartialOrd<Cow<'a, OsStr>> for PathDSL {
    //noinspection RsTypeCheck
    #[inline(always)]
    fn partial_cmp(&self, other: &Cow<'a, OsStr>) -> Option<Ordering> {
        self.path.as_path().partial_cmp(&*other)
    }
}

impl PartialOrd<OsStr> for PathDSL {
    //noinspection RsTypeCheck
    #[inline(always)]
    fn partial_cmp(&self, other: &OsStr) -> Option<Ordering> {
        self.path.as_path().partial_cmp(&*other)
    }
}

impl PartialOrd<OsString> for PathDSL {
    //noinspection RsTypeCheck
    #[inline(always)]
    fn partial_cmp(&self, other: &OsString) -> Option<Ordering> {
        self.path.as_path().partial_cmp(&*other)
    }
}

/////////
// Ord //
/////////

impl Ord for PathDSL {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

/////////////
// FromStr //
/////////////

impl FromStr for PathDSL {
    type Err = Infallible;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s).map(|path| PathDSL { path })
    }
}

//////////
// Hash //
//////////

impl Hash for PathDSL {
    #[inline(always)]
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
    #[inline(always)]
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

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.path.as_path().iter()
    }
}

/////////////
// Default //
/////////////

impl Borrow<Path> for PathDSL {
    #[inline(always)]
    fn borrow(&self) -> &Path {
        self.path.borrow()
    }
}

/////////
// Div //
/////////

impl Div<PathDSL> for PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(mut self, rhs: PathDSL) -> Self::Output {
        if self.path.as_os_str().is_empty() {
            rhs
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

    #[inline(always)]
    fn div(mut self, rhs: &T) -> Self::Output {
        self.path.push(rhs.as_ref());
        self
    }
}

impl<T> Div<&mut T> for PathDSL
where
    T: AsRef<Path> + ?Sized,
{
    type Output = PathDSL;

    #[inline(always)]
    fn div(mut self, rhs: &mut T) -> Self::Output {
        self.path.push(rhs.as_ref());
        self
    }
}

impl Div<OsString> for PathDSL {
    type Output = PathDSL;

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn div(mut self, rhs: Box<Path>) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Div<Cow<'_, Path>> for PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(mut self, rhs: Cow<'_, Path>) -> Self::Output {
        self.path.push(rhs);
        self
    }
}

impl Div<Cow<'_, OsStr>> for PathDSL {
    type Output = PathDSL;

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn div(self, rhs: &T) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs.as_ref());
        new_self
    }
}

impl<T> Div<&mut T> for &PathDSL
where
    T: AsRef<Path> + ?Sized,
{
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: &mut T) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs.as_ref());
        new_self
    }
}

impl Div<OsString> for &PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: OsString) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<String> for &PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: String) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<PathBuf> for &PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: PathBuf) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Box<Path>> for &PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Box<Path>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Cow<'_, Path>> for &PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Cow<'_, Path>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Cow<'_, OsStr>> for &PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Cow<'_, OsStr>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

//////////////
// Div &mut //
//////////////

impl Div<PathDSL> for &mut PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: PathDSL) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl<T> Div<&T> for &mut PathDSL
where
    T: AsRef<Path> + ?Sized,
{
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: &T) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs.as_ref());
        new_self
    }
}

impl<T> Div<&mut T> for &mut PathDSL
where
    T: AsRef<Path> + ?Sized,
{
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: &mut T) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs.as_ref());
        new_self
    }
}

impl Div<OsString> for &mut PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: OsString) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<String> for &mut PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: String) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<PathBuf> for &mut PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: PathBuf) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Box<Path>> for &mut PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Box<Path>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Cow<'_, Path>> for &mut PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Cow<'_, Path>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

impl Div<Cow<'_, OsStr>> for &mut PathDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Cow<'_, OsStr>) -> Self::Output {
        let mut new_self = (*self).clone();
        new_self.path.push(rhs);
        new_self
    }
}

/////////////////
// CopylessDSL //
/////////////////

/// Implementation struct for the no-copy optimization. Should not ever
/// be found in user code.
#[derive(Default)]
#[doc(hidden)]
pub struct CopylessDSL;

impl CopylessDSL {
    /// Creates a new empty CopylessDSL
    #[doc(hidden)]
    #[inline(always)]
    pub fn new() -> CopylessDSL {
        CopylessDSL
    }
}

impl Into<PathDSL> for CopylessDSL {
    #[inline(always)]
    fn into(self) -> PathDSL {
        PathDSL::new()
    }
}

impl Into<PathBuf> for CopylessDSL {
    #[inline(always)]
    fn into(self) -> PathBuf {
        PathBuf::new()
    }
}

impl Div<PathDSL> for CopylessDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: PathDSL) -> Self::Output {
        rhs
    }
}

impl<T> Div<&T> for CopylessDSL
where
    T: AsRef<Path> + ?Sized,
{
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: &T) -> Self::Output {
        PathDSL::from(rhs)
    }
}

impl<T> Div<&mut T> for CopylessDSL
where
    T: AsRef<Path> + ?Sized,
{
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: &mut T) -> Self::Output {
        PathDSL::from(rhs)
    }
}

impl Div<OsString> for CopylessDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: OsString) -> Self::Output {
        PathDSL::from(rhs)
    }
}

impl Div<String> for CopylessDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: String) -> Self::Output {
        PathDSL::from(rhs)
    }
}

impl Div<PathBuf> for CopylessDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: PathBuf) -> Self::Output {
        PathDSL::from(rhs)
    }
}

impl Div<Box<Path>> for CopylessDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Box<Path>) -> Self::Output {
        PathDSL::from(rhs)
    }
}

impl Div<Cow<'_, Path>> for CopylessDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Cow<'_, Path>) -> Self::Output {
        PathDSL::from(rhs)
    }
}

impl Div<Cow<'_, OsStr>> for CopylessDSL {
    type Output = PathDSL;

    #[inline(always)]
    fn div(self, rhs: Cow<'_, OsStr>) -> Self::Output {
        PathDSL::from(&*rhs)
    }
}

#[cfg(windows)]
#[doc(hidden)]
#[macro_export]
macro_rules! separator {
    () => {
        "\\"
    };
}

#[cfg(not(windows))]
#[doc(hidden)]
#[macro_export]
macro_rules! separator {
    () => {
        "/"
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! concat_separator {
    ( $e:literal, $($other:literal),+ ) => {
        concat!($e, $crate::separator!(), $crate::concat_separator!($($other),+))
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

/// Efficient macro for creating a `PathBuf`.
///
/// General usage documentation is available at the [crate root](index.html). The following is
/// documentation of the optimizations made and internal implementation details.
///
/// # Expansion
///
/// The macro is a fairly simple forwarding macro that just matches against the `|` syntax specified
/// and forwards it on to the `Div` based implementation. However it does do some small optimizations,
/// including the use of a hidden type called `CopylessDSL` which allows for the
/// zero-copy optimization be guarenteed. A single `/` operation on `CopylessDSL` always
/// produces a `PathDSL`.
///
/// Some example expansions (on a unix-like system):
///
/// ```rust
/// # use path_dsl::{CopylessDSL, PathDSL, path};
/// # use std::path::PathBuf;
/// # let ret1 =
/// path!("concat" | "optimization");
/// # let res2 =
/// Into::<PathBuf>::into(CopylessDSL::new() / "concat/optimization");
/// # assert_eq!(ret1, res2);
/// ```
///
/// ```rust
/// # use path_dsl::{CopylessDSL, PathDSL, path};
/// # use std::path::PathBuf;
/// // Steals the data from `owning_path`
/// # let owning_path = PathBuf::new();
/// # let ret1 =
/// path!(owning_path | "concat" | "optimization");
/// # let owning_path = PathBuf::new();
/// # let res2 =
/// Into::<PathBuf>::into(CopylessDSL::new() / owning_path / "concat/optimization");
/// # assert_eq!(ret1, res2);
/// ```
///
/// ```rust
/// # use path_dsl::{CopylessDSL, PathDSL, path};
/// # use std::path::PathBuf;
/// # let owning_path = PathBuf::new();
/// // Copies the data from `owning_path` because we already have a buffer
/// # let ret1 =
/// path!("concat" | "optimization" | owning_path | "other_thing");
/// # let owning_path = PathBuf::new();
/// # let res2 =
/// Into::<PathBuf>::into(CopylessDSL::new() / "concat/optimization" / owning_path / "other_thing");
/// # assert_eq!(ret1, res2);
/// ```
///
/// # String Literal Concatenation
///
/// One of the optimizations made in the macro is the correct concatenation of multiple string literals in a row, as
/// shown above. This is normally not recommended because there are situations where a path with `/`
/// will not work on a windows machine. To get around this, I have first actually verified that `\\`
/// only happens on windows with a ripgrep of the rust codebase (master as of 2019-08-13):
///
/// ```text
/// $ rg "MAIN_SEP: .*="
/// rust\src\libstd\sys\sgx\path.rs
/// 19:pub const MAIN_SEP: char = '/';
///
/// rust\src\libstd\sys\unix\path.rs
/// 19:pub const MAIN_SEP: char = '/';
///
/// rust\src\libstd\sys\wasi\path.rs
/// 19:pub const MAIN_SEP: char = '/';
///
/// rust\src\libstd\sys\vxworks\path.rs
/// 19:pub const MAIN_SEP: char = '/';
///
/// rust\src\libstd\sys\wasm\path.rs
/// 19:pub const MAIN_SEP: char = '/';
///
/// rust\src\libstd\sys\windows\path.rs
/// 93:pub const MAIN_SEP: char = '\\';
/// ```
///
/// I then have an internal macro that I define multiple times using `#[cfg(windows)]` etc. to always
/// give me the correct separator no matter the platform.
///
/// Additionally, due to either my inability to write macros well, or an inherent limitation in rust's
/// declarative macros, I can't match on a set of `|` separated string literals variadically. As a result
/// I have unrolled the combiner out to 16 string literals in a row. This should be enough for basically
/// everyone. If you go above this limit, it will combine them into `ceil(N/16)` literals not 1. If you
/// need this limit raised, feel free to submit a PR or an issue, but... why? 😃
///
/// # CopylessDSL
///
/// `CopylessDSL` is a `#[doc(hidden)]` class that aids in the zero-copy optimization. It is a very limited
/// form of `PathDSL` that supports `Div` on all types `PathDSL` supports. It will steal the buffer of any
/// moved in owning values. All `Div` operations return a `PathDSL`. Additionally all macro invocations are
/// surrounded by a forced conversion to a `PathDSL` so this type should never be seen in user code.
///
/// If this type shows up in user code at all, this is a bug and should be reported.
#[macro_export]
macro_rules! path {
    ( $($other:tt)* ) => {
         ::std::convert::Into::<std::path::PathBuf>::into($crate::path_impl!( @($crate::CopylessDSL::new())@ $($other)* ));
    };
    () => {  $crate::PathDSL::new() };
}
