use std::collections::HashMap;
use bevy_utils::HashSet;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pattern {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<char>,
}

impl Pattern {
    fn init_dirty_indices(&self, dirty_indices: &mut HashSet<usize>) {
        for i in 0..self.tiles.len() {
            dirty_indices.insert(i);
        }
    }

    fn replace_at_position(&mut self, replace: &Pattern, position: usize, dirty_indices: &mut HashSet<usize>) {
        // Check if the position and replace pattern dimensions are valid
        let row_start = position / self.width as usize;
        let col_start = position % self.width as usize;

        if row_start + replace.height as usize > self.height as usize || 
           col_start + replace.width as usize > self.width as usize {
            // If the replace pattern does not fit, do nothing (or handle error)
            return;
        }

        // Replace the corresponding tiles
        for r in 0..replace.height {
            for c in 0..replace.width {
                // Calculate the index in self.tiles to be replaced
                let index_in_self = (row_start + r as usize) * self.width as usize + (col_start + c as usize);
                // Get the corresponding index from replace.tiles
                let index_in_replace = (r * replace.width + c) as usize;
                // Replace the tile
                self.tiles[index_in_self] = replace.tiles[index_in_replace];
                dirty_indices.insert(index_in_self);
            }
        }
    }
    
    fn new() -> Pattern {
        Pattern { width: 0, height: 0, tiles: Vec::new() }
    }

    fn rotate(&self) -> Pattern {
        let mut rotated_tiles = vec![' '; (self.width * self.height) as usize];
        for r in 0..self.height {
            for c in 0..self.width {
                let src_index = (r * self.width + c) as usize;
                let dst_index = ((c * self.height + (self.height - r - 1)) % (self.width * self.height)) as usize;
                rotated_tiles[dst_index] = self.tiles[src_index];
            }
        }
        Pattern {
            width: self.height,
            height: self.width,
            tiles: rotated_tiles,
        }
    }

    fn get_rotations(&self) -> Vec<Pattern> {
        let mut result = Vec::new();
        let mut rotation = self.clone();
        for _ in 0..4 {
            rotation = rotation.rotate();
            result.push(rotation.clone());
        }
        result
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub find: Pattern,
    pub replace: Pattern,
    pub positions: Vec<usize>,
}

impl Rule {
    fn new() -> Rule {
        Rule {
            find: Pattern::new(),
            replace: Pattern::new(),
            positions: Vec::new(),
        }
    }

    fn update_positions(&mut self, tiles: &Pattern, dirty_indices: &HashSet<usize>) {
        // For each index in the pattern, check if it's dirty.
        for &dirty_index in dirty_indices {
            // Calculate the row and column based on the dirty index.
            let row = dirty_index / tiles.width as usize;
            let col = dirty_index % tiles.width as usize;

            // Check surrounding area that could be affected by this dirty index.
            // We need to check all positions that the find pattern could start at and include this tile.
            let start_row = row.saturating_sub((self.find.height - 1) as usize);
            let end_row = (row + (self.find.height - 1) as usize).min(tiles.height as usize - 1);
            let start_col = col.saturating_sub((self.find.width - 1) as usize);
            let end_col = (col + (self.find.width - 1) as usize).min(tiles.width as usize - 1);

            for r in start_row..=end_row {
                for c in start_col..=end_col {
                    // Calculate the potential start index of the find pattern
                    let start_index = r * tiles.width as usize + c;

                    // Verify if the find pattern can fit from this start index.
                    if self.does_pattern_fit(tiles, start_index) {
                        self.positions.push(start_index);
                    }
                }
            }
        }

        // Ensure unique positions (in case multiple dirty tiles affect same areas)
        self.positions.sort_unstable();
        self.positions.dedup();
    }

    /// Helper function to check if the find pattern fits at the given start index in tiles.
    fn does_pattern_fit(&self, tiles: &Pattern, start_index: usize) -> bool {
        let row_start = start_index / tiles.width as usize;
        let col_start = start_index % tiles.width as usize;

        if row_start + self.find.height as usize > tiles.height as usize ||
           col_start + self.find.width as usize > tiles.width as usize {
            return false;
        }

        for r in 0..self.find.height {
            for c in 0..self.find.width {
                let idx_in_self = (row_start + r as usize) * tiles.width as usize + (col_start + c as usize);
                let idx_in_find = (r * self.find.width + c) as usize;
                if tiles.tiles[idx_in_self] != self.find.tiles[idx_in_find] {
                    return false;
                }
            }
        }
        true
    }
    
    fn get_random_position(&mut self, rng: &mut ChaChaRng) -> Option<usize> {
        if self.positions.is_empty() {
            None
        } else {
            let index = rng.gen_range(0..self.positions.len());
            let position = self.positions[index];
            self.positions.remove(index);
            Some(position)
        }
    }

    fn execute(&mut self, rng: &mut ChaChaRng, tiles: &mut Pattern, dirty_indices: &mut HashSet<usize>) -> bool {
        self.update_positions(tiles, dirty_indices);
        let Some(position) = self.get_random_position(rng) else {
            return false;
        };
        tiles.replace_at_position(&self.replace, position, dirty_indices);
        return true;
    }

    fn rotate(&self) -> Rule {
        return Rule { 
            find: self.find.rotate(), 
            replace: self.replace.rotate(),
            positions: Vec::new(),
        }
    }

    pub fn get_rotations(&self) -> Vec<Rule> {
        let mut result = Vec::new();
        let mut rotation = self.clone();
        for _ in 0..4 {
            rotation = rotation.rotate();
            result.push(rotation.clone());
        }
        result
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuleTree {
    // Branches
    StandardSet(Vec<RuleTree>), // Plays the first valid rule until it is no longer valid, then plays the next rule. Repeat.
    SequenceSet(Vec<RuleTree>), // Plays each rule once in order, until no rule is played.
    RandomSet(Vec<RuleTree>), // Plays a random rule from the set once. Tries every rule until one works.
    FiniteRepeat(u32, Box<RuleTree>), // A rule which repeats the contained rule a specified number of times, or until it fails.

    // Leaves
    StandardRule(Rule), // A standard rule.
    ReplaceRule(Rule),  // A single replace rule.
    ReplaceRateRule(f32, Rule),  // A single replace rule.
}

impl RuleTree {
    fn execute(&mut self, rng: &mut ChaChaRng, tiles: &mut Pattern, dirty_indices: &mut HashSet<usize>, config: &mut TileMapGeneratorConfig) -> bool {
        match self {
            // Branches 
            RuleTree::StandardSet(sub_rules) => {
                for sub_rule in sub_rules.iter_mut() {
                    tiles.init_dirty_indices(dirty_indices);
                    while sub_rule.execute(rng, tiles, dirty_indices, config) {}
                }
                true
            },
            RuleTree::SequenceSet(sub_rules) => {
                let mut any_executed = false;
                for sub_rule in sub_rules.iter_mut() {
                    if sub_rule.execute(rng, tiles, dirty_indices, config) {
                        any_executed = true;
                    }
                }
                any_executed
            },
            RuleTree::RandomSet(sub_rules) => {
                use rand::seq::SliceRandom;
                let mut shuffled_rules = sub_rules.clone();
                shuffled_rules.shuffle(rng);

                for sub_rule in shuffled_rules.iter_mut() {
                    if sub_rule.execute(rng, tiles, dirty_indices, config) {
                        return true;
                    }
                }
                false
            },
            RuleTree::FiniteRepeat(num, rule) => {
                for _ in 0..*num {
                    if rule.execute(rng, tiles, dirty_indices, config) {
                        break;
                    }
                }
                false
            },

            // Leaves
            RuleTree::StandardRule(rule) => {
                let result = rule.execute(rng, tiles, dirty_indices);
                result
            },
            RuleTree::ReplaceRule(rule) => {
                rule.update_positions(tiles, dirty_indices);

                let num_positions = rule.positions.len();
                for _ in 0..num_positions {
                    if let Some(position) = rule.get_random_position(rng) {
                        tiles.replace_at_position(&rule.replace, position, dirty_indices);
                    }
                }
                false
            },
            RuleTree::ReplaceRateRule(rate, rule) => {
                rule.update_positions(tiles, dirty_indices);

                let num_positions = rule.positions.len();
                for _ in 0..num_positions {
                    if let Some(position) = rule.get_random_position(rng) {
                        if rng.gen::<f32>() < *rate {
                            tiles.replace_at_position(&rule.replace, position, dirty_indices);
                        }
                    }
                }
                false
            },
        }
    }

    pub fn standard_rule(rule: Rule) -> Self {
        RuleTree::StandardRule(rule)
    }

    pub fn random_rotations(rule: &Rule) -> Self {
        let rotations = rule.get_rotations();

        let mut rules = Vec::new();
        for rotation in rotations {
            rules.push(RuleTree::standard_rule(rotation));
        }
        RuleTree::RandomSet(rules)
    }
    
    pub fn replace_rotations(rule: &Rule) -> Self {
        let rotations = rule.get_rotations();
        let mut rules = Vec::new();
        for rot in rotations.iter() {
            rules.push(RuleTree::ReplaceRule(rot.clone()));
        }
        RuleTree::SequenceSet(rules)
    }

    pub fn replace_rate_rotations(rule: &Rule, rate: f32) -> Self {
        let rotations = rule.get_rotations();

        let mut rules = Vec::new();
        for rotation in rotations {
            rules.push(RuleTree::ReplaceRateRule(rate, rotation));
        }
        RuleTree::RandomSet(rules)
    }
}

pub struct TileMapGenerator {
    pub char1: char,
    pub char2: char,
    pub rules: RuleTree,
    pub tiles: Pattern,
    pub config: TileMapGeneratorConfig,
}

impl TileMapGenerator {
    pub fn new(width: usize, height: usize) -> Self {
        TileMapGenerator {
            char1: 'B',
            char2: 'W',
            rules: RuleTree::StandardSet(Vec::new()),
            tiles: Pattern {
                width: width,
                height: height,
                tiles: Vec::new(),
            },
            config: TileMapGeneratorConfig {
                alphabet: HashMap::new(),
            }
        }
    }

    pub fn standard_background(&mut self) -> & mut Self {
        let size = self.tiles.width * self.tiles.height;
        for _ in 0..size {
            self.tiles.tiles.push(self.char1);
        }
        self
    }

    pub fn with_center_seed(&mut self) -> & mut Self {
        let center_width = self.tiles.width / 2; 
        let center_height = self.tiles.height / 2;
        self.tiles.tiles[(center_height * self.tiles.width + center_width) as usize] = self.char2;
        self
    }

    pub fn set_tile_map_size(&mut self, width: usize, height: usize) -> &mut Self {
        self.tiles.width = width;
        self.tiles.height = height;
        self
    }

    pub fn set_alphabet(&mut self, alphabet: &[(char, [u8; 4])]) -> &mut Self {
        self.char1 = alphabet[0].0;
        self.char2 = alphabet[1].0;
        let mut hashmap = HashMap::new();

        for a in alphabet.iter() {
            hashmap.insert(a.0, a.1);
        }
        self.config.alphabet = hashmap;

        self
    }
    
    pub fn standard_alphabet(&mut self) -> &mut Self {
        self.set_alphabet(&[
            ('B', [0x00, 0x00, 0x00, 0xFF]), // Black
            ('I', [0x4B, 0x00, 0x82, 0xFF]), // Indigo
            ('P', [0x80, 0x00, 0x80, 0xFF]), // Purple
            ('E', [0x50, 0xC8, 0x78, 0xFF]), // Emerald
            ('N', [0xA5, 0x2A, 0x2A, 0xFF]), // browN
            ('D', [0x55, 0x55, 0x55, 0xFF]), // Dead, Dark (Gray)
            ('A', [0x80, 0x80, 0x80, 0xFF]), // Alive, grAy
            ('W', [0xFF, 0xFF, 0xFF, 0xFF]), // White
            ('R', [0xFF, 0x00, 0x00, 0xFF]), // Red
            ('O', [0xFF, 0xA5, 0x00, 0xFF]), // Orange
            ('Y', [0xFF, 0xFF, 0x00, 0xFF]), // Yellow
            ('G', [0x00, 0x80, 0x00, 0xFF]), // Green
            ('U', [0x00, 0x00, 0xFF, 0xFF]), // blUe
            ('S', [0x70, 0x80, 0x90, 0xFF]), // Slate
            ('K', [0xFF, 0xC0, 0xCB, 0xFF]), // pinK
            ('F', [0xE5, 0xAA, 0x70, 0xFF]), // Fawn
        ])
    }

    pub fn set_rules(&mut self, rule: RuleTree) -> &mut Self {
        self.rules = rule;
        self
    }

    pub fn execute(&mut self, rng: &mut ChaChaRng) -> &mut Self {
        self.rules.execute(rng, &mut self.tiles, &mut HashSet::new(), &mut self.config);
        self
    }

    pub fn get_tiles(&self) -> &Pattern {
        &self.tiles
    }
}

pub struct TileMapGeneratorConfig {
    pub alphabet: HashMap<char, [u8; 4]>,
}

