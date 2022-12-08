use std::f64::consts::E;
use itertools::iproduct; 
use imageproc::region_labelling::{connected_components, Connectivity};
use ndarray::{Array2};
use image::{ImageBuffer, Luma};
use std::collections::HashMap;
use rand::Rng; 

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

//implementation from https://stackoverflow.com/questions/56762026/how-to-save-ndarray-in-rust-as-image

pub fn array_to_image(arr: &Array2<u32>) -> ImageBuffer<Luma<u32>, Vec<u32>> {
    assert!(arr.is_standard_layout());

    let (height, width) = arr.dim();
    let arr = arr.clone();
    let raw = arr.into_raw_vec();

    ImageBuffer::<Luma<u32>, Vec<u32>>::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
} 

pub fn get_connected_from_arr(arr: &Array2<u32>, l: u16) -> (HashMap<u32, usize>, Array2<u32>) {

    let image = array_to_image(arr);
    let background_color = Luma([0u32]);
    let connected = connected_components(&image, Connectivity::Four, background_color);
    let connected_raw = connected.as_raw();

    let mut m: HashMap<u32, usize> = HashMap::new();

    for x in connected_raw {
        *m.entry(*x).or_default() += 1;
    }

    let connected_comps = Array2::from_shape_vec((l as usize, l as usize), connected_raw.to_vec()).unwrap();

    return (m, connected_comps)
    

}

pub fn sample_random_indices(arr: &Array2<u32>, l: &u16, n: usize) -> Vec<(usize, usize)> {

    let mut sampled_indices = Vec::new();

    for _ in 0..n {

        let random_index = get_sample_index(arr, l);
        sampled_indices.push(random_index);
    }

    return sampled_indices;
}

pub fn get_sample_index(arr: &Array2::<u32>, l: &u16) -> (usize, usize) {

    let mut matches =  Vec::new();

    for (i_test, j_test) in iproduct!(0..*l, 0..*l){
        
        if arr[[i_test as usize, j_test as usize]] == 0 {

            matches.push((i_test as usize, j_test as usize))

    
        }

    }

        let num = rand::thread_rng().gen_range(0..matches.len());

    return matches[num as usize]
    
    

}

pub fn get_spark_avg_yield(arr: &Array2<u32>, l: u16, prob_arr: &Array2::<f64>) ->  f64 {

    //let arr = arr.clone();
    let total_trees = arr.sum();
    let (mut comp_size_hash, labeled_arr) = get_connected_from_arr(arr, l);
    let mut burn_square = Array2::<f64>::zeros((l as usize, l as usize));

    for (i_test, j_test) in iproduct!(0..l, 0..l){ 

        let i_test_u = i_test as i16;
        let j_test_u = j_test as i16;
        let l_u = l as i16;
        let mut v = vec![(i_test_u, j_test_u),(i_test_u + 1, j_test_u), (i_test_u - 1, j_test_u),
                     (i_test_u, j_test_u + 1), (i_test_u, j_test_u - 1)];
        v.retain(|&i |(i.0 < l_u && i.1 < l_u && i.0 >= 0 && i.1 >= 0));
        let mut grps_present: HashMap<u32, usize> = HashMap::new();
        for elem in v {

            let grp = labeled_arr[[elem.0 as usize, elem.1 as usize]];
            let mut size = comp_size_hash.get(&grp).unwrap();
            *grps_present.entry(grp).or_default() = *size;


        }

        let mut burn_total = 0;

        grps_present.remove(&0);

        let grps_vals = grps_present.values();

        for val in grps_vals {

            burn_total += val;

        }

        burn_square[[i_test as usize, j_test as usize]] = burn_total as f64;


    }

    let burn_prob = burn_square*prob_arr;

    return (total_trees as f64)/(l as f64*l as f64) - burn_prob.sum()/(l as f64*l as f64)
}


