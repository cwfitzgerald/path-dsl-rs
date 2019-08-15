use path_dsl::{path, PathDSL};
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

macro_rules! dsl_test {
    ($(constructor: $constructor:path,)? $(converter: ($($conv:tt)+),)? $(self: ($($selfmod:tt)+),)? name: $id:ident) => {
        #[allow(unused)]
        #[test]
        fn $id() {
            let mut first = $($constructor)?("ident");
            let second = $($($conv)+)?(first);
            let res_dsl = $($($selfmod)+)?PathDSL::new() / second / "my_file";

            let mut first = $($constructor)?("ident");
            let second = $($($conv)+)?(first);
            let res_macro = path!(second | "my_file");

            let mut real = PathBuf::new();
            real.push("ident");
            real.push("my_file");

            assert_eq!(res_dsl, real);
            assert_eq!(res_macro, real);
        }
    };
}

macro_rules! owned_dsl_test {
    ($(constructor: $constructor:path,)? name: $name:ident) => {
        paste::item!(dsl_test!{$(constructor: $constructor,)? name: [<dsl_ $name>]});
        paste::item!(dsl_test!{$(constructor: $constructor,)? converter: (&), name: [<dsl_ $name _ref>]});
        paste::item!(dsl_test!{$(constructor: $constructor,)? converter: (&mut), name: [<dsl_ $name _ref_mut>]});

        paste::item!(dsl_test!{$(constructor: $constructor,)? self: (&), name: [<dsl_ref_ $name>]});
        paste::item!(dsl_test!{$(constructor: $constructor,)? converter: (&), self: (&), name: [<dsl_ref_ $name _ref>]});
        paste::item!(dsl_test!{$(constructor: $constructor,)? converter: (&mut), self: (&), name: [<dsl_ref_ $name _ref_mut>]});

        paste::item!(dsl_test!{$(constructor: $constructor,)? self: (&mut), name: [<dsl_ref_mut_ $name>]});
        paste::item!(dsl_test!{$(constructor: $constructor,)? converter: (&), self: (&mut), name: [<dsl_ref_mut_ $name _ref>]});
        paste::item!(dsl_test!{$(constructor: $constructor,)? converter: (&mut), self: (&mut), name: [<dsl_ref_mut_ $name _ref_mut>]});
    };
}

owned_dsl_test!(constructor: OsStr::new, name: osstr);
owned_dsl_test!(constructor: OsString::from, name: osstring);
owned_dsl_test!(name: str);
owned_dsl_test!(constructor: String::from, name: string);
owned_dsl_test!(constructor: Path::new, name: path);
owned_dsl_test!(constructor: PathBuf::from, name: pathbuf);
owned_dsl_test!(constructor: PathDSL::from, name: dsl);
owned_dsl_test!(constructor: gen_box_path, name: box_path);
owned_dsl_test!(constructor: gen_cow_path, name: cow_path);
owned_dsl_test!(constructor: gen_cow_osstr, name: arc_osstr);

fn gen_box_path(p: &str) -> Box<Path> {
    Box::from(Path::new(p))
}
fn gen_cow_path(p: &str) -> Cow<Path> {
    Cow::from(Path::new(p))
}
fn gen_cow_osstr(p: &str) -> Cow<OsStr> {
    Cow::from(OsStr::new(p))
}

macro_rules! partial_ord_test {
    (owned, $lhs:expr, $rhs:expr) => {
        assert!($lhs < $rhs)
    };
    (unowned, $lhs:expr, $rhs:expr) => {
        assert!($lhs < *$rhs)
    };
    ($(constructor: $constructor:path,)? $(converter: ($($conv:tt)+),)? name: $name:ident, $ownage:tt) => {
        paste::item!{
            #[allow(unused)]
            #[test]
            fn [<partial_ord_ $name>]() {
                let lhs = PathDSL::from("aaaaa");

                let mut first = $($constructor)?("zzzzz");
                let second = $($($conv)+)?(first);

                partial_ord_test!($ownage, lhs, second);
            }
        }
    };
}

partial_ord_test!(constructor: OsStr::new, name: osstr, unowned);
partial_ord_test!(constructor: OsString::from, name: osstring, owned);
partial_ord_test!(constructor: Path::new, name: path, unowned);
partial_ord_test!(constructor: PathBuf::from, name: pathbuf, owned);
partial_ord_test!(constructor: PathDSL::from, name: dsl, owned);
partial_ord_test!(constructor: gen_box_path, name: box_path, unowned);
partial_ord_test!(constructor: gen_cow_path, name: cow_path, owned);
partial_ord_test!(constructor: gen_cow_osstr, name: cow_osstr, owned);
