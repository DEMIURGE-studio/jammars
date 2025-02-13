use jammars::*;

mod visualize;

fn main() {
    let mut rules = sequence![
        one![;*B* > *A*],
        one![;BA > UU, ;AB > GG],
        one![;UA > UU, ;AG > GG],
        one![U / G > A / A],
        all![G > U],
        one![AU / UA > AA / AA],
        steps![2, all![A / U > A / A]],
        all![*A* / AUA > *** / *A*],
    ];
    visualize::runner(&mut rules);
}