use std::fmt::Display;
use std::convert::AsRef;
use std::ffi::{CStr, OsStr};

#[derive(Debug)]
struct Foo;

fn main() {
    //Example from here - https://riptutorial.com/rust/example/9458/unwrapping-a-reference-to-an-option-owning-its-contents
    let wrapped = Some(Foo);
    let wrapped_ref = &wrapped;
    // println!("{:?}", wrapped.unwrap()); // Works - coz we're simply unwrapping an option
    // println!("{:?}", wrapped_ref.unwrap()); // Error - coz we're trying to unwrap a borrowed option
    let foo_ref = wrapped_ref.as_ref().unwrap(); // Transforms &Option<T> into &Option<&T>, then unwraps into &T
    println!("{:?}", foo_ref); //&T
    println!("{:?}", *foo_ref); //T

    //try with a &str
    let x = "123";
    let y = Some(x); //we must store this as an interim value, or we get this error -> https://stackoverflow.com/questions/28469667/borrowed-value-does-not-live-long-enough-when-using-the-builder-pattern
    let z = &y.as_ref().unwrap();
    println!("{}", z);

    //try w/o option
    let x = "123";
    let z: &&str = &x.as_ref(); //works as long as we give z an explicit type
    println!("{}", z);

    // -----------------------------------------------------------------------------
    // https://www.youtube.com/watch?v=iKFljZP6JD0

    // ok so the whole idea is that as_ref converts one type of ref into another type of ref
    let s = "555"; // &str
    let s2 = String::from(s); //String

    //allows us to accept both String and &str into this function as argument]
    fn print_as_string<T: Display + AsRef<str>>(s: T) {
        println!("{:?}", s.as_ref());
    }

    fn print_as_bytes<T: Display + AsRef<[u8]>>(s: T) {
        println!("{:?}", s.as_ref());
    }

    fn print_as_OsStr<T: Display + AsRef<OsStr>>(s: T) {
        println!("{:?}", s.as_ref());
    }

    // print_as_string(s);
    // print_as_string(s2);

    // print_as_bytes(s);
    // print_as_bytes(s2);

    print_as_OsStr(s);
    print_as_OsStr(s2);

    // -----------------------------------------------------------------------------
    // https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust
    // take a ref to self and return a ref to type T. so eg take &Option(T), return &Option(&T)
    // makes most sense when SourceT = wrapper around TargetT

    //ints
    struct WrapperOnInt(i32); //tuple struct
    impl AsRef<i32> for WrapperOnInt {
        fn as_ref(&self) -> &i32 {
            &self.0
        }
    }
    let w = WrapperOnInt(777);
    let w_ref = w.as_ref();
    println!("{}", w_ref); //&i32
    println!("{}", *w_ref); //proves it's a &i32

    //String
    struct WrapperOnString(String);
    impl AsRef<String> for WrapperOnString {
        fn as_ref(&self) -> &String {
            &self.0
        }
    }
    let s = WrapperOnString(String::from("123123"));
    let s_ref = s.as_ref();
    println!("{}", s_ref);
    println!("{}", *s_ref);

    //vec
    struct WrapperOnVec(Vec<i32>);
    impl AsRef<Vec<i32>> for WrapperOnVec {
        fn as_ref(&self) -> &Vec<i32> {
            &self.0
        }
    }
    let v = WrapperOnVec(vec![1,2,3]);
    let v_ref = v.as_ref();
    println!("{:?}", v_ref);
    println!("{:?}", *v_ref);

}

