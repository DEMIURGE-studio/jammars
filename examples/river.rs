use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = sequence![
        repeat![5, one![B > W]],
        repeat![5, one![B > R]],
        one![RB > RR, WB > WW],
        all![RW > UU],
        all![W > B, R > B],
        repeat![1, all![UB > UU]],
        all![BU / UB > U* / **],
        all![UB > *G],
        repeat![13, one![B > E]],
        one![EB > *E, GB > *G],
    ];
    visualize::runner(&mut rules);
}