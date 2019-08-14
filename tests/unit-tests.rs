use path_dsl::{path, PathDSL};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::borrow::Cow;

macro_rules! dsl_test {
    ($(first: $first:path,)? $(second: ($($conv:tt)+),)? $(self: ($($selfmod:tt)+),)? name: $id:ident) => {
        #[allow(unused)]
        #[test]
        fn $id() {
            let mut first = $($first)?("ident");
            let second = $($($conv)+)?(first);
            let res_dsl = $($($selfmod)+)?PathDSL::new() / second / "my_file";

            let mut first = $($first)?("ident");
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
    ($(first: $first:path,)? name: $name:ident) => {
        paste::item!(dsl_test!{$(first: $first,)? name: [<dsl_ $name>]});
        paste::item!(dsl_test!{$(first: $first,)? second: (&), name: [<dsl_ $name _ref>]});
        paste::item!(dsl_test!{$(first: $first,)? second: (&mut), name: [<dsl_ $name _ref_mut>]});

        paste::item!(dsl_test!{$(first: $first,)? self: (&), name: [<dsl_ref_ $name>]});
        paste::item!(dsl_test!{$(first: $first,)? second: (&), self: (&), name: [<dsl_ref_ $name _ref>]});
        paste::item!(dsl_test!{$(first: $first,)? second: (&mut), self: (&), name: [<dsl_ref_ $name _ref_mut>]});

        paste::item!(dsl_test!{$(first: $first,)? self: (&mut), name: [<dsl_ref_mut_ $name>]});
        paste::item!(dsl_test!{$(first: $first,)? second: (&), self: (&mut), name: [<dsl_ref_mut_ $name _ref>]});
        paste::item!(dsl_test!{$(first: $first,)? second: (&mut), self: (&mut), name: [<dsl_ref_mut_ $name _ref_mut>]});
    };
}

owned_dsl_test!(first: OsStr::new, name: osstr);
owned_dsl_test!(first: OsString::from, name: osstring);
owned_dsl_test!(name: str);
owned_dsl_test!(first: String::from, name: string);
owned_dsl_test!(first: Path::new, name: path);
owned_dsl_test!(first: PathBuf::from, name: pathbuf);
owned_dsl_test!(first: PathDSL::from, name: dsl);
owned_dsl_test!(first: gen_box_path, name: box_path);
owned_dsl_test!(first: gen_cow_path, name: cow_path);
owned_dsl_test!(first: gen_cow_osstr, name: arc_osstr);

fn gen_box_path(p: &str) -> Box<Path> {
    Box::from(Path::new(p))
}
fn gen_cow_path(p: &str) -> Cow<Path> {
    Cow::from(Path::new(p))
}
fn gen_cow_osstr(p: &str) -> Cow<OsStr> {
    Cow::from(OsStr::new(p))
}
