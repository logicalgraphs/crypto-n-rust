use std::future::Future;

// 1. Define the extension trait
pub trait AsyncOptionExt<T> {
    fn async_and_then<U, F, Fut>(self, f: F) -> impl Future<Output=Option<U>>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = Option<U>>;
}

// 2. Implement it for all Option<T>
impl<T> AsyncOptionExt<T> for Option<T> {
    async fn async_and_then<U, F, Fut>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = Option<U>>,
    {
        match self {
            Some(value) => f(value).await,
            None => None,
        }
    }
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;
   use crate::string_utils::s;

   async fn get_name_by_id(id: u32) -> Option<String> {
       if id == 7 { Some("Alice".to_string()) } else { None }
   }

   #[tokio::test] async fn test_async_and_then_some() {
      let user_id: Option<u32> = Some(7);

      // Now you can fluidly chain async operations
      let username = user_id
          .async_and_then(|id| async move { get_name_by_id(id).await })
          .await;

      assert_eq!(Some(s("Alice")), username);
   }
}
