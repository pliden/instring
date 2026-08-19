use instring::InString;
use instring::Intern;

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
fn from_str() {
    let foo = "FOO";
    let _ = InString::from(foo);
}

#[test]
fn from_string() {
    let foo = String::from("FOO");
    let _ = InString::from(foo);
}

#[test]
fn as_str() {
    fn print_as_str(string: &str) {
        println!("{}", string);
    }

    let foo = "FOO".intern();
    print_as_str(&foo);
}

#[test]
fn as_string() {
    fn print_as_string(string: &String) {
        println!("{}", string);
    }

    let foo = "FOO".intern();
    print_as_string(&foo);
}

#[test]
fn as_ref() {
    fn print_as_ref(string: impl AsRef<str>) {
        println!("{}", string.as_ref());
    }

    let foo = "FOO".intern();
    print_as_ref(&foo);
}

#[test]
fn partial_eq_str() {
    let s = "FOO".intern();
    let foo = "FOO";
    let bar = "BAR";
    assert!(s == foo);
    assert!(s != bar);
}

#[test]
fn partial_eq_string() {
    let s = "FOO".intern();
    let foo = String::from("FOO");
    let bar = String::from("BAR");
    assert!(s == foo);
    assert!(s != bar);
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
fn all() {
    let foo = "FOO".intern();
    let bar = "BAR".intern();

    let all = InString::all().collect::<Vec<_>>();

    assert!(all.len() >= 2);
    assert!(all.contains(&foo));
    assert!(all.contains(&bar));
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
