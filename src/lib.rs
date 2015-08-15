
pub trait TakeWhileWithFailureIterator: Iterator {
    fn take_while_with_failure<P>(self, predicate: P) -> TakeWhileWithFailure<Self, P> where
        P: FnMut(&Self::Item) -> bool, Self: Sized {
            TakeWhileWithFailure{iter: self, flag: false, predicate: predicate}
        }
}

/// An iterator that accepts elements while `predicate` is true, including the first failure.
#[derive(Clone)]
pub struct TakeWhileWithFailure<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I: Iterator, P> Iterator for TakeWhileWithFailure<I, P>
where P: FnMut(&I::Item) -> bool {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            self.iter.next().and_then(|x| {
                if !(self.predicate)(&x) {
                    self.flag = true;
                }
                Some(x)
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}


impl<'a, I: Iterator + ?Sized> TakeWhileWithFailureIterator for &'a mut I {
    /// Creates an iterator that yields elements as long as the predicate returns true.
    /// Additionally, it includes the first element that returns false, after which no further
    /// elements will be yielded.
    ///
    /// This can be useful if, for example, you would like to read a stream until the first error
    /// and then stop processing immediately without losing the error.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = [1, 2, 3, 4, 5];
    /// let mut it = a.iter().take_while_with_failure(|&a| *a < 2);
    /// assert_eq!(it.next(), Some(&1));
    /// assert_eq!(it.next(), Some(&2));
    /// assert!(it.next().is_none());
    /// ```
    fn take_while_with_failure<P>(self, predicate: P) -> TakeWhileWithFailure<Self, P> where Self: Sized, P: FnMut(&Self::Item) -> bool {
        TakeWhileWithFailure{iter: self, flag: false, predicate: predicate}
    }
}


#[test]
fn it_includes_failure() {
    assert_eq!(vec![2,4,5,6].iter().take_while_with_failure(|x| (*x)%2 == 0).collect::<Vec<&i64>>(), vec![&2,&4,&5]);
}

#[test]
fn it_always_includes_the_first_element() {
    assert_eq!(vec![2,4,5,6].iter().take_while_with_failure(|_| false).collect::<Vec<&i64>>(), vec![&2]);
}

#[test]
fn it_includes_all_when_true() {
    assert_eq!(vec![2,4,5,6].iter().take_while_with_failure(|_| true).collect::<Vec<&i64>>(), vec![&2,&4,&5,&6]);
}

#[test]
fn it_works_on_empty_vecs() {
    let empty_vec: Vec<i64> = Vec::new();
    assert_eq!(empty_vec.iter().take_while_with_failure(|_| true).collect::<Vec<&i64>>().len(), 0);
}
