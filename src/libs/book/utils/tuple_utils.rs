pub fn fst<T: Clone, U>(p: (T, U)) -> T { p.0.clone() }
pub fn snd<T, U: Clone>(p: (T, U)) -> U { p.1.clone() }

pub fn swap<T, U>(t: (T, U)) -> (U, T) {
   (t.1, t.0)
}

// Now we must consider if tuple_utils is a trojan horse for arrow functions

// Put another way: do I now need arrow_utils?

pub fn first<A, B, C>(f: impl Fn(A) -> C) -> impl Fn((A, B)) -> (C, B) {
   move |(a, b)| (f(a), b)
}
