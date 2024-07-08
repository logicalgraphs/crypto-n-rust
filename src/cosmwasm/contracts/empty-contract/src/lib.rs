use cosmwasm_std::{
    entry_point, DepsMut, Empty, Env, MessageInfo, Response, StdResult
   
};

#[entry_point]
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty)
     -> StdResult<Response> {
   Ok(Response::new())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
