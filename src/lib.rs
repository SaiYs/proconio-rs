//! Easy IO library for competitive programming.
//!
//! `proconio` provides an easy way to read values from stdin (or other source).  The main is
//! `input!` macro.
//!
//! # Examples
//!
//! The macro's user interface is basically the same with [tanakh's input
//! macro](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8).
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("32 54 -23");
//!
//! input! {
//! #   from source,
//!     n: u8,
//!     m: u32,
//!     l: i32,
//! }
//!
//! // now you can use n, m and l as variable.
//! println!("{} {} {}", n, m, l);
//! # assert_eq!(n, 32);
//! # assert_eq!(m, 54);
//! # assert_eq!(l, -23);
//! ```
//!
//! In above code, variables n, m and l are declared and stored values are read from stdin.
//!
//! You can declare mutable variables like below:
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("32 54");
//!
//! input! {
//! #   from source,
//!     n: u32,
//!     mut m: u32,
//! }
//!
//! m += n; // OK: m is mutable
//! # assert_eq!(n, 32);
//! # assert_eq!(m, 86);
//! ```
//!
//! You can read an array or a matrix like this:
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("5 4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");
//!
//! input! {
//! #    from source,
//!     n: usize,
//!     m: usize,
//!     a: [[i32; n]; m] // `a` is Vec<Vec<i32>>, (n, m)-matrix.
//! }
//! # assert_eq!(
//! #     a,
//! #     [
//! #         [1, 2, 3, 4, 5],
//! #         [1, 2, 3, 4, 5],
//! #         [1, 2, 3, 4, 5],
//! #         [1, 2, 3, 4, 5]
//! #     ]
//! # );
//! ```
//!
//! Strings can be read as various types:
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! use proconio::types::{Bytes, Chars};
//! # let source = AutoSource::from("  string   chars\nbytes");
//!
//! input! {
//! #     from source,
//!     string: String, // read as String
//!     chars: Chars,   // read as Vec<char>
//!     bytes: Bytes,   // read as Vec<u8>
//! }
//!
//! // if you enter "string chars bytes" to the stdin, they are like this.
//! assert_eq!(string, "string");
//! assert_eq!(chars, ['c', 'h', 'a', 'r', 's']);
//! assert_eq!(bytes, b"bytes");
//! ```
//!
//! You can read tuples:
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("1 2 3 4 5");
//!
//! input! {
//! #   from source,
//!     t: (i32, i32, i32, i32, i32),
//! }
//!
//! // if you enter "1 2 3 4 5" to the stdin, `t` is like this.
//! assert_eq!(t, (1, 2, 3, 4, 5));
//! ```
//!
//! And you can freely combine these types.
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let source = AutoSource::from("4 3 1 1 1 1 2 2 2 2 3 3 3 3 4 4 4 4 5 5 5 5");
//!
//! input! {
//! #   from source,
//!     n: usize,
//!     m: usize,
//!     t: [([u32; m], i32); n],
//! }
//! # assert_eq!(
//! #     t,
//! #     [
//! #         (vec![1,1,1],1),
//! #         (vec![2,2,2],2),
//! #         (vec![3,3,3],3),
//! #         (vec![4,4,4],4),
//! #     ]
//! # );
//! ```
//!
//! You can use `input!` macro multiple times.  For the second time, `input!` macro reads rest of
//! input.  It works even if the first input stops at the middle of a line.  The subsequent reads
//! will be started at the rest of the line.  This may be helpful for problems where multiple
//! datasets are given once.
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! # let mut source = AutoSource::from("4 2 1 2 2 3 4 2 1 2 2 3 4");
//!
//! input! {
//! #   from &mut source,
//!     n: usize,
//! }
//!
//! for i in 0..n {
//!     input! {
//! #       from &mut source,
//!         m: usize,
//!         a: [i32; m],
//!     }
//! #   assert_eq!(a[0], if i % 2 == 0 { 1 } else { 3 });
//! #   assert_eq!(a[1], if i % 2 == 0 { 2 } else { 4 });
//! }
//! ```
//!
//! Some special types exists.  `Usize1` and `Isize1` are.  They are read as `usize` and `isize`
//! respectively, but the read value is decremented.  It enables us to automatically convert
//! 1-indexed vertices numbers to 0-indexed array indices.
//!
//! ```
//! # extern crate proconio;
//! # use proconio::source::auto::AutoSource;
//! use proconio::input;
//! use proconio::types::Usize1;
//! # let mut source = AutoSource::from("4   1 3   3 4   6 1   5 3");
//!
//! input! {
//! #   from &mut source,
//!     n: usize,
//!     edges: [(Usize1, Usize1); n],
//! }
//!
//! // if you enter "4   1 3   3 4   6 1   5 3", the decremented value is stored.
//! assert_eq!(edges[0], (0, 2));
//! assert_eq!(edges[1], (2, 3));
//! assert_eq!(edges[2], (5, 0));
//! assert_eq!(edges[3], (4, 2));
//! ```
//!
//! `Usize1` and `Isize1` is a simple unit struct.  This type is only used to tell "how to read the
//! value".  It can be defined by `Readable` trait.  This trait doesn't require the output type to
//! be the same with the implementor.  `Usize1` is implementing `Readable` trait, and there it
//! defines the type of read value is `usize`.  You can implement `Readable` for your own type to
//! read values in customized way.
//!
//! Finally, you can make your own types `Readable` using `#[derive_readable]` attribute.  Types
//! used in the struct are automatically translated to their output types, so a member declared as
//! `Usize1` has type `usize` as real struct.
//!
//! ```
//! # extern crate proconio;
//! # extern crate proconio_derive;
//! use proconio::input;
//! # use proconio::source::auto::AutoSource;
//! use proconio_derive::derive_readable;
//!
//! // Unit struct can derive readable.  This generates a no-op for the reading.  Not ignoring
//! // the read value, but simply skip reading process.  You cannot use it to discard the input.
//! #[derive_readable]
//! #[derive(PartialEq, Debug)]
//! struct Weight;
//!
//! #[derive_readable]
//! #[derive(PartialEq, Debug)]
//! struct Cost(i32);
//!
//! #[derive_readable]
//! #[derive(Debug)]
//! struct Edge {
//!     from: usize,
//!     to: proconio::types::Usize1, // The real Edge::to has type usize.
//!     weight: Weight,
//!     cost: Cost,
//! }
//!
//! fn main() {
//! #   let source = AutoSource::from("12 32 35");
//!     input! {
//! #       from source,
//!         edge: Edge,
//!     }
//!
//!     // if you enter "12 32 35" to the stdin, the values are as follows.
//!     assert_eq!(edge.from, 12);
//!     assert_eq!(edge.to, 31);
//!     assert_eq!(edge.weight, Weight);
//!     assert_eq!(edge.cost, Cost(35));
//! }
//! ```

pub mod read;
pub mod source;
pub mod types;

use crate::source::auto::AutoSource;
use lazy_static::lazy_static;
use std::cell::UnsafeCell;
use std::io;
use std::io::stdout;
use std::io::{BufReader, BufWriter, Stdin, Stdout};
use std::sync::Mutex;

lazy_static! {
    #[doc(hidden)]
    pub static ref STDIN_SOURCE: Mutex<AutoSource<BufReader<Stdin>>> =
        Mutex::new(AutoSource::new(BufReader::new(io::stdin())));
}

thread_local! {
    pub static STDOUT: UnsafeCell<BufWriter<Stdout>> = UnsafeCell::new(BufWriter::new(stdout()));
}

/// read input from stdin.
///
/// basic syntax is:
/// ```ignore
/// input! {
///     from source,          // optional: if you omitted, stdin is used by default.
///     (mut) variable: type, // mut is optional: mut makes the variable mutable.
///     ...
/// }
/// ```
/// the trailing comma is optional.  `source` can be anything implementing `Source`.  This macro
/// moves out the specified source.  If you want to prevent moving, you can use `&mut source` since
/// `&mut S` where `S: Source` also implements `Source`.
///
/// **Note:** for macro matcher's limitation, only a single token tree (`i32`, `[T; n]`, `(T, U)` or
/// etc) is accepted by `type`. This means you can't use generics `YourType<T>` as the type.  To
/// use such a type, you can alias it like `type YourTypeT = YourType<T>` to make it a single token
/// tree.  (This is why `Chars` and `Bytes` exists.  They are simply alias of `Vec<char>` and
/// `Vec<u8>`.)
#[macro_export]
macro_rules! input {
    (from $source:expr $(,)?) => {};
    (from $source:expr, mut $var:ident: $kind:tt $($rest:tt)*) => {
        let mut s = $source;
        let mut $var = $crate::read_value!($kind; &mut s);
        input!(from &mut s $($rest)*);
    };
    (from $source:expr, $var:ident: $kind:tt $($rest:tt)*) => {
        let mut s = $source;
        let $var = $crate::read_value!($kind; &mut s);
        input!(from &mut s $($rest)*);
    };
    ($($rest:tt)*) => {
        let mut locked_stdin = $crate::STDIN_SOURCE.lock().expect("failed to lock the stdin");
        input! {
            from &mut *locked_stdin,
            $($rest)*
        };
        drop(locked_stdin); // release the lock
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! read_value {
    // array
    ([$kind:tt; $len:expr]; $source:expr) => {{
        let mut res = Vec::new();
        res.reserve($len);
        for _ in 0..$len {
            res.push($crate::read_value!($kind; $source));
        }
        res
    }};

    // tuple
    (($($kind:tt),*); $source:expr) => {
        (
            $($crate::read_value!($kind; $source),)*
        )
    };

    // normal other
    ($ty:tt; $source:expr) => {
        $crate::read_value!(@ty $ty; $source);
    };

    // actual reading
    (@ty $ty:ty; $source:expr) => {
        <$ty as $crate::source::Readable>::read($source)
    }
}

#[macro_export]
macro_rules! output {
    ($($tt:tt)*) => {{
        use std::io::Write as _;
        $crate::STDOUT.with(|out| write!(unsafe { &mut *out.get() }, $($tt)*).unwrap());
    }}
}

#[macro_export]
macro_rules! outputln {
    ($($tt:tt)*) => {{
        use std::io::Write as _;
        $crate::STDOUT.with(|out| writeln!(unsafe { &mut *out.get() }, $($tt)*).unwrap());
    }}
}

pub fn flush_output() {
    use std::io::Write as _;
    crate::STDOUT.with(|out| unsafe { &mut *out.get() }.flush().unwrap());
}

#[cfg(test)]
mod tests {
    use crate::source::auto::AutoSource;

    #[test]
    fn input_number() {
        let source = AutoSource::from("    32   54 -23\r\r\n\nfalse");

        input! {
            from source,
            n: u8,
            m: u32,
            l: i32,
        }

        assert_eq!(n, 32);
        assert_eq!(m, 54);
        assert_eq!(l, -23);
    }

    #[test]
    fn input_str() {
        use crate::types::{Bytes, Chars};
        let source = AutoSource::from("  string   chars\nbytes");

        input! {
            from source,
            string: String,
            chars: Chars,
            bytes: Bytes,
        }

        assert_eq!(string, "string");
        assert_eq!(chars, ['c', 'h', 'a', 'r', 's']);
        assert_eq!(bytes, b"bytes");
    }

    #[test]
    fn input_array() {
        let source = AutoSource::from("5 4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

        input! {
            from source,
            n: usize,
            m: usize,
            a: [[i32; n]; m] // no trailing comma is OK
        }

        assert_eq!(
            a,
            [
                [1, 2, 3, 4, 5],
                [1, 2, 3, 4, 5],
                [1, 2, 3, 4, 5],
                [1, 2, 3, 4, 5]
            ]
        );
    }

    #[test]
    fn input_tuple() {
        let source = AutoSource::from("4 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5 1 2 3 4 5");

        input! {
            from source,
            n: usize,
            t: [(i32, i32, i32, i32, i32); n],
        }

        assert_eq!(
            t,
            [
                (1, 2, 3, 4, 5),
                (1, 2, 3, 4, 5),
                (1, 2, 3, 4, 5),
                (1, 2, 3, 4, 5)
            ]
        );
    }

    #[test]
    fn input_multiple_times() {
        let mut source = AutoSource::from("4 1 2 3 4\n1 2\r\n\r\r\n3 4");

        input! {
            from &mut source,
            n: usize,
        }

        for i in 0..n {
            input! {
                from &mut source,
                j: i32, k: i32,
            }

            assert_eq!(j, if i % 2 == 0 { 1 } else { 3 });
            assert_eq!(k, if i % 2 == 0 { 2 } else { 4 });
        }
    }

    #[test]
    fn input_iusize1() {
        use crate::types::Usize1;

        let mut source = AutoSource::from("4 1 2 3 4 5 6 7 8");

        input! {
            from &mut source,
            n: usize,
        }

        for i in 0..n {
            input! {
                from &mut source,
                from: Usize1, to: Usize1
            }

            assert_eq!(from, i * 2);
            assert_eq!(to, i * 2 + 1);
        }
    }

    #[test]
    fn input_mut() {
        let mut source = AutoSource::from("8 1 2 3 4 5 6 7 8");
        input! {
            from &mut source,
            mut n: usize,
        }

        let mut sum = 0;
        while n > 0 {
            input!(from &mut source, x: u32);
            sum += x;
            n -= 1;
        }
        assert_eq!(sum, 36);
    }
}
