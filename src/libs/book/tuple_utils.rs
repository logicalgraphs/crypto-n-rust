pub fn fst<T: Clone, U>(p: (T, U)) -> T { p.0.clone() }
pub fn snd<T, U: Clone>(p: (T, U)) -> U { p.1.clone() }
