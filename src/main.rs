mod make_spark;
use make_spark::*;
use std::env;
use ndarray::{Array2};
use polars::prelude::*;
use std::time::{Duration, Instant};

fn main() {
   let start = Instant::now();
   let args: Vec<String> = env::args().collect();
   let l = args[1].parse::<u16>().unwrap();
   let D = args[2].parse::<u16>().unwrap();
   let prob_array = make_probability_array(&l);
   let mut density = 0.0;
   let mut starting_arr = Array2::<u32>::zeros((l as usize, l as usize));
   let mut densities = Vec::new();
   let mut yields = Vec::new();
   for k in 0..(&l*&l) {

      let sampled_indexes = sample_random_indices(&starting_arr, &l, D as usize);
      let  (mut win_i, mut win_j, mut max_yield) = (0, 0, 0.0);
      for idx in sampled_indexes {

         starting_arr[[idx.0 as usize, idx.1 as usize]] = 1;
         let avg_yield = get_spark_avg_yield(&starting_arr, l, &prob_array);
         if avg_yield > max_yield {

            max_yield = avg_yield;
            win_i = idx.0;
            win_j = idx.1;

         }


         starting_arr[[idx.0 as usize, idx.1 as usize]]  = 0;

      }
      starting_arr[[win_i as usize, win_j as usize]] = 1;
      density = (starting_arr.sum() as f64)/(l as f64*l as f64);
      //println!("yield:{}, density: {}", max_yield, density);
      densities.push(density);
      yields.push(max_yield);
        
   }
   let s1 = Series::new("densities", &densities);
   let s2 = Series::new("yields", &yields);
   let mut df: PolarsResult<DataFrame> = DataFrame::new(vec![s1, s2]);


   let mut file = std::fs::File::create(format!("Data/run_d{}_l{}.csv", &D, &l)).unwrap();
   CsvWriter::new(&mut file).finish(&mut df.unwrap()).unwrap();
   let duration = start.elapsed();
   println!("Time elapsed is: {:?}", duration);

   //let (mut comp_size_hash, labeled_arr) = get_connected_from_arr(starting_arr, l);
   //let test_yield = get_spark_avg_yield(&starting_arr, l, prob_array);
   //let random_indices = sample_random_indices(&starting_arr, &l, 3);
   //println!("avg_burn: {:?}", test_yield);
   //println!("final arr: {:?}", starting_arr);
   //println!("df: {:?}", df);

}
