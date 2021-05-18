use std::io;
use std::convert::TryFrom;
use std::io::ErrorKind;

pub fn main() {
    // idiomatic way to do type conversions in rust is to use the traits in the std::convert module —
    // From<T>, Into<U>, TryFrom<T>, TryInto<U>, AsRef<U>, and AsMut<U>
    // https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust

    // -----------------------------------------------------------------------------
    // From<SourceT> -> TargetT
    //  -consumes argument and returns Self
    //  -conversion always succeeds (can't fail once compiled)
    //  -is reflexive (T > T works)
    let s_str = "hello world"; //&str
    let s_String = String::from(s_str); //String

    // Into<Target T> <- SourceT
    //  -consumes self and returns argument type
    //  -conversion always succeeds (can't fail once compiled)
    //  -is reflexive (T > T works)
    let s_String:String = s_str.into(); //String
    // SO THIS DOES EXACTLY THE SAME AS LINE 11. THE POINT IS, IF String HAS FROM() METHOD IT ALSO HAS INTO() METHOD. THEY DO THE SAME THING.

    //But it doesn't mean that &str will have from()/into() methods. Eg the below fail:
    // let s_str = std::str::From(s_String);
    // let s_str: &str = s_String.into();
    // let s_str: &str = Into::into(s_String);

    //Then there are types that have it both ways:
    let s_Boxed = s_String.into_boxed_str(); //Box<str>, which is kinda like &str except we're using a smart pointer instead of a normal pointer
    let s_String = String::from(s_Boxed); //String again

    //We can implement our own custom types

    // A) won't work - i'm trying to implement an external trait for a external object
    // B) we don't implement Into, we implement From and Into gets implemented automatically
    // impl Into<&str> for String {
    //     fn into(self) -> &'static str {
    //         &self[..]
    //     }
    // }

    // This still won't work for same reason as A) above
    // impl From<String> for &str {
    //     fn from(self) -> String {
    //         String::from(self)
    //     }
    // }

    //but we can do a custom type
    struct MyString {
        content: String,
    }

    impl From<String> for MyString {
        fn from(s: String) -> Self {
            Self {
                content: s
            }
        }
    }

    let s_Custom = MyString::from(s_String); //MyString
    let s_String2 = String::from("asdf");
    let s_Custom2:MyString = s_String2.into(); //MyString

    // -----------------------------------------------------------------------------
    // TryFrom, TryInto
    // like above, but can fail and hence return a Result<TargetT, ErrorT>

    //let's say only 3 packet types exist
    enum PacketType {
        Data = 0,
        Fin = 1,
        State = 3,
    }

    //but we want to be able to call PacketType::from(<u8>) on any interger
    //we need a FALLIBLE from
    impl TryFrom<u8> for PacketType {
        type Error = io::Error;
        fn try_from(x:u8) -> Result<Self, Self::Error> {
            match x {
                0 => Ok(Self::Data),
                1 => Ok(Self::Fin),
                2 => Ok(Self::State),
                n => Err(io::Error::new(ErrorKind::Other, "oh no!")),
            }
        }
    }

    //now let's try!
    let good_packet = PacketType::try_from(1).unwrap();
    // let bad_packet = PacketType::try_from(55).unwrap(); //panic!

    // -----------------------------------------------------------------------------
    // use From/Into to broaden fn arg types

    fn takes_int(x: i32) {
        println!("I am indeed an int: {}", x);
    }

    fn takes_anything_that_converts_to_int<T: Into<i32>>(x: T) {
        let x_converted: i32 = x.into();
        takes_int(x_converted);
    }

    //this works because i8 implements from/into functionality for i32 - https://doc.rust-lang.org/std/primitive.i32.html#method.from
    takes_anything_that_converts_to_int(123i8);

}

