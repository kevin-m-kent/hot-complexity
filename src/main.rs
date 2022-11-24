mod make_spark;
use make_spark::*;
use std::env;
use ndarray::{Array2};

fn main() {
   let args: Vec<String> = env::args().collect();
   let l = args[1].parse::<u16>().unwrap();
   let prob_array = make_probability_array(&l);
   let mut starting_arr = Array2::<u32>::zeros((l as usize, l as usize));
   starting_arr[[1,0]] = 1;
   starting_arr[[2,0]] = 1;
   starting_arr[[4,0]] = 1;
   //let (mut comp_size_hash, labeled_arr) = get_connected_from_arr(starting_arr, l);
   let test_yield = get_spark_avg_yield(starting_arr, l, prob_array);
   println!("conencted obj: {:?}", test_yield)
}
