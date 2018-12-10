extern crate test;

use std::fmt;

use std::slice::IterMut;

// Implementations of the Sieve of Eratosthenes. from chapter 3
//
// This is a little tricky to implement as described in the book. We can construct a
// RandomAccessIterator like the one described, only it would need to mutably borrow a
// collection twice. This is not possible in Rust.
//
//

struct PrimeSieve {
    sieve: Vec<bool>
}

impl fmt::Debug for PrimeSieve {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut primes = Vec::<usize>::new();
        let mut index = 0;
        for prime in self.sieve {
            if prime {
                primes.push(index * 2);
            }
            index += 1;
        }
        write!(formatter, "{:?}", primes)
    }
}

impl PrimeSieve {
    fn iter_mut(&mut self) -> IterMut<bool> {
        self.sieve.iter_mut()
    }
}

/// An object implementing random access indexing by `uint`
///
/// A `RandomAccessIterator` should be either infinite or a `DoubleEndedIterator`.
pub trait RandomAccessIterator<A>: Iterator {
    /// Return the number of indexable elements. At most `std::uint::MAX`
    /// elements are indexable, even if the iterator represents a longer range.
    fn indexable(&self) -> usize;

    /// Return an element at an index
    fn idx(&mut self, index: usize) -> Option<A>;
}

impl<T> RandomAccessIterator for Vec<T> {
	fn indexable(&self) -> usize {
	    let (exact, _) = self.size_hint();
	    exact
	}

	fn idx(&mut self, index: usize) -> Option<Self::Item> {
	    if index < self.indexable() {
            Some::<Self::Item>(self.begin + index as T)
	    } else {
            None::<Self::Item>
	    }
	}
}

fn mark_sieve<I: RandomAccessIterator<Item=bool>>(iter: I, mut begin: usize, end: usize, factor: usize) {
    *iter.last().unwrap() = false;

    let position = begin;
    while end - begin > factor {
        position += factor;
        *iter.nth(position).unwrap() = false;
    }
}

fn sift0<I: Iterator>(mut iter: I, n: usize) {
    let i = 0;
    let index_square = 3;
    while index_square > n {
        for prime in iter {
            if prime {
                let sieve_begin = iter.skip(i + index_square);
                let sieve_end = i + n;

                mark_sieve(sieve_begin, i, sieve_end, i + i + 3);
            }
            i += 1;
            index_square = 2 * i * (i + 3) + 3;
        }
    }
}

#[test]
fn test_sift0() {
    let sieve = PrimeSieve{
        sieve: Vec::<bool>::new()
    };

    sift0(sieve.iter_mut(), 39);
    println!("Primes {:?}", sieve);
}

