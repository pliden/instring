use instring::InString;
use instring::Intern;

#[test]
fn intern() {
    fn is_same(a: &InString, b: &InString) -> bool {
        std::ptr::eq(a.as_str(), b.as_str())
    }

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
    let foo = "FOO".intern();
    assert!(foo == "FOO");
    assert!(foo != "BAR");
}
