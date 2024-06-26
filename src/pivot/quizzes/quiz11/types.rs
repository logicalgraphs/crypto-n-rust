use chrono::NaiveDate;

use book::{
   json_utils::{AsJSON,json_list,to_object},
   string_utils::quot
};

pub struct Ratio {
   dt: NaiveDate,
   ratio: f32
}

fn mk_ratio((dt, ratio): (&NaiveDate, &f32)) -> Ratio {
   Ratio { dt: dt.clone(), ratio: ratio.clone() }
}

impl AsJSON for Ratio {
   fn as_json(&self) -> String {
      to_object("date ratio", 
                &[quot(&format!("{}", &self.dt)),
                  format!("{:?}", self.ratio)])
   }
}

pub struct Ratios {
   name: String,
   ratios: Vec<Ratio>
}

pub fn mk_ratios(t1: &str, t2: &str, 
                 dates: &Vec<NaiveDate>, ratios: &Vec<f32>) -> Ratios {
   let dt_ratios: Vec<Ratio> =
      dates.into_iter().zip(ratios.into_iter()).map(mk_ratio).collect();
   Ratios { name: format!("{t1}/{t2}"), ratios: dt_ratios }
}

impl AsJSON for Ratios {
   fn as_json(&self) -> String {
      to_object("name ratios", &[quot(&self.name), json_list(&self.ratios)])
   }
}
