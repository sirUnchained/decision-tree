use ordered_float::OrderedFloat;
use std::{collections::HashSet, ops::Index};

struct Node {
    feature_index: i32,
    true_branch: Option<Box<Node>>,
    false_branch: Option<Box<Node>>,
    result: i32,
    val: f32,
}
impl Node {
    pub fn new(
        feature_index: i32,
        true_branch: Option<Box<Node>>,
        false_branch: Option<Box<Node>>,
        result: i32,
        val: f32,
    ) -> Self {
        Self {
            feature_index,
            true_branch,
            false_branch,
            result,
            val,
        }
    }
}

fn build_tree(x: Vec<Vec<f32>>, y: Vec<i32>) -> Node {
    let y_set: HashSet<i32> = y.iter().cloned().collect();
    if y_set.capacity() == 1 {
        return Node {
            result: y[0],
            true_branch: None,
            false_branch: None,
            feature_index: 0,
            val: 0.,
        };
    }

    let best_gain: i32 = 0;
    let best_criteria: Option<Node> = None;
    let best_sets: Option<Node> = None;
    let n_features: usize = x.index(0).capacity();
    let current_entropy: f32 = entropy(y);

    for feature in 0..n_features {
        let feature_values: Vec<f32> = get_features_of_column(x.clone(), feature as i32);
    }

    return Node {
        result: 9,
        true_branch: None,
        false_branch: None,
        feature_index: 0,
        val: 0.,
    };
}

/*
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
*/

fn split_data(x: Vec<Vec<f32>>, y: Vec<i32>, feature: i32, value: f32) {
    let feature_arr = get_features_of_column(x.clone(), feature);

    let true_indices: Vec<f32> = feature_arr.iter().find_map(|&x| x > value).collect();
    // let false_indices = feature_arr.iter().filter(|&&x| x <= value).collect();
}

fn get_features_of_column(datas: Vec<Vec<f32>>, column_index: i32) -> Vec<f32> {
    let mut features: Vec<f32> = Vec::new();

    for i in 0..datas[0].capacity() {
        features.push(datas[i][column_index as usize]);
    }

    features
}

fn entropy(data: Vec<i32>) -> f32 {
    let max = data.capacity() as f32;
    let count: Vec<i32> = bincount(data);
    let probabilities: Vec<f32> = count.iter().map(|&x| x as f32 / max).collect();
    println!("{:#?}", probabilities);

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

    return l;
}

fn main() {}
