/// First N Fibonacci numbers using a Y combinator. Inspired by the Python
/// snippet in the foreword to The Recursive Book of Recursion.
use std::rc::Rc;

/// A Y combinator, copied from the Rust Playground at
/// <https://gist.github.com/rust-play/f1db7595a2c1f4a5b2f8cfa5a3bb017d>.  If
/// you know who the original author was, please open an issue so they can be
/// credited!
fn y<A, O, F>(f: Rc<dyn Fn(Rc<dyn Fn(A) -> O>) -> F>) -> impl Fn(A) -> O
where
    F: Fn(A) -> O,
    F: 'static,
    A: 'static,
    O: 'static,
{
    struct X<F>(Rc<dyn Fn(X<F>) -> F>);

    impl<F> Clone for X<F> {
        fn clone(&self) -> Self {
            Self(Rc::clone(&self.0))
        }
    }

    impl<F> X<F> {
        fn call(&self, x: Self) -> F {
            (self.0)(x)
        }
    }

    (|x: X<F>| x.call(x.clone()))(X(Rc::new(move |x: X<F>| {
        f(Rc::new(move |a| (x.call(x.clone()))(a)))
    })))
}

fn fib(n: usize) -> u32 {
    let next = |(a, b)| (b, a + b);
    y(Rc::new(move |f: Rc<dyn Fn(usize) -> (u32, u32)>| {
        move |n| -> (u32, u32) {
            match n {
                0 => (0, 1),
                n => next(f(n - 1)),
            }
        }
    }))(n)
    .0
}

pub fn fibs(n: usize) -> Vec<u32> {
    (0..n).map(fib).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib() {
        // Constructing a (strictly evaluated) list of 37 fibs, as in the
        // original example, would be prohibitively slow.
        assert_eq!(fibs(10), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
