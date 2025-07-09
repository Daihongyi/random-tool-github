// random_generator.rs
use rand::Rng;
use std::fs;
use std::collections::HashSet;

pub struct RandomGenerator {
    core_version:String,
    lower_bound: i64,
    upper_bound: i64,
    pub(crate) num_to_generate: usize,
    pub(crate) allow_duplicates: bool,
    generated_numbers: Vec<i64>,
}

impl RandomGenerator {
    pub fn new() -> Self {
        Self {
            core_version: "v1.0".parse().unwrap(),
            lower_bound: 0,
            upper_bound: 1024,
            num_to_generate: 1,
            allow_duplicates: false,
            generated_numbers: Vec::new(),
        }
    }

    pub fn set_lower_bound(&mut self, lower: i64) {
        self.lower_bound = lower;
    }

    pub fn set_upper_bound(&mut self, upper: i64) {
        self.upper_bound = upper;
    }

    pub fn set_num_to_generate(&mut self, num: usize) {
        self.num_to_generate = num;
    }

    pub fn set_allow_duplicates(&mut self, allow: bool) {
        self.allow_duplicates = allow;
    }

    pub fn get_allow_duplicates(&self) -> bool {
        self.allow_duplicates
    }

    pub fn generate_numbers(&mut self) {
        let mut rng = rand::rng();
        self.generated_numbers.clear();

        if self.lower_bound > self.upper_bound {
            return;
        }

        if !self.allow_duplicates {
            let range_size = (self.upper_bound - self.lower_bound + 1) as usize;
            if self.num_to_generate > range_size {
                return;
            }

            let mut unique_set = HashSet::new();
            while unique_set.len() < self.num_to_generate {
                let num = rng.random_range(self.lower_bound..=self.upper_bound);
                unique_set.insert(num);
            }
            self.generated_numbers = unique_set.into_iter().collect();
        } else {
            for _ in 0..self.num_to_generate {
                let num = rng.random_range(self.lower_bound..=self.upper_bound);
                self.generated_numbers.push(num);
            }
        }
    }

    pub fn clear_numbers(&mut self) {
        self.generated_numbers.clear();
    }

    pub fn get_numbers(&self) -> &Vec<i64> {
        &self.generated_numbers
    }

    pub fn get_bounds(&self) -> (i64, i64) {
        (self.lower_bound, self.upper_bound)
    }

    pub fn get_settings(&self) -> (usize, bool) {
        (self.num_to_generate, self.allow_duplicates)
    }

    pub fn save_numbers(&self, filename: &str) -> std::io::Result<()> {
        let content = self.generated_numbers
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(filename, content)
    }

    pub fn get_core_version(&self) -> &str {
        &self.core_version
    }
}
