#[derive(Debug, Clone)] 
pub struct Dyad<T>{
   pair: (String, String),
   value: T
}

pub fn mk_dyad<T>(pair: (String, String), value: T) -> Dyad<T> {
   Dyad { pair, value }
}

pub fn unpair<T: Clone>(d: &Dyad<T>) -> ((String, String), T) {
   (d.pair.clone(), d.value.clone())
}

#[derive(Debug, Clone)] 
pub struct Tag<T>{
   tag: String,
   value: T
}

pub fn mk_tag<T>((tag, value): (String, T)) -> Tag<T> {
   Tag { tag, value }
}

pub fn untag<T: Clone>(t: &Tag<T>) -> (String, T) {
   (t.tag.clone(), t.value.clone())
}

pub fn value<T: Clone>(t: &Tag<T>) -> T { t.value.clone() }
