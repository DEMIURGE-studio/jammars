use jammars::*;

mod visualize;

fn main() {
    let mut rules = sequence![
        steps![30, one![B > E]],
        steps![30, one![B > Y]],
        all![EB > *E, YB > *Y],
    ];
    visualize::runner(&mut rules);
}