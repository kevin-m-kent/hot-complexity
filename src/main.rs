mod make_spark;
use make_spark::*;
use std::env;
use ndarray::{Array2};

fn main() {
   let args: Vec<String> = env::args().collect();
   let l = args[1].parse::<u16>().unwrap();
   let total = prob_total(l);
   let res = normalized_prob(0, 0, l, total);
   let prob_array = make_probability_array(l);
   let mut starting_arr = Array2::<f64>::zeros((l as usize, l as usize));
   println!("prob: {}", prob_array[[0, 0]])
}
