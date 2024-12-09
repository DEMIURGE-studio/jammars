/// Reimplementation of MarkovJunior's logic in rust, following [notes](https://gist.github.com/dogles/a926ab890552cc7e45400a930398449d).
use glam::{uvec2, UVec2};
use ndarray::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
//use std::collections::HashSet;

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
    Custom(fn(grid: &mut Grid) -> bool),

    /// `One`` takes a set of rewrite rules.
    /// Each step, it will find all rules that have at least one match on the grid, and apply a random match to apply.
    /// `One` will end if there are no rules left that have any matches.
    One(Vec<Rule>),
    All(Vec<Rule>, usize, usize),
    // I don't think I can implement the prl node in rust at this time.

    Markov(Vec<Rules>),
    Sequence(Vec<Rules>, usize),
    Repeat(usize, usize, Box<Rules>),

    // TODO: start, end, path
    //Path(char, char, char),
}

impl Rules {
    pub fn apply(&mut self, grid: &mut Grid) -> bool {
        match self {
            Self::Rule(rule) => {
                let mut matches = grid.find(rule.pattern.clone(), rule.symmetry);
                rule.apply(grid, &mut matches)
            },
            Self::Custom(rule) => rule(grid),
            Self::One(rules) => {
                let mut matches = Vec::new();
                for (i, rule) in rules.iter_mut().enumerate() {
                    if rule.origin != ' ' {
                        grid.set_origin(rule.origin);
                        rule.origin = ' ';
                    }
                    for m in grid.find(rule.pattern.clone(), rule.symmetry) {
                        matches.push((i, m));
                    }
                }
                if matches.is_empty() {
                    false
                } else {
                    let i = grid.rng.gen_range(0..matches.len());
                    let choice = matches[i].clone();
                    rules[choice.0].apply(grid, &mut vec![choice.1]);
                    true
                }
            },
            Self::All(rules, index, count) => {
                if let Some(rule) = rules.get_mut(*index) {
                    if rule.origin != ' ' {
                        grid.set_origin(rule.origin);
                        rule.origin = ' ';
                    }
                    let mut matches = grid.find(rule.pattern.clone(), rule.symmetry);
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
                        let i = grid.rng.gen_range(0..matches.len());
                        let choice = matches.remove(i);
                        rules[*index].apply(grid, &mut vec![choice.clone()]);
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
            Self::Markov(rules) => {
                for rule in rules {
                    if rule.apply(grid) {
                        return true;
                    }
                }
                false
            },
            Self::Sequence(rules, index) => {
                if !rules[*index].apply(grid) {
                    if *index < rules.len() - 1 {
                        *index += 1;
                        return true;
                    } else {
                        return false;
                    }
                }
                true
            },
            Self::Repeat(repeat, original, rules) => {
                if *repeat > 0 {
                    if rules.apply(grid) {
                        *repeat -= 1;
                        true
                    } else {
                        *repeat = *original;
                        false
                    }
                } else {
                    rules.apply(grid);
                    *repeat = *original;
                    return false;
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub pattern: Pattern,
    pub origin: char,
    pub symmetry: u8,
}

impl Rule {
    pub fn apply(&mut self, grid: &mut Grid, matches: &mut Vec<Match>) -> bool {
        if !matches.is_empty() {
            let i = grid.rng.gen_range(0..matches.len());
            let choice = matches.remove(i);
            //self.pattern.rotate(choice.rot);
            if grid.fits(choice.pos, &choice.pattern) {
                grid.replace(choice.pos, &choice.pattern);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub alphabet: Vec<char>,
    pub size: UVec2,
    pub tiles: Array2<char>,
    pub rng: ChaChaRng,
}

impl Grid {
    pub fn new(width: u32, height: u32, alphabet: &str) -> Self {
        let alphabet: Vec<char> = alphabet.to_uppercase().chars().collect();
        let start = alphabet[0];
        Self {
            alphabet,
            size: uvec2(width, height),
            tiles: Array2::from_elem((width as usize, height as usize), start),
            rng: ChaChaRng::from_entropy(),
        }
    }

    pub fn find(&self, mut pattern: Pattern, symmetry: u8) -> Vec<Match> {
        let original = pattern.clone();
        let mut results = Vec::new();
        let mut rotations = vec![];
        // Default symmetry
        if symmetry & 0b1000 != 0 {
            rotations.extend_from_slice(&[
                Rotation::None,
                Rotation::Clockwise,
                Rotation::Mirror,
                Rotation::Counter,
            ]);
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
            // At least set to check for no rotations...
            rotations.push(Rotation::None);
        }
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                for rotation in &rotations {
                    pattern.rotate(*rotation);
                    if *rotation != Rotation::None {
                        if pattern.find == original.find {
                            continue;
                        }
                    }
                    if self.fits(uvec2(x, y), &pattern) {
                        results.push(Match {
                            pattern: pattern.clone(),
                            pos: uvec2(x, y),
                        });
                    }
                }
            }
        }
        results
    }

    pub fn fits(&self, pos: UVec2, pattern: &Pattern) -> bool {
        let mut matching = true;
        for ((y, x), &find) in pattern.find.indexed_iter() {
            if let Some(&tile) = self.get(pos.x as usize + x, pos.y as usize + y) {
                if tile != find && !(tile == '*' || find == '*') {
                    matching = false;
                    break;
                }
            } else {
                matching = false;
                break;
            }
        }
        matching
    }

    pub fn replace(&mut self, pos: UVec2, pattern: &Pattern) {
        for ((y, x), &replace) in pattern.replace.indexed_iter() {
            if let Some(tile) = self.get_mut(pos.x as usize + x, pos.y as usize + y) {
                if replace != '*' {
                    *tile = replace;
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.tiles.get((x, y))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char> {
        self.tiles.get_mut((x, y))
    }

    pub fn set_origin(&mut self, origin: char) {
        if let Some(tile) = self.get_mut(self.size.x as usize / 2, self.size.y as usize / 2) {
            *tile = origin;
        }
    }
}

/// Instance of a matching pattern on a grid
#[derive(Clone, Debug)]
pub struct Match {
    /// The Rule which this is a match for
    pub pattern: Pattern,
    /// The position of the top left corner of match
    pub pos: UVec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rotation {
    Clockwise, // 90
    Counter, // 270
    Mirror, // 180
    None, // 0
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Char {
    pub cell: char,
    pub rgb: [u8; 3],
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub current: Rotation,
    pub find: Array2<char>,
    pub replace: Array2<char>,
}

impl Pattern {
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

    fn _rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Clockwise => {
                self.find.swap_axes(0, 1);
                self.replace.swap_axes(0, 1);
                self.find.invert_axis(Axis(1));
                self.replace.invert_axis(Axis(1));
            },
            Rotation::Counter => {
                self.find.swap_axes(0, 1);
                self.replace.swap_axes(0, 1);
                self.find.invert_axis(Axis(0));
                self.replace.invert_axis(Axis(0));
            },
            Rotation::Mirror => {
                self.find.invert_axis(Axis(1));
                self.replace.invert_axis(Axis(1));
                self.find.invert_axis(Axis(0));
                self.replace.invert_axis(Axis(0));
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