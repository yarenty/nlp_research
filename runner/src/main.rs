use nlp;
use data;

fn main() {
    println!("Hello, world!");
    let point =  data::get_dataset();
    nlp::train(&point);
}
