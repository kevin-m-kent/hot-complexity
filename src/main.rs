mod make_spark;
use make_spark::*;

fn main() {
   let l = 10;
   let total = prob_total(l);
   let res = normalized_prob(0, 0, l, total);
   println!("prob: {}", res)
}
