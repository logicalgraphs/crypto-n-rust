use std::future::Future;

// 1. Define the extension trait
pub trait AsyncTryExt<T> {
    type Output<U>; // Generic Associated Type (GAT)

    fn async_and_then<U, F, Fut>(self, f: F) -> impl Future<Output = Self::Output<U>>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = Self::Output<U>>;
}

// 2. Implement it for all Option<T>
impl<T> AsyncTryExt<T> for Option<T> {
    type Output<U> = Option<U>;
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

// 3. and for Result<T, E>

impl<T, E> AsyncTryExt<T> for Result<T, E> {
    type Output<U> = Result<U, E>;
    async fn async_and_then<U, F, Fut>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = Result<U, E>>,
    {
        match self {
            Ok(value) => f(value).await,
            Err(err) => Err(err),
        }
    }
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;
   use crate::{ err_utils::ErrStr, string_utils::s, utils::pred };

   async fn get_name_by_id(id: u32) -> Option<String> {
       pred(id == 7, s("Alice"))
   }

   async fn get_addy_by_id(id: u32) -> ErrStr<String> {
       if id == 3 { Ok(s("123 Main Street"))
       } else {
          Err(format!("Cannot find address for id {id}"))
       }
   }

   #[tokio::test] async fn test_async_and_then_some() {
      let user_id: Option<u32> = Some(7);

      // Now you can fluidly chain async operations
      let username = user_id
          .async_and_then(|id| async move { get_name_by_id(id).await })
          .await;

      assert_eq!(Some(s("Alice")), username);
   }

   #[tokio::test] async fn fail_async_and_then() {
      let user_id = Some(7);
      let user_addy =
         user_id.ok_or(s("erroneous"))
                .async_and_then(|id| async move { get_addy_by_id(id).await })
                .await;
      assert!(user_addy.is_err());
   }

   #[tokio::test] async fn test_async_and_then_ok() {
      let user_id = Some(3);
      let user_addy =
         user_id.ok_or(s("erroneous"))
                .async_and_then(|id| async move { get_addy_by_id(id).await })
                .await;
      assert_eq!(user_addy, Ok(s("123 Main Street")));
   }
}
