#![allow(dead_code)]
use std::{
    cell::RefCell,
    collections::VecDeque,
    time::Instant,
};

pub struct UpdatesCounter {
    pub start: Instant,
    times: RefCell<VecDeque<f32>>,
}

impl UpdatesCounter {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            times: RefCell::new(VecDeque::new()),
        }
    }

    pub fn update(&self) -> usize {
        let now = self.start.elapsed().as_secs_f32();
        match self.times.try_borrow_mut() {
            Ok(mut data) => {
                while !data.is_empty() && *data.front().unwrap() < now - 1.0 {
                    data.pop_front();
                }
                data.push_back(now);
                data.len()
            },
            _ => 0,
        }
    }
}