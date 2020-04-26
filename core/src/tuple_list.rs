//! A heterogeneous list using tuples
//!
//! # Examples
//!
//! ```rust
//! use game_engine::core::tuple_list::*;
//!
//! let list3 = (true, 5.0f32, 1u32);
//! let list4 = list3.push_back(Some(-3i32));
//!
//! assert_eq!(list4, (true, 5.0f32, 1u32, Some(-3i32)));
//! assert_eq!(list4.pop_back().1, list3);
//! assert_eq!(list3.pop_front().0, true);
//! ```
//!
//! The below code does not compile because of type mismatch.
//!
//! ```rust,compile_fail
//! # use game_engine::core::tuple_list::*;
//! #
//! # let list3 = (true, 5.0f32, 1u32);
//! # let list4 = list3.push_back(Some(-3i32));
//! #
//! assert_eq!(list4, (true, 5.0f32, 1u32, -3i32));
//! ```

/// A tuple type that is a list
///
/// All tuples of size up to 26 and the empty tuple (`()`) implement this trait.
pub trait TupleList {
    /// The length of this list
    const LENGTH: usize;
}

/// A tuple list that can be pushed to the front
///
/// Since tuples of size up to 26 are considered as lists, those of sizes up to 25 implement this
/// trait.
pub trait TupleListPushFront<T>: TupleList {
    /// The tuple list type after pushing the new item
    type Pushed: TupleList;

    /// Push a new item into the list
    fn push_front(self, new_item: T) -> Self::Pushed;
}

/// A tuple list that can be pushed to the back
///
/// Since tuples of size up to 26 are considered as lists, those of sizes up to 25 implement this
/// trait.
pub trait TupleListPushBack<T>: TupleList {
    /// The tuple list type after pushing the new item
    type Pushed: TupleList;

    /// Push a new item into the list
    fn push_back(self, new_item: T) -> Self::Pushed;
}

/// A tuple list that can be popped from the front
///
/// Since the empty tuple (`()`) has no items inside it, only tuples of size from 1 to 26 implement
/// this trait.
pub trait TupleListPopFront<T>: TupleList {
    /// The tuple list type after popping out the first item
    type Popped: TupleList;

    /// Pop the first item from the list
    fn pop_front(self) -> (T, Self::Popped);
}

/// A tuple list that can be popped from the back
///
/// Since the empty tuple (`()`) has no items inside it, only tuples of size from 1 to 26 implement
/// this trait.
pub trait TupleListPopBack<T>: TupleList {
    /// The tuple list type after popping out the last item
    type Popped: TupleList;

    /// Pop the last item from the list
    fn pop_back(self) -> (T, Self::Popped);
}

macro_rules! impl_tuple_list {
    ($length:expr; $( $ty:ident ),*) => {
        impl<$( $ty ),*> TupleList for ($( $ty, )*) {
            const LENGTH: usize = $length;
        }
    };
}

impl_tuple_list!(0;);
impl_tuple_list!(1; A);
impl_tuple_list!(2; A, B);
impl_tuple_list!(3; A, B, C);
impl_tuple_list!(4; A, B, C, D);
impl_tuple_list!(5; A, B, C, D, E);
impl_tuple_list!(6; A, B, C, D, E, F);
impl_tuple_list!(7; A, B, C, D, E, F, G);
impl_tuple_list!(8; A, B, C, D, E, F, G, H);
impl_tuple_list!(9; A, B, C, D, E, F, G, H, I);
impl_tuple_list!(10; A, B, C, D, E, F, G, H, I, J);
impl_tuple_list!(11; A, B, C, D, E, F, G, H, I, J, K);
impl_tuple_list!(12; A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple_list!(13; A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple_list!(14; A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_tuple_list!(15; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_tuple_list!(16; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_tuple_list!(17; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_tuple_list!(18; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_tuple_list!(19; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_tuple_list!(20; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_tuple_list!(21; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_tuple_list!(22; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_tuple_list!(23; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_tuple_list!(24; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_tuple_list!(25; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_tuple_list!(26; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

macro_rules! impl_tuple_push {
    ($( $var:ident: $ty:ident ),*) => {
        impl<New $( , $ty )*> TupleListPushFront<New> for ($( $ty, )*) {
            type Pushed = (New, $( $ty, )*);
            fn push_front(self, new_item: New) -> Self::Pushed {
                let ($( $var, )*) = self;
                (new_item, $( $var, )*)
            }
        }

        impl<$( $ty, )* New> TupleListPushBack<New> for ($( $ty, )*) {
            type Pushed = ($( $ty, )* New,);
            fn push_back(self, new_item: New) -> Self::Pushed {
                let ($( $var, )*) = self;
                ($( $var, )* new_item,)
            }
        }
    };
}

impl_tuple_push!();
impl_tuple_push!(a: A);
impl_tuple_push!(a: A, b: B);
impl_tuple_push!(a: A, b: B, c: C);
impl_tuple_push!(a: A, b: B, c: C, d: D);
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E);
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F);
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V, w: W);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V, w: W, x: X);
#[rustfmt::skip]
impl_tuple_push!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V, w: W, x: X, y: Y);

macro_rules! first_in_list {
    ($first:ident $( , $rest:ident )*) => {
        $first
    };
}

macro_rules! drop_first_in_list {
    ($first:ident $( , $rest:ident )*) => { ($( $rest, )*) };
}

macro_rules! last_in_list {
    ($single:ident) => { $single };
    ($first:ident, $( $rest:ident ),+) => { last_in_list!($( $rest ),+) };
}

macro_rules! drop_last_in_list {
    ($( $list:ident ),+) => { drop_last_in_list!(@inner () <- $( $list ),+) };
    (@inner ($( $gathered:ident ),*) <- $last:ident) => { ($( $gathered, )*) };
    (@inner ($( $gathered:ident ),*) <- $rest_first:ident, $( $rest:ident ),+) => {
        drop_last_in_list!(@inner ($( $gathered, )* $rest_first) <- $( $rest ),+)
     };
}

macro_rules! impl_tuple_pop {
    ($( $var:ident: $ty:ident ),+) => {
        impl<$( $ty ),+> TupleListPopFront<first_in_list!($( $ty ),+)> for ($( $ty, )+) {
            type Popped = drop_first_in_list!($( $ty ),+);
            fn pop_front(self) -> (first_in_list!($( $ty ),+), Self::Popped) {
                let ($( $var, )+) = self;
                (first_in_list!($( $var ),+), drop_first_in_list!($( $var ),+))
            }
        }

        impl<$( $ty ),+> TupleListPopBack<last_in_list!($( $ty ),+)> for ($( $ty, )+) {
            type Popped = drop_last_in_list!($( $ty ),+);
            fn pop_back(self) -> (last_in_list!($( $ty ),+), Self::Popped) {
                let ($( $var, )+) = self;
                (last_in_list!($( $var ),+), drop_last_in_list!($( $var ),+))
            }
        }
    };
}

impl_tuple_pop!(a: A);
impl_tuple_pop!(a: A, b: B);
impl_tuple_pop!(a: A, b: B, c: C);
impl_tuple_pop!(a: A, b: B, c: C, d: D);
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E);
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F);
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V, w: W);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V, w: W, x: X);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V, w: W, x: X, y: Y);
#[rustfmt::skip]
impl_tuple_pop!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M,
    n: N, o: O, p: P, q: Q, r: R, s: S, t: T, u: U, v: V, w: W, x: X, y: Y, z: Z);
