use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = one![W:WBB > WAW];
    visualize::runner(&mut rules);
}