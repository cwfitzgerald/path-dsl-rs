# path-dsl

DSL PathBuf Wrapper and Macro for easy creation of paths.

PathBuf (and Path) give us a cross platform way to handle paths,
but when you are creating part a path, you often want to
use a raw string or a formatted string to express that. While
this is significantly more terse, it has cross platform issues
because of the slash use in the string. Enter PathDSL.

#### **Incorrect:** String

This is an easy but incorrect way of creating a path.

```rust
use std::path::PathBuf;
// Fails on windows when put onto the end of an absolute path
let path = PathBuf::from("dir1/dir2/dir3/file.txt");
```

#### PathBuf API

This is a correct but extremely verbose and mutable way of creating a path.
It is possible to mitigate the mutability by making a mutable path inside a block
and then assigning the result of the block to an immutable variable, but that increases
the amount of code.

```rust
use std::path::PathBuf;
let mut path = PathBuf::new();
path.push("dir1");
path.push("dir2");
path.push("dir3");
path.push("file.txt");
```

## PathDSL Macro

Compare with PathDSL's [`path!`](https://docs.rs/path-dsl/*/path_dsl/macro.path.html) macro (note the use of `|` instead of `/` due to rust's macro rules).
PathDSL is a drop-in replacement for PathBuf and is easily and cheaply convertible back and forth. This
macro has a couple optimizations over just using the PathDSL class manually, described later. It is
recommended to always use the macro when using the DSL.

```rust
use path_dsl::{path, PathDSL};
// Type annotation for illustration only, not needed
let path: PathDSL = path!("dir1" | "dir2" | "dir3" | "file.txt");
```

#### PathDSL

You can also generate a PathDSL directly, though this is discouraged.

```rust
use path_dsl::PathDSL;
let path = PathDSL::from("dir1") / "dir2" / "dir3" / "file.txt";
```

#### Adding Path-Like Structures

As well as using regular string literals, you can use anything that can be passed to `PathBuf::push`
as a part of the DSL.

Note the borrow on `other`: as these types are not `Copy`, they will be moved
into the path unless you borrow them. This matches behavior with `PathBuf::push`, but can be surprising
when used in a infix expression.

```rust
use path_dsl::{path, PathDSL};

let other = PathBuf::from("some_dir");
let filename: &str = "my_file.txt";

let path = PathDSL::from("dir1") / "dir2" / &other / filename;
let mac  = path!("dir1" | "dir2" | other | filename);
```

#### Moving vs Borrowing

Both the macro and the DSL type behave the same with regard to borrowing vs moving. If a
reference is provided, it will borrow the provided value. However, if a value is provided
**it will move it**, making the value unusable afterwards. While these are the normal rules
for rust, infix operators are normally used with `Copy` types, so this may be **surprising**.

Both mutable and immutable borrows are supported, though they will never actually mutate anything.

```rust,compile_fail
use path_dsl::{path, PathDSL};
# use std::path::PathBuf;

let value = PathBuf::from("some_dir");
let borrow: &str = "my_file.txt";

let path = PathDSL::new() / value / borrow;
let mac  = path!(value | borrow); // Will not compile because `value` was moved
```

You must manually borrow it:

```rust
use path_dsl::{path, PathDSL};

let value = PathBuf::from("some_dir");
let borrow: &str = "my_file.txt";

let path = PathDSL::new() / &value / borrow; // Borrow value so it can be used later
let mac  = path!(value | borrow); // Not used afterwards, so doesn't need a borrow
```

#### PathDSL <=> PathBuf

`PathDSL` is designed to be a drop-in replacement for `PathBuf`, including trivial conversions
between the two. In any situation where you would be able to use `PathBuf` you can use
`PathDSL`. However there are some situations where you must have a `PathBuf`. Obtaining it
is trivial through dereferencing or through the `PathDSL::into_pathbuf` function.

PathDSL is `#[repr(transparent)]` over `PathBuf` and all functions are force-inlined so
conversions and operations should be cost-free compared to the equivalent `PathBuf` operation.
If they aren't, please file a bug.

Some known issues are:

**Equality**

```rust
use path_dsl::path;

let dsl = path!("file.txt");
let buf = PathBuf::from("file.txt");

assert!(dsl == buf);
// Must de-reference to PathBuf can't implement `Eq` for `PathBuf`
assert!(buf == *dsl);
```

**Function Calls**

```rust
use path_dsl::path;

fn func(p: PathBuf) {
}

let dsl = path!("file.txt");
let buf = PathBuf::from("file.txt");

func(buf);
// Must convert into `PathBuf`
// Dereferencing doesn't work because `func` moves.
func(dsl.into_pathbuf());
// func(dsl.into()) also works
```

#### Macro Optimizations

As previously mentioned, the macro contains some optimizations over using raw `PathDSL` and should always
be used over manually using PathDSL. These optimizations happen at compile time, and are guaranteed.
Further details on these can be found on the [`path!`](https://docs.rs/path-dsl/*/path_dsl/macro.path.html) macro documentation.

**String Literal Concatenation:**

While it is ill-advised to use string literals with slashes in a `Path`, The [`path!`](https://docs.rs/path-dsl/*/path_dsl/macro.path.html) macro
takes slashes into account, and automatically constructs a single string literal from multiple
consecutive string literals. This can potentially save an allocation or two in the underlying
`OsString`.

```rust
use path_dsl::path;

let p = path!("this" | "is" | "combined");
if cfg!(windows) {
    assert_eq!(p, PathBuf::from("this\\is\\combined"));
} else {
    assert_eq!(p, PathBuf::from("this/is/combined"));
}
```

**First-Argument Optimization:**

When the very first argument of the [`path!`](https://docs.rs/path-dsl/*/path_dsl/macro.path.html) macro is a owning `PathBuf`, `OsString` or `PathDSL`
passed by value (moved), instead of copying everything into a new `PathDSL`, it will just steal the
buffer from that moved-in value. This allows you to use the [`path!`](https://docs.rs/path-dsl/*/path_dsl/macro.path.html) macro fearlessly when appending
to already existing variables.

```rust
use path_dsl::path;

let first = PathBuf::from("a_very_long_folder_name");
let p = path!(first); // Does not copy anything.
```

## Known Issues

Due to my mitigation of a [rustc bug](https://github.com/rust-lang/rust/issues/63460) there may be
issues when renaming `path_dsl` crate and using the [`path!`](https://docs.rs/path-dsl/*/path_dsl/macro.path.html) macro. I currently have not have experienced this,
but if it happens, please report an issue and I'll add it to the documentation.

## Why Use A Crate?

You may be wondering why you should use a crate for this when you can easily wrap `PathBuf` and
add some `Div` implementations. This is basically what I thought as well until I actually went
to go implement this crate. There is a surprising amount of very tedious and particular code to try to emulate
`PathBuf` directly, as well as to test the functionality.

With this in mind, I have made `path_dsl` completely dependency free, choosing to lean on declarative
macros over proc macros as to not depend on things like `syn`. Additionally, everything is contained within
this one file, and I have added `#[deny(unsafe_code)]` for good measure. Hopefully this makes this crate light
enough and easily-auditable enough to be an acceptable dependency.

License: MIT
