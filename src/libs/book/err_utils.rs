use std::fmt::Debug;

pub type ErrStr<T> = Result<T, String>;

pub fn err_or<T, E: Debug>(thunk: Result<T, E>, err_str: &str) -> ErrStr<T> {
   match thunk {
      Ok(ans) => Ok(ans),
      Err(err) => Err(format!("{err_str}. Error: {err:?}"))
   }
}
