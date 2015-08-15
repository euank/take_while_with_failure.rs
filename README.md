# Take While with Failure

This library implements an additional iterator for rust.

Creates an iterator that yields elements as long as the predicate returns true.
Additionally, it includes the first element that returns false, after which no further
elements will be yielded.

This can be useful if, for example, you would like to read a stream until the first error
and then stop processing immediately without losing the error.

## Example

```
let a = [1, 2, 3, 4, 5];
let mut it = a.iter().take_while_with_failure(|&a| *a < 2);
assert_eq!(it.next(), Some(&1));
assert_eq!(it.next(), Some(&2));
assert!(it.next().is_none());
```
