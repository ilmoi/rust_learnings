use std::fmt;
use std::cmp::Ordering;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

#[derive(Debug)]
enum IntsAndStrs {
    Int(i32),
    Str(&'static str),
}

impl fmt::Display for IntsAndStrs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for IntsAndStrs {
    fn eq(&self, other: &Self) -> bool {
        let my_val = match self {
            IntsAndStrs::Int(x) => *x,
            IntsAndStrs::Str(y) => y.parse::<i32>().unwrap(),
        };
        let their_val = match other {
            IntsAndStrs::Int(x) => *x,
            IntsAndStrs::Str(y) => y.parse::<i32>().unwrap(),
        };
        my_val == their_val
    }
}

impl PartialOrd for IntsAndStrs {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let my_val = match self {
            IntsAndStrs::Int(x) => *x,
            IntsAndStrs::Str(y) => y.parse::<i32>().unwrap(),
        };
        let their_val = match other {
            IntsAndStrs::Int(x) => *x,
            IntsAndStrs::Str(y) => y.parse::<i32>().unwrap(),
        };
        Some(my_val.cmp(&their_val))
    }
}

impl Copy for IntsAndStrs {}

impl Clone for IntsAndStrs {
    fn clone(&self) -> Self {
        *self
    }
}


fn main() {
    // int
    let v_int = vec![1,2,3];
    println!("largest int is {}", largest(&v_int));

    // str
    let v_str = vec!["1","2","3"];
    println!("largest str is {}", largest(&v_str));

    // custom type
    let fake_int = IntsAndStrs::Int(1);
    let fake_str = IntsAndStrs::Str("2");
    let v = vec![fake_int, fake_str];
    println!("largest int/str is {}", largest(&v));
}

