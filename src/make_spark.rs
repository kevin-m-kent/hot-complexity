//mod std::f64::consts;
use std::f64::consts::E;
use itertools::iproduct; 
//use imageproc::region_labelling::{connected_components, Connectivity};
use ndarray::{Array2};

fn prob_spark(i: i16, j: i16, l: &u16) -> f64 {
    let l = l/10;
    let base: f64 = E;
    let i_use: i16 = i + 1;
    let j_use: i16 = j + 1;
    return base.powf(-i_use as f64/l as f64)*base.powf(-j_use as f64/l as f64);
}

pub fn prob_total(l: &u16) -> f64 {
    let mut total: f64 = 0.0;
    for (i_test, j_test) in iproduct!(0..*l, 0..*l){
        total += prob_spark(i_test as i16, j_test as i16, l);
    }
    return total

}

pub fn normalized_prob(i: i16, j: i16, l: &u16, total: &f64) -> f64 {

    return prob_spark(i, j, &l)/total

}

pub fn make_probability_array(l: &u16) -> Array2::<f64> {

    let mut prob_array = Array2::<f64>::zeros((*l as usize, *l as usize));

    let total = prob_total(l);
    let mut prob_result = 0.0;

    for (i_test, j_test) in iproduct!(0..*l, 0..*l){
        
        prob_result = normalized_prob(i_test as i16, j_test as i16, &l, &total);
        prob_array[[i_test as usize, j_test as usize]] = prob_result;
    }

    prob_array

}


