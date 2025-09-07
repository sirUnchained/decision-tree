use ndarray::Array1;
// use std::{collections::HashMap, intrinsics::log2f32, ops::Index};

struct Node {
    feature_index: i32,
    threshold: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    info_gain: i32,
    val: f32,
}
impl Node {
    pub fn new(
        feature_index: i32,
        threshold: i32,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        info_gain: i32,
        val: f32,
    ) -> Self {
        Self {
            feature_index,
            threshold,
            left,
            right,
            info_gain,
            val,
        }
    }
}

struct Decision_tree {
    root: Option<Box<Node>>,
    min_sample_split: u32,
    max_depth: i32,
    num_features: u32,
    num_samples: u32,
    curr_depth: i32,
}
impl Decision_tree {
    pub fn new(min_sample_split: u32, max_depth: i32) -> Self {
        Self {
            root: None,
            min_sample_split,
            max_depth,
            num_features: 0,
            num_samples: 0,
            curr_depth: 0,
        }
    }

    pub fn build(&mut self, x: Vec<Vec<f32>>, y: Vec<Vec<f32>>) {
        // getting matrix shape
        self.num_samples = x.len() as u32;
        self.num_features = x[0].len() as u32;

        let curr_depth = 0;

        if self.num_samples >= self.min_sample_split && curr_depth <= self.max_depth {}
    }

    fn get_best_split(&mut self, x: Vec<Vec<f32>>, y: Vec<Vec<f32>>) {
        // let mut best_split: HashMap<String, i32> = HashMap::new();

        for feature_index in 0..self.num_features {}
    }
}

fn entropy(data: Vec<i32>) -> f32 {
    let max = *data.iter().max().unwrap() as usize;
    let count: Vec<i32> = bincount(data);
    let probabilities: Vec<f32> = count.iter().map(|&x| x as f32 / max as f32).collect();

    // let mut entropy: Vec<f32> = Vec::with_capacity(max);
    let mut sum: f32 = 0.0;

    for p in probabilities {
        if p as f32 > 0.0 {
            sum += p * p.log2();
        }
    }

    sum *= -1.0;
    return sum;
}

fn bincount(x: Vec<i32>) -> Vec<i32> {
    let max = x.iter().max().unwrap();
    let mut l: Vec<i32> = Vec::with_capacity(*max as usize);

    let mut count_of_index_num;
    for i in 0..=*max {
        count_of_index_num = 0;

        for j in x.iter() {
            if i == *j {
                count_of_index_num += 1;
            }
        }

        l.push(count_of_index_num);
    }

    l
}

fn main() {
    println!("{}", entropy(vec![1, 1, 5, 7, 8, 9]));
}
