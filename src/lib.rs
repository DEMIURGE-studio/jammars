//! This crate is a reimplementation of logic used by [Markov Junior].
//! 
//! The logic that has been implemented has mostly been done following the [technical notes] linked in [Markov Junior]'s README.
//! So far, the One, All, Markov and Sequence nodes should be working as expected, there are plans to implement the other nodes in the future.
//! 
//! # Example
//! ```
//! use jammars::*;
//! 
//! let mut count = 0;
//! let mut grid = Grid::new(100, 100, "BWA");
//! let mut rules = one![W:WBB > WAW];
//! while rules.apply(&mut grid) {
//!     count += 1;
//! }
//! 
//! println!("Rules finished after {} steps", count);
//! ```
//! 
//! [Markov Junior]: https://github.com/mxgmn/MarkovJunior
//! [technical notes]: https://gist.github.com/dogles/a926ab890552cc7e45400a930398449d

use rand::prelude::*;
use std::cell::Cell;

pub use rule_macros::*;

#[macro_use]
mod macros;

/// Rules is a tree structure where different nodes perform different types of operations and/or
/// influence which of their child nodes are executed at any point.
/// 
/// Each step asks the currently executing node to apply it's transformation, or
/// return false if the node has more work to do.
#[derive(Clone, Debug)]
pub enum Rules {
    /// Rewrite rule
    Rule(Rule),

    /// `One`` takes a set of rewrite rules.
    /// Each step, it will find all rules that have at least one match on the grid, and apply a random match to apply.
    /// `One` will end if there are no rules left that have any matches.
    One(Vec<Rule>),
    All(Vec<Rule>, usize, usize),
    // I don't think I can implement the prl node in rust at this time.

    Markov(Vec<Rules>),
    Sequence(Vec<Rules>, usize),
    Steps(usize, usize, Box<Rules>),

    // TODO: start, end, path
    //Path(char, char, char),
}

impl Rules {
    pub fn apply<G: Grid, R: RngCore>(&mut self, grid: &mut G, rng: &mut R) -> bool {
        match self {
            // Applies single rule
            Self::Rule(rule) => {
                let mut matches = grid.find_matches(&rule.pattern, rule.symmetry);
                rule.apply(grid, rng, &mut matches)
            },
            // Finds all matches for every rule and applies one at random each step
            Self::One(rules) => {
                let mut matches = Vec::new();
                for (i, rule) in rules.iter_mut().enumerate() {
                    if rule.origin != ' ' {
                        grid.set_origin(rule.origin);
                        rule.origin = ' ';
                    }
                    for m in grid.find_matches(&rule.pattern, rule.symmetry) {
                        matches.push((i, m));
                    }
                }
                if matches.is_empty() {
                    false
                } else {
                    let i = rng.gen_range(0..matches.len());
                    let choice = matches[i].clone();
                    rules[choice.0].apply(grid, rng, &mut vec![choice.1]);
                    true
                }
            },
            // Applies all matches of each rule, one at a time, in order, in a single step
            Self::All(rules, index, count) => {
                if let Some(rule) = rules.get_mut(*index) {
                    if rule.origin != ' ' {
                        grid.set_origin(rule.origin);
                        rule.origin = ' ';
                    }
                    let mut matches = grid.find_matches(&rule.pattern, rule.symmetry);
                    if matches.is_empty() {
                        return if *index + 1 < rules.len() {
                            *index += 1;
                            true
                        } else {
                            false
                        };
                    } else {
                        *count += matches.len();
                    }
                    while !matches.is_empty() {
                        let i = rng.gen_range(0..matches.len());
                        let choice = matches.remove(i);
                        rules[*index].apply(grid, rng, &mut vec![choice.clone()]);
                    }
                    *index += 1;
                    true
                } else {
                    if *count > 0 {
                        *index = 0;
                        true
                    } else {
                        false
                    }
                }
            },
            // Attempts to apply each rule in order. Stops only when all rules cannot be applied
            Self::Markov(rules) => {
                for rule in rules {
                    if rule.apply(grid, rng) {
                        return true;
                    }
                }
                false
            },
            // Applies a rule until it can't be applied anymore, then moves on to the next rule
            Self::Sequence(rules, index) => {
                if !rules[*index].apply(grid, rng) {
                    if *index < rules.len() - 1 {
                        *index += 1;
                        return true;
                    } else {
                        return false;
                    }
                }
                true
            },
            // Sets a limit of steps for any node
            Self::Steps(repeat, original, rules) => {
                if *repeat > 0 {
                    if rules.apply(grid, rng) {
                        *repeat -= 1;
                        true
                    } else {
                        *repeat = *original;
                        false
                    }
                } else {
                    rules.apply(grid, rng);
                    *repeat = *original;
                    return false;
                }
            },
        }
    }
}

/// The core of all logic
#[derive(Clone, Debug)]
pub struct Rule {
    pub pattern: Pattern,
    pub origin: char,
    pub symmetry: u8,
}

impl Rule {
    /// Apply a single match to the Grid
    pub fn apply<G: Grid, R: RngCore>(&mut self, grid: &mut G, rng: &mut R, matches: &mut Vec<Match>) -> bool {
        if !matches.is_empty() {
            let i = rng.gen_range(0..matches.len());
            let choice = matches.remove(i);
            //self.pattern.rotate(choice.rot);
            if grid.check_pattern(choice.x, choice.y, &choice.pattern) {
                grid.replace_pattern(choice.x, choice.y, &choice.pattern);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub enum Symmetry {
    X, Y, Z, W,
}

/// Trait interface to grids
pub trait Grid {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    //fn symmetry(&self) -> &[Symmetry];
    fn get(&self, x: usize, y: usize) -> Option<char>;
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char>;

    fn set_origin(&mut self, origin: char) {
        if let Some(tile) = self.get_mut(self.width() / 2, self.height() / 2) {
            *tile = origin;
        }
    }

    fn find_matches(&self, pattern: &Pattern, symmetry: u8) -> Vec<Match> {
        let mut temp = pattern.clone();
        let mut results = Vec::new();
        let mut rotations = Vec::new();
        // Check if symmetry is default
        if symmetry & 0b1000 != 0 {
            rotations = vec![
                Rotation::None,
                Rotation::Clockwise,
                Rotation::Mirror,
                Rotation::Counter,
            ];
        } else {
            // Axis 1, innermost axis, presumably x?
            if symmetry & 0b1 != 0 {
                rotations.push(Rotation::None);
                rotations.push(Rotation::Mirror);
            }
            // Axis 0, outermost axis, presumably y?
            if symmetry & 0b10 != 0 {
                rotations.push(Rotation::Clockwise);
                rotations.push(Rotation::Counter);
            }
            // Axis 3, nonexistent axis, future proofing 3d grammars
            if symmetry & 0b100 != 0 {
                todo!()
            }
        }
        if rotations.is_empty() {
            rotations = vec![Rotation::None];
        }
        for y in 0..self.height() {
            for x in 0..self.width() {
                for rotation in &rotations {
                    temp.rotate(*rotation);
                    // Check if the pattern is the same as the original, in which case, we don't want duplicate matches.
                    if *rotation != Rotation::None {
                        if temp.find.array == pattern.find.array {
                            continue;
                        }
                    }
                    if self.check_pattern(x, y, &pattern) {
                        results.push(Match {
                            pattern: pattern.clone(),
                            x, y,
                        });
                    }
                }
            }
        }
        results
    }

    /// Checks if a provided Pattern fits at the given coordinates
    fn check_pattern(&self, x: usize, y: usize, pattern: &Pattern) -> bool {
        for tx in 0..pattern.find.width() {
            for ty in 0..pattern.find.height() {
                let Some(find) = pattern.find.get(tx, ty) else {
                    return false;
                };
                let Some(tile) = self.get(x + tx, y + ty) else {
                    return false;
                };
                if tile != find && !(tile == '*' || find == '*') {
                    return false;
                }
            }
        }
        true
    }

    fn replace_pattern(&mut self, x: usize, y: usize, pattern: &Pattern) {
        for tx in 0..pattern.replace.width() {
            for ty in 0..pattern.replace.height() {
                let Some(replace) = pattern.replace.get(tx, ty) else {
                    return;
                };
                if let Some(tile) = self.get_mut(x + tx, y + ty) {
                    if replace != '*' {
                        *tile = replace;
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Grammar {
    axes: [Cell<bool>; 2],
    swapped: Cell<bool>,
    pub array: Vec<Vec<char>>,
}

impl Grammar {
    pub fn new(array: &[&[char]]) -> Self {
        let mut vec = vec![];
        for l in array {
            vec.push(l.to_vec());
        }
        Self {
            axes: [Cell::new(false), Cell::new(false)],
            swapped: Cell::new(false),
            array: vec,
        }
    }

    fn invert_axis(&self, i: usize) {
        let v = self.axes[i].get();
        self.axes[i].replace(!v);
    }

    fn swap_axes(&self) {
        self.swapped.replace(!self.swapped.get());
    }
}

impl Grid for Grammar {
    fn width(&self) -> usize {
        if self.swapped.get() {
            self.array.len()
        } else {
            self.array[0].len()
        }
    }

    fn height(&self) -> usize {
        if self.swapped.get() {
            self.array[0].len()
        } else {
            self.array.len()
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        let x = if self.axes[if self.swapped.get() { 1 } else { 0 }].get() {
            self.width() - x - 1
        } else {
            x
        };
        let y = if self.axes[if self.swapped.get() { 0 } else { 1 }].get() {
            self.height() - y - 1
        } else {
            y
        };
        let Some(outer) = self.array.get(if self.swapped.get() { x } else { y }) else {
            return None;
        };
        outer.get(if self.swapped.get() { y } else { x }).copied()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char> {
        let y = if self.axes[if self.swapped.get() { 1 } else { 0 }].get() {
            self.height() - y - 1
        } else {
            y
        };
        let x = if self.axes[if self.swapped.get() { 0 } else { 1 }].get() {
            self.width() - x - 1
        } else {
            x
        };
        let Some(outer) = self.array.get_mut(if self.swapped.get() { y } else { x }) else {
            return None;
        };
        outer.get_mut(if self.swapped.get() { x } else { y })
    }
}

/// Instance of a matching pattern on a grid
#[derive(Clone, Debug)]
pub struct Match {
    /// The Rule which this is a match for
    pub pattern: Pattern,
    /// The position of the top left corner of match
    pub x: usize,
    pub y: usize,
}

/// Rotations of a Pattern
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rotation {
    Clockwise, // 90
    Counter, // 270
    Mirror, // 180
    None, // 0
}

/// The arrays of chars to find matches and apply replacements.
#[derive(Clone, Debug)]
pub struct Pattern {
    pub current: Rotation,
    pub find: Grammar,
    pub replace: Grammar,
}

impl Pattern {
    /// Applies a rotation based on the current rotation
    pub fn rotate(&mut self, rotation: Rotation) {
        use Rotation::*;
        self._rotate(match (self.current, rotation) {
            (None, new) => new,
            (Clockwise, None) => Counter,
            (Clockwise, Mirror) => Clockwise,
            (Clockwise, Counter) => Mirror,
            (Mirror, None) => Mirror,
            (Mirror, Clockwise) => Counter,
            (Mirror, Counter) => Clockwise,
            (Counter, None) => Clockwise,
            (Counter, Clockwise) => Mirror,
            (Counter, Mirror) => Counter,
            _ => None,
        });
        self.current = rotation;
    }

    /// Applies the provided rotation directly
    fn _rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Clockwise => {
                self.find.swap_axes();
                self.replace.swap_axes();
                self.find.invert_axis(1);
                self.replace.invert_axis(1);
            },
            Rotation::Counter => {
                self.find.swap_axes();
                self.replace.swap_axes();
                self.find.invert_axis(0);
                self.replace.invert_axis(0);
            },
            Rotation::Mirror => {
                self.find.invert_axis(1);
                self.replace.invert_axis(1);
                self.find.invert_axis(0);
                self.replace.invert_axis(0);
            },
            Rotation::None => {},
        }
    }
}

pub fn alphabet_color(cell: char) -> [u8; 3] {
    match cell {
        'B' => [0x00, 0x00, 0x00], //Black
        'I' => [0x1D, 0x2B, 0x53], //Indigo
        'P' => [0x7E, 0x25, 0x53], //Purple
        'E' => [0x00, 0x87, 0x51], //Emerald
        'N' => [0xAB, 0x52, 0x36], //browN
        'D' => [0x5F, 0x57, 0x4F], //Dead
        'A' => [0xC2, 0xC3, 0xC7], //Alive
        'W' => [0xFF, 0xF1, 0xE8], //White
        'R' => [0xFF, 0x00, 0x4D], //Red
        'O' => [0xFF, 0xA3, 0x00], //Orange
        'Y' => [0xFF, 0xEC, 0x27], //Yellow
        'G' => [0x00, 0xE4, 0x36], //Green
        'U' => [0x29, 0xAD, 0xFF], //blUe
        'S' => [0x83, 0x76, 0x9C], //Slate
        'K' => [0xFF, 0x77, 0xA8], //pinK
        'F' => [0xFF, 0xCC, 0xAA], //Fawn
        'C' => [0x00, 0xFF, 0xFF], //Cyan
        'H' => [0xE4, 0xBB, 0x40], //Honey
        'J' => [0x4B, 0x69, 0x2F], //Jungle
        'L' => [0x84, 0x7E, 0x87], //Light
        'M' => [0xFF, 0x00, 0xFF], //Magenta
        'Q' => [0x9B, 0xAD, 0xB7], //aQua
        'T' => [0x37, 0x94, 0x6E], //Teal
        'V' => [0x8F, 0x97, 0x4A], //oliVe
        'X' => [0xFF, 0x00, 0x00], //X
        'Z' => [0xFF, 0xFF, 0xFF], //Z
        _ => [0xFF, 0xFF, 0xFF],
    }
}