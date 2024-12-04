use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = standard![
        one![R:RBB > GGR],
        one![RGG > WWR],
    ];
    visualize::runner(&mut rules);
}