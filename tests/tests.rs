use instring::InString;
use instring::Intern;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

fn is_same(a: &InString, b: &InString) -> bool {
    std::ptr::eq(a.as_str(), b.as_str())
}

#[test]
fn str_intern() {
    let foo0 = "FOO".intern();
    let bar0 = "BAR".intern();
    let foo1 = "FOO".intern();
    let bar1 = "BAR".intern();
    let bar2 = "BAR".intern();
    let bar3 = bar2.clone();

    println!("foo0: '{foo0}'");
    println!("foo1: '{foo1}'");
    println!("bar0: '{bar0}'");
    println!("bar1: '{bar1}'");
    println!("bar2: '{bar2}'");
    println!("bar3: '{bar3}'");

    assert!(is_same(&foo0, &foo1));
    assert!(is_same(&bar0, &bar1));
    assert!(is_same(&bar0, &bar2));
    assert!(is_same(&bar0, &bar3));
}

#[test]
fn string_intern() {
    let foo0 = String::from("FOO").intern();
    let bar0 = String::from("BAR").intern();
    let foo1 = String::from("FOO").intern();
    let bar1 = String::from("BAR").intern();
    let bar2 = String::from("BAR").intern();
    let bar3 = bar2.clone();

    println!("foo0: '{foo0}'");
    println!("foo1: '{foo1}'");
    println!("bar0: '{bar0}'");
    println!("bar1: '{bar1}'");
    println!("bar2: '{bar2}'");
    println!("bar3: '{bar3}'");

    assert!(is_same(&foo0, &foo1));
    assert!(is_same(&bar0, &bar1));
    assert!(is_same(&bar0, &bar2));
    assert!(is_same(&bar0, &bar3));
}

#[test]
fn as_ref_osstr() {
    fn print(string: impl AsRef<OsStr>) {
        println!("{:?}", string.as_ref());
    }

    let foo = "FOO".intern();
    print(&foo);
}

#[test]
fn as_ref_path() {
    fn print(string: impl AsRef<Path>) {
        println!("{:?}", string.as_ref());
    }

    let foo = "FOO".intern();
    print(&foo);
}

#[test]
fn as_ref_u8() {
    fn print(string: impl AsRef<[u8]>) {
        println!("{:?}", string.as_ref());
    }

    let foo = "FOO".intern();
    print(&foo);
}

#[test]
fn as_ref_str() {
    fn print(string: impl AsRef<str>) {
        println!("{:?}", string.as_ref());
    }

    let foo = "FOO".intern();
    print(&foo);
}

#[test]
fn borrow_str() {
    fn print(string: impl Borrow<str>) {
        println!("{:?}", string.borrow());
    }

    let foo = "FOO".intern();
    print(foo);
}

#[test]
fn deref_str() {
    fn print(string: &str) {
        println!("{:?}", string);
    }

    let foo = "FOO".intern();
    print(&foo);
}

#[test]
fn deref_string() {
    fn print(string: &String) {
        println!("{:?}", string);
    }

    let foo = "FOO".intern();
    print(&foo);
}

#[test]
fn from_cow() {
    let foo = Cow::Borrowed("FOO");
    let _ = InString::from(foo);

    let foo = Cow::Owned(String::from("FOO"));
    let _ = InString::from(foo);
}

#[test]
fn from_instring() {
    let foo = InString::from("FOO");
    let _ = InString::from(&foo);
}

#[test]
fn from_char() {
    let foo = 'F';
    let _ = InString::from(foo);
}

#[test]
fn from_box_str() {
    let s = String::from("FOO");
    let foo: Box<str> = s.into();
    let _ = InString::from(foo);
}

#[test]
fn from_mut_str() {
    let mut s = String::from("FOO");
    let foo = s.as_mut_str();
    let _ = InString::from(foo);
}

#[test]
fn from_str() {
    let s = String::from("FOO");
    let foo = s.as_str();
    let _ = InString::from(foo);
}

#[test]
fn from_string() {
    let foo = String::from("FOO");
    let _ = InString::from(foo);
}

#[test]
fn from_ref_string() {
    let foo = String::from("FOO");
    let _ = InString::from(&foo);
}

#[test]
fn from_str_trait() {
    let _ = InString::from_str("FOO").unwrap();
}

#[test]
fn sliece_index() {
    let foo = String::from("FOO");
    assert!(&foo[..] == "FOO");
    assert!(&foo[0..2] == "FO");
    assert!(&foo[1..3] == "OO");
}

#[test]
fn partial_eq_cow() {
    let s = "FOO".intern();
    let foo = Cow::Borrowed("FOO");
    assert!(s == foo);
}

#[test]
fn partial_eq_instring() {
    let s = "FOO".intern();
    let foo = InString::from("FOO");
    let bar = InString::from("BAR");
    assert!(s == foo);
    assert!(s != bar);
}

#[test]
fn partial_eq_str() {
    let s = "FOO".intern();
    let foo = "FOO";
    let bar = "BAR";
    assert!(s == foo);
    assert!(s != bar);
    assert!(s == *foo);
    assert!(s != *bar);
}

#[test]
fn partial_eq_string() {
    let s = "FOO".intern();
    let foo = String::from("FOO");
    let bar = String::from("BAR");
    let foo_ref = &foo;
    let bar_ref = &bar;
    assert!(s == foo);
    assert!(s != bar);
    assert!(s == foo_ref);
    assert!(s != bar_ref);
}

#[test]
fn partial_eq_path() {
    let s = "FOO".intern();
    let foo = PathBuf::from("FOO");
    let bar = PathBuf::from("BAR");
    assert!(s == *foo.as_path());
    assert!(s != *bar.as_path());
}

#[test]
fn partial_eq_pathbuf() {
    let s = "FOO".intern();
    let foo = PathBuf::from("FOO");
    let bar = PathBuf::from("BAR");
    assert!(s == foo);
    assert!(s != bar);
}

#[test]
fn all() {
    let foo = "FOO".intern();
    let bar = "BAR".intern();

    let all = InString::all().collect::<Vec<_>>();

    assert!(all.len() >= 2);
    assert!(all.contains(&foo));
    assert!(all.contains(&bar));
}

#[test]
fn ref_count() {
    let s = "UNIQUE";

    let s0 = s.intern();
    println!("{}", s0.ref_count());
    assert!(s0.ref_count() == 1);

    let s1 = s.intern();
    assert!(s1.ref_count() == 2);

    let s2 = s.intern();
    assert!(s2.ref_count() == 3);

    let s3 = s.intern();
    assert!(s3.ref_count() == 4);

    let s4 = s.intern();
    assert!(s4.ref_count() == 5);
}

#[cfg(feature = "stats")]
#[test]
fn stats() {
    let _foo0 = "FOO".intern();
    let _foo1 = "FOO".intern();
    let _bar0 = "BAR".intern();
    let _bar1 = "BAR".intern();

    let stats = InString::stats();

    assert!(stats.interned_strings >= 2);
    assert!(stats.interned_bytes >= 6);
    assert!(stats.deduped_strings >= 2);
    assert!(stats.deduped_bytes >= 6);
}
