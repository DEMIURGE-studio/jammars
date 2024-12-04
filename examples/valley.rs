use jammars::*;
use ndarray::array;

mod visualize;

fn main() {
    let mut rules = sequence![
        one![;*B* > *A*],
        one![;BA > UU, ;AB > GG],
        one![;UA > UU, ;AG > GG],
        one![U / G > B / B],
        one![G > U],
        one![BU / UB > BB / BB],
        all![B / U > B / B],
        all![B / U > B / B],
        all![*B* / BUB > *** / *B*],
    ];
    visualize::runner(&mut rules);
}