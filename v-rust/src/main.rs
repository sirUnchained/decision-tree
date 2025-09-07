use ordered_float::OrderedFloat;
use std::{collections::HashSet, ops::Index, vec};

struct Node {
    feature_index: i32,
    true_branch: Option<Box<Node>>,
    false_branch: Option<Box<Node>>,
    result: Option<i32>,
    val: f32,
}
impl Node {
    pub fn new(
        feature_index: i32,
        true_branch: Option<Box<Node>>,
        false_branch: Option<Box<Node>>,
        result: Option<i32>,
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
    if y_set.len() == 1 {
        return Node::new(0, None, None, Some(y[0]), 0.);
    }

    let mut best_gain: f32 = 0.;
    let mut best_criteria: Option<(usize, f32)> = Option::None;
    let mut best_sets: Option<(Vec<Vec<f32>>, Vec<i32>, Vec<Vec<f32>>, Vec<i32>)> = Option::None;

    let mut n_features = 0;
    if x.len() != 0 {
        n_features = x.index(0).len();
    }
    let current_entropy: f32 = entropy(y.clone());

    for feature in 0..n_features {
        let feature_values_vec: Vec<f32> = get_features_of_column(x.clone(), feature as i32);
        let feature_values_set: HashSet<OrderedFloat<f32>> =
            feature_values_vec.into_iter().map(OrderedFloat).collect();

        for value in feature_values_set {
            let (true_x, true_y, false_x, false_y) =
                split_data(x.clone(), y.clone(), feature as i32, value.into());
            let true_entropy = entropy(true_y.clone());
            let false_entropy = entropy(false_y.clone());
            let p = true_y.len() as f32 / y.len() as f32;
            let gain = current_entropy - p * true_entropy - (1.0 - p) * false_entropy;
            if gain > best_gain {
                best_gain = gain;
                best_criteria = Some((feature, value.into()));
                best_sets = Some((true_x, true_y, false_x, false_y));
            }
        }
    }

    if best_gain > 0. {
        let mut true_branch = None;
        let mut false_branch = None;
        match best_sets {
            None => {}
            Some(best_sets) => {
                true_branch = Some(Box::new(build_tree(best_sets.0, best_sets.1)));
                false_branch = Some(Box::new(build_tree(best_sets.2, best_sets.3)));
            }
        }

        match best_criteria {
            None => {
                return Node::new(0, None, None, Some(y[0]), 0.);
            }
            Some(best_criteria) => {
                return Node::new(
                    best_criteria.0 as i32,
                    true_branch,
                    false_branch,
                    Some(y[0]),
                    best_criteria.1,
                );
            }
        }
    }

    return Node::new(0, None, None, None, 0.);
}

fn split_data(
    x: Vec<Vec<f32>>,
    y: Vec<i32>,
    feature: i32,
    value: f32,
) -> (Vec<Vec<f32>>, Vec<i32>, Vec<Vec<f32>>, Vec<i32>) {
    let feature_arr = get_features_of_column(x.clone(), feature);

    let mut true_indices: Vec<i32> = Vec::new();
    for i in 0..feature_arr.len() {
        if feature_arr[i] > value {
            true_indices.push(i.clone() as i32);
        }
    }
    let true_indices = true_indices;

    let mut false_indices: Vec<i32> = Vec::new();
    for i in 0..feature_arr.len() {
        if feature_arr[i] <= value {
            false_indices.push(i as i32);
        }
    }
    let false_indices = false_indices;

    let mut true_x = Vec::new();
    let mut true_y = Vec::new();

    for elem in true_indices {
        true_x.push(x[elem as usize].clone());
        true_y.push(y[elem as usize]);
    }

    let mut false_x = Vec::new();
    let mut false_y = Vec::new();

    for elem in false_indices {
        false_x.push(x[elem as usize].clone());
        false_y.push(y[elem as usize]);
    }

    return (true_x, true_y, false_x, false_y);
}

fn get_features_of_column(datas: Vec<Vec<f32>>, column_index: i32) -> Vec<f32> {
    let mut features: Vec<f32> = Vec::new();

    for i in 0..datas[0].len() {
        features.push(datas[i][column_index as usize]);
    }

    features
}

fn entropy(data: Vec<i32>) -> f32 {
    let max = data.len() as f32;
    let count: Vec<i32> = bincount(data);
    let probabilities: Vec<f32> = count.iter().map(|&x| x as f32 / max).collect();

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
    let mut max: i32 = 0;
    match x.iter().max() {
        None => {}
        Some(val) => {
            max = *val;
        }
    };

    let mut l: Vec<i32> = Vec::new();

    let mut count_of_index_num;
    for i in 0..=max {
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

fn predict(tree: Node, sample: Vec<f32>) -> i32 {
    match tree.result {
        None => {
            let mut branch = tree.false_branch;
            if sample[tree.feature_index as usize] <= tree.val {
                match branch {
                    None => {
                        return 0;
                    }
                    Some(v) => {
                        branch = v.true_branch;
                    }
                }
            }
            match branch {
                None => {
                    return 0;
                }
                Some(v) => {
                    println!("{:#?}", v.result);
                    return predict(*v, sample);
                }
            }
        }
        Some(v) => {
            println!("{}", v);
            return v;
        }
    }
}

fn main() {
    let x = vec![vec![1., 1.], vec![1., 0.], vec![0., 1.], vec![0., 0.]];
    let y = Vec::from([1, 1, 0, 0]);

    let tree = build_tree(x, y);

    let sample = vec![0., 0.];
    println!("{}", predict(tree, sample));
}
