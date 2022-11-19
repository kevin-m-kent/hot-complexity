//mod std::f64::consts;
use std::f64::consts::E;
use itertools::iproduct; 

pub fn prob_spark(i: i16, j: i16, l: u16) -> f64 {
    let l = l/10;
    let base: f64 = E;
    let i_use: i16 = i + 1;
    let j_use: i16 = j + 1;
    return base.powf(-i_use as f64/l as f64)*base.powf(-j_use as f64/l as f64);
}

pub fn prob_total(l: u16) -> f64 {
    let mut total: f64 = 0.0;
    for (i_test, j_test) in iproduct!(0..l, 0..l){
        total += prob_spark(i_test as i16, j_test as i16, l);
    }
    return total

}

pub fn normalized_prob(i: i16, j: i16, l: u16, total: f64) -> f64 {

    return prob_spark(i, j, l)/total

}


