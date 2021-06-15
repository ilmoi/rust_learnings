/*
Relationships between Iterator, IntoIterator, FromIterator
- Iterator -> automatically implements IntoIterator/FromIterator -> makes .into_iter() available

- IntoIterator/FromIterator -> also automatically implements Iterator / makes .into_iter() available

- .iter() and .iter_mut() don't come for free when you implement Iterator / IntoIterator / FromIterator
- instead they either need to already exist on a collection, or you have to define them separately
*/

// -----------------------------------------------------------------------------
// Iterator

pub fn main() {
    let mv = vec![1,2,3];
    let mv2 = MyVecIterator{my_vec: &mv, counter: 0};

    // test .into_iter()
    for i in mv2.clone().into_iter() {
        println!("{}", i);
    }

    // test w/o appending anything (implicitly calls .into_iter())
    for i in mv2.clone() {
        println!("{}", i);
    }

}

#[derive(Clone)]
struct MyVecIterator<'a, T> {
    counter: usize,
    my_vec: &'a Vec<T>,
}

impl<'a, T> Iterator for MyVecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.my_vec.get(self.counter)?;
        self.counter += 1;
        Some(i)
    }
}

// CAN'T IMPLEMENT THIS COZ NEED TO OWN THE VALUE
// impl<'a, T> IntoIterator for MyVecIterator<'a, T> {
//     type Item = T;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.my_vec.into_iter()
//     }
// }

// -----------------------------------------------------------------------------
// IntoIterator

// pub fn main() {
//     let mv = vec![1,2,3];
//     let mv_perm= MyPermVecIterator{my_vec: mv, counter: 0};
//
//     // test .into_iter()
//     for i in mv_perm.clone().into_iter() {
//         println!("{}", i);
//     }
//
//     // test w/o appending anything (implicitly calls .into_iter())
//     for i in mv_perm.clone() {
//         println!("{}", i);
//     }
//
//     //this is how I'm testing IntoIter (map) and FromIter (collect)
//     let mv_collected: Vec<i32> = mv_perm.clone().into_iter().map(|x| x).collect();
//     println!("{:?}", mv_collected);
// }
//
// #[derive(Clone)]
// struct MyPermVecIterator<T> {
//     counter: usize,
//     my_vec: Vec<T>,
// }
//
// // IF LEAVE THIS ONE GET A CONFLICTING IMPL ERROR
// // impl<T: Copy> Iterator for MyPermVecIterator<T> {
// //     type Item = T;
// //
// //     fn next(&mut self) -> Option<Self::Item> {
// //         let i = self.my_vec.get(self.counter)?;
// //         self.counter += 1;
// //         Some(*i)
// //     }
// // }
//
// impl<T> IntoIterator for MyPermVecIterator<T> {
//     type Item = T;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.my_vec.into_iter()
//     }
// }

// -----------------------------------------------------------------------------
// .iter(), .iter_mut()

// /*
// these methods (together with .into_iter()) create iterators from another collection
// thus we really have 2 ways of creating an iterator:
// 1. impl Iterator ourselves, which includes defining the item and the next() function
// 2. calling one of the 3 above methods, if already implemented on the collection
//
// But we can also manually define them as below:
//  */
//
// pub fn main() {
//     let mv = vec![1, 2, 3];
//     let mv2 = MyVecIterator { my_vec: &mv, counter: 0 };
//
//     for i in mv.clone().iter() {
//         println!("{}", i);
//     }
// }
//
// #[derive(Clone)]
// struct MyVecIterator<'a, T> {
//     counter: usize,
//     my_vec: &'a Vec<T>,
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
// trait Iter<T> {
//     fn iter(&self) -> MyVecIterator<T>;
// }
//
// impl<T> Iter<T> for Vec<T> {
//     fn iter(&self) -> MyVecIterator<T> {
//         MyVecIterator {
//             counter: 0,
//             my_vec: self
//         }
//     }
// }

// -----------------------------------------------------------------------------
// decoupling the counter from the data structure
// as per advice on my question here - https://stackoverflow.com/questions/67675256/is-it-possible-to-impl-iterator-for-a-tuple-struct-holding-a-vector

// pub fn main() {
//     let mv = MyVec(vec![1, 2, 3]);
//
//     for i in mv.iter() {
//         println!("{}", i);
//     }
// }
//
// struct MyVec<T>(Vec<T>);
//
// // ---------------- wrapper
//
// struct MyVecIterator<'a, T> {
//     counter: usize,
//     my_vec: &'a MyVec<T>,
// }
//
// impl<'a, T> Iterator for MyVecIterator<'a, T> {
//     type Item = &'a T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let i = self.my_vec.0.get(self.counter)?;
//         self.counter += 1;
//         Some(i)
//     }
// }
//
// trait Iter<T> {
//     fn iter(&self) -> MyVecIterator<T>;
// }
//
// impl<T> Iter<T> for MyVec<T> {
//     fn iter(&self) -> MyVecIterator<T> {
//         MyVecIterator {
//             counter: 0,
//             my_vec: self
//         }
//     }
// }

