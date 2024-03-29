use book::{
   err_utils::{ErrStr,err_or},
   rest_utils::read_rest
};
use crate::types::{LSD, parse_lsds_without_burn, merge_burn_rates_d};
use crate::read_rest::fetch_burns;

pub fn fetch_stride_lsds() -> ErrStr<Vec<LSD>> {
   let stroll = "Stride-Labs/stride/stakeibc/host_zone";
   let url = "https://stride-api.polkachu.com";
   let body = err_or(read_rest(&format!("{url}/{stroll}")),
                     "Unable to read LSD REST endpoint")?;
   let burns = fetch_burns()?;
   let burnless_lsds = parse_lsds_without_burn(&body);
   Ok(merge_burn_rates_d(&burnless_lsds, &burns, true))
}
