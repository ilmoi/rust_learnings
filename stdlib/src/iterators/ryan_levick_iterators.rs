// https://www.youtube.com/watch?v=lQt0adYPdfQ

// -----------------------------------------------------------------------------
// Iterator + tuple struct

// pub fn main() {
//     let mv = MyVec::new(vec![1,2,3]);
//
//     for i in mv {
//         println!("{}", i);
//     }
// }
//
// struct MyVec<T>(Vec<T>, usize);
//
// impl<T: Copy> MyVec<T> {
//     fn new(v :Vec<T>) -> Self {
//         Self(v, 0)
//     }
// }
//
// impl<T: Copy> Iterator for MyVec<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         //doesn't work - no next() method on vec (vec itself doesn't impl Iterator)
//         // self.0.next()
//
//         //doesn't work - self.0 behind a mutable reference
//         // self.0.into_iter().next()
//
//         //doesn't work - same as above
//         // let mut vec_internal = self.0.iter();
//         // vec_internal.next()
//
//         //doesn't work - infinite loop
//         // self.0.clone().into_iter().next()
//
//         //doesn't work - infinite loop
//         // let mut vec_internal = self.0.iter();
//         // if let Some(x) = vec_internal.next() {
//         //     Some(*x)
//         // } else {
//         //     None
//         // }
//
//         let next_item = self.0.get(self.1)?;
//         self.1 += 1;
//         Some(*next_item)
//     }
// }

// -----------------------------------------------------------------------------
// Iterator + named struct (taking ownership)

// pub fn main() {
//     let mv2 = MyVec2::new(vec![1,2,3]);
//     for i in mv2 {
//         println!("{}", i);
//     }
// }
//
// struct MyVec2<T> {
//     counter: usize,
//     my_vec: Vec<T>,
// }
//
// impl<T: Copy> MyVec2<T> {
//     fn new(my_vec: Vec<T>) -> Self {
//         Self {
//             counter: 0,
//             my_vec,
//         }
//     }
// }
//
// impl<T: Copy> Iterator for MyVec2<T> {
//     type Item = T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let i = self.my_vec.get(self.counter)?;
//         self.counter += 1;
//         Some(*i)
//     }
// }

// -----------------------------------------------------------------------------
// Iterator + named struct (references)

// pub fn main() {
//     let mv = vec![1,2,3]; //needs to be now defined on a sep line, otherwise we're dropping it while calling MyVec2::new() on it
//     let mv2 = MyVecIterator::new(&mv);
//     for i in mv2 {
//         println!("{}", i);
//     }
// }
//
// struct MyVecIterator<'a, T> {
//     counter: usize,
//     my_vec: &'a Vec<T>, //main place where we have to make a decision - this time we're taking a ref to a vec rather than the vec itself
// }
//
// impl<'a, T> MyVecIterator<'a, T> {
//     fn new(my_vec: &'a Vec<T>) -> Self {
//         Self {
//             counter: 0,
//             my_vec,
//         }
//     }
// }
//
// impl<'a, T> Iterator for MyVecIterator<'a, T> {
//     type Item = &'a T; //we start here, by assigning a lifetime - this forces us to add it everywhere else
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let i = self.my_vec.get(self.counter)?;
//         self.counter += 1;
//         Some(i)
//     }
// }

// -----------------------------------------------------------------------------
// extending vec to be an iterator, instead of creating a new type

// /*
// we can implement the above as "trait extension" for normal Vec, and hence be able to access it directly
// this is a common pattern in rust where if you want to extend the type you don't own, this is how you'd do it
// in std lib Vec actually has an Iter trait, which does exactly the same as below, without the "my"
// this is why we can iterate over a vec by calling vec.iter()
// NOTE: we still need to impl Iterator for MyVecIterator, otherwise for loop won't work
//
// Big picture: really this .my_iter thing is performing the same job as my .new() function
// */
//
// pub fn main() {
//     let mv = vec![1,2,3]; //needs to be now defined on a sep line, otherwise we're dropping it while calling MyVec2::new() on it
//     for i in mv.my_iter() {
//         println!("{}", i);
//     }
// }
//
// struct MyVecIterator<'a, T> {
//     counter: usize,
//     my_vec: &'a Vec<T>, //main place where we have to make a decision - this time we're taking a ref to a vec rather than the vec itself
// }
//
// trait MyIter<T> {
//     fn my_iter(&self) -> MyVecIterator<T>;
// }
//
// impl<'a, T> Iterator for MyVecIterator<'a, T> {
//     type Item = &'a T; //we start here, by assigning a lifetime - this forces us to add it everywhere else
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let i = self.my_vec.get(self.counter)?;
//         self.counter += 1;
//         Some(i)
//     }
// }
//
// impl<T> MyIter<T> for Vec<T> {
//     fn my_iter(&self) -> MyVecIterator<T> {
//         MyVecIterator {
//             counter: 0,
//             my_vec: &self,
//         }
//     }
// }

// -----------------------------------------------------------------------------
// IntoIterator + tuple struct

// pub fn main() {
//     let mv = MyVec(vec![1,2,3]);
//
//     for i in mv.into_iter() {
//         println!("{}", i);
//     }
//
//     // let mut mv_iter = mv.into_iter();
//     // println!("{}", mv_iter.next().unwrap());
//     // println!("{}", mv_iter.next().unwrap());
//
//     // doesn't work: above fns exhaust the iterator
//     // println!("{:?}", mv);
// }
//
// #[derive(Debug)]
// struct MyVec(Vec<i32>);
//
// impl IntoIterator for MyVec {
//     type Item = i32;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// -----------------------------------------------------------------------------
// IntoIterator + named struct

// pub fn main() {
//     let mv2 = MyVec2::new(vec![1,2,3]);
//     for i in mv2.into_iter() {
//         println!("{}", i);
//     }
// }
//
// struct MyVec2<T> {
//     counter: usize,
//     my_vec: Vec<T>,
// }
//
// impl<T: Copy> MyVec2<T> {
//     fn new(my_vec: Vec<T>) -> Self {
//         Self {
//             counter: 0,
//             my_vec,
//         }
//     }
// }
//
// impl<T> IntoIterator for MyVec2<T> {
//     type Item = T;
//     type IntoIter = std::vec::IntoIter<Self::Item>; //we're saying that the iterator we want to turn the underlying MyVec2<T> into is the vec::IntoIter
//     fn into_iter(self) -> Self::IntoIter {
//         self.my_vec.into_iter() //and of course that's exactly the type that gets returned here
//     }
// }

// -----------------------------------------------------------------------------
// IntoIterator - conflicting implementation.

// /*
// because we've already implemented Iterator for our type MyVecIterator, we can't also implement IntoIterator
// the former AUTOMATICALLY IMPLEMENTS the latter
// */
//
// pub fn main() {
//     let mv = vec![1,2,3];
//     let mv2 = MyVecIterator::new(&mv);
//     for i in mv2 {
//         println!("{}", i);
//     }
// }
//
// struct MyVecIterator<'a, T> {
//     counter: usize,
//     my_vec: &'a Vec<T>,
// }
//
// impl<'a, T> MyVecIterator<'a, T> {
//     fn new(my_vec: &'a Vec<T>) -> Self {
//         Self {
//             counter: 0,
//             my_vec,
//         }
//     }
// }
//
// impl<'a, T> Iterator for MyVecIterator<'a, T> {
//     type Item = &'a T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let i = self.my_vec.get(self.counter)?;
//         self.counter += 1;
//         Some(i)
//     }
// }
//
// impl<'a, T> IntoIterator for MyVecIterator<'a, T> {
//     type Item = T;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         //
//     }
// }

// -----------------------------------------------------------------------------
// map & collect

pub fn main() {
    let mv = vec![1, 2, 3];
    let mut mv2 = MyVecIterator::new(&mv).map(|x| x*5);

    // --- MAP ---
    //loop doesn't work with MyIterator - for loop to work we'd need to implement either Iterator or IntoIterator
    // for i in mv2 {
    //     println!("{}", i);
    // }
    // while let Some(x) = mv2.next() {
    //     println!("{}", x);
    // }

    // --- COLLECT ---
    // NOTE: will only work if we call .map().collect, just .collect() won't work (we're not recreating the iterator)
    let mv3: Vec<i32> = mv2.collect();
    println!("{:?}", mv3);
}

//obv we're writing this boi for practice, in reality we'd never have to re-implement Iterator, we'd just use the one from std lib
trait MyIterator: Sized {
    //need to add Sized for map to work correctly: we're moving self in the signature of map, taht's why
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn map<F, B>(self, f: F) -> MyMap<Self, F>
        where F: FnMut(Self::Item) -> B {
        MyMap {
            iterator: self,
            func: f,
        }
    }
    fn collect<B>(self) -> B
        where B: MyFromIterator<Self::Item> {
        B::from_iter(self)
    }
}

struct MyVecIterator<'a, T> {
    counter: usize,
    my_vec: &'a Vec<T>,
}

impl<'a, T> MyVecIterator<'a, T> {
    fn new(my_vec: &'a Vec<T>) -> Self {
        Self {
            counter: 0,
            my_vec,
        }
    }
}

impl<'a, T> MyIterator for MyVecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.my_vec.get(self.counter)?;
        self.counter += 1;
        Some(i)
    }
}

// --- map-specific functionality

struct MyMap<I, F> {
    iterator: I,
    func: F,
}

//we also need to implement iterator for MyMap, because we're going to be calling .next() on it
impl<B, I: MyIterator, F: FnMut(I::Item) -> B> MyIterator for MyMap<I, F> {
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterator.next()?;
        Some((self.func)(item))
    }
}

// --- collect-specific functionality

//FromIterator takes an item A and implements a method from_iter that takes an item T (an iterator) and converts to A.
// for all this to work T must implement IntoIterator (or in our case, for simplicity, MyIterator)
// T = what we take, A = what we yield
trait MyFromIterator<A:Sized> {
    fn from_iter<T>(iter: T) -> Self where T: MyIterator<Item = A>;
}

impl MyFromIterator<i32> for Vec<i32> {
    fn from_iter<T>(mut iter: T) -> Self where T: MyIterator<Item = i32> {
        let mut result = Vec::new();
        while let Some(item) = iter.next() {
            result.push(item);
        }
        result
    }
}
