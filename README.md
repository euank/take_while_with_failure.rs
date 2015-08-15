# Take While with Failure

This library implements an additional iterator for rust.

Creates an iterator that yields elements as long as the predicate returns true.
Additionally, it includes the first element that returns false, after which no further
elements will be yielded.

This can be useful if, for example, you would like to read a stream until the first error
and then stop processing immediately without losing the error.

## But what about itertools fold\_results?

Right after implementing this I found out that that package exists. Oops.

I highly recommend using itertools'
[fold\_results](https://bluss.github.io/rust-itertools/doc/itertools/trait.Itertools.html#method.fold_results)
if it is relevant.

## Naming

I couldn't think of a better one. If you can, feel free to pull request.

## This is dumb

Aww, shucks, thanks. You too :heart:

Really though, I'm a neophyte with rust and it's possible I overlooked something when I made this and it's in fact utterly dumb and useless. Open an issue and tell me!

Oh, and I do know I could have used a for loop to replace this fairly trivially. That doesn't count :grin:

## Example

```
let a = [1, 2, 3, 4, 5];
let mut it = a.iter().take_while_with_failure(|&a| *a < 2);
assert_eq!(it.next(), Some(&1));
assert_eq!(it.next(), Some(&2));
assert!(it.next().is_none());
```
