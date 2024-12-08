use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = markov![
        one![R:RBB > GGR],
        one![RGG > WWR],
    ];
    visualize::runner(&mut rules);
}