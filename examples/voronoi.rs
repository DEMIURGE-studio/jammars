use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = sequence![
        repeat![30, one![B > E]],
        repeat![30, one![B > Y]],
        all![EB > *E, YB > *Y],
    ];
    visualize::runner(&mut rules);
}