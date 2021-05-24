/*
This file contains the dynamic dispatch version. Static version was shown in Ch10 and also here:
https://stackoverflow.com/questions/67445196/transform-static-dispatch-code-into-dynamic-dispatch
 */

trait AsInt {
    fn as_int(&self) -> i32;
    fn box_clone<'a>(&'a self) -> Box<dyn AsInt + 'a>;
}

impl AsInt for i32 {
    fn as_int(&self) -> i32 {
        *self
    }

    fn box_clone(&self) -> Box<dyn AsInt> {
        Box::new(*self)
    }
}

impl AsInt for &str {
    fn as_int(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }

    fn box_clone<'a>(&'a self) -> Box<dyn AsInt + 'a> {
        Box::new(*self) //&str
    }
}

fn largest_dyn<'a>(list: &'a [Box<dyn AsInt + 'a>]) -> Box<dyn AsInt + 'a> {
    let mut largest = list[0].box_clone();
    for i in list {
        if i.as_int() > largest.as_int() {
            largest = i.box_clone();
        }
    }
    largest
}

fn main() {
    let v: Vec<Box<dyn AsInt>> = vec![10, 2, 35, -8, 1]
        .into_iter()
        .map(|n| Box::new(n) as _)
        .collect();
    assert!(largest_dyn(&v).as_int() == 35);

    println!("largest int is {}", largest_dyn(&v).as_int());
    println!("largest int is {}", largest_dyn(&v[..]).as_int());

    //now try with strings
    let v_str: Vec<Box<dyn AsInt>> = vec!["10", "2", "35", "-8", "1"]
        .into_iter()
        .map(|n| Box::new(n) as _)
        .collect();

    println!("largest str is {}", largest_dyn(&v_str).as_int());
    println!("largest str is {}", largest_dyn(&v_str[..]).as_int());
}
