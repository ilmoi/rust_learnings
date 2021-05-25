pub fn main() {}

//you can give flatten anything that returns an IntoIter and it will call into_iter() for you
fn flatten<O>(iter: O) -> Flatten<O::IntoIter>
    where
        O: IntoIterator,
        O::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

struct Flatten<O>
    where
        O: Iterator,
        O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>, //we need the iterator type of the item, not the item itself
}

impl<O> Flatten<O>
    where
        O: Iterator,
        O::Item: IntoIterator,
{
    fn new(outer: O) -> Self {
        Self {
            outer,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
    where
        O: Iterator,
        O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }
            // the question mark below is the part that terminates the loop at some point
            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }
}