mod make_spark;
use make_spark::*;
use std::env;
use ndarray::{Array3, Array, Array2, arr2};
use cv_convert::{FromCv, IntoCv, TryFromCv, TryIntoCv};
use imageproc::region_labelling::{connected_components, Connectivity};
use nalgebra as na;
use opencv as cv;
use image::{GrayImage, ImageBuffer, Luma};
use nshare::ToNdarray2;


fn main() {
   let args: Vec<String> = env::args().collect();
   let l = args[1].parse::<u16>().unwrap();
   let total = prob_total(&l);
   let res = normalized_prob(0, 0, &l, &total);
   let prob_array = make_probability_array(&l);
   let mut starting_arr = Array2::<i64>::zeros((l as usize, l as usize));
   starting_arr[[1,0]] = 1;
   starting_arr[[2,0]] = 1;
   starting_arr[[4,0]] = 1;
   let test_arr = get_connected_from_arr(starting_arr, l);
   println!("conencted obj: {}", test_arr)
}
