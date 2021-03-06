// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![feature(test)]

extern crate test;

use std::f64::{NAN, NEG_INFINITY, INFINITY, MAX};
use std::mem::size_of;
use test::black_box;

// Ensure the const-eval result and runtime result of float comparison are equivalent.

macro_rules! compare {
    ($op:tt) => {
        compare!(
            [NEG_INFINITY, -MAX, -1.0, -0.0, 0.0, 1.0, MAX, INFINITY, NAN],
            $op
        );
    };
    ([$($lhs:expr),+], $op:tt) => {
        $(compare!(
            $lhs,
            $op,
            [NEG_INFINITY, -MAX, -1.0, -0.0, 0.0, 1.0, MAX, INFINITY, NAN]
        );)+
    };
    ($lhs:expr, $op:tt, [$($rhs:expr),+]) => {
        $({
            // Wrap the check in its own function to reduce time needed to borrowck.
            fn check() {
                static CONST_EVAL: bool = $lhs $op $rhs;
                let runtime_eval = black_box($lhs) $op black_box($rhs);
                assert_eq!(CONST_EVAL, runtime_eval, stringify!($lhs $op $rhs));
                assert_eq!(
                    size_of::<[u8; ($lhs $op $rhs) as usize]>(),
                    runtime_eval as usize,
                    stringify!($lhs $op $rhs (forced const eval))
                );
            }
            check();
        })+
    };
}

fn main() {
    assert_eq!(0.0/0.0 < 0.0/0.0, false);
    assert_eq!(0.0/0.0 > 0.0/0.0, false);
    assert_eq!(NAN < NAN, false);
    assert_eq!(NAN > NAN, false);

    compare!(==);
    compare!(!=);
    compare!(<);
    compare!(<=);
    compare!(>);
    compare!(>=);
}
