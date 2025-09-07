use ordered_float::OrderedFloat;
use std::{collections::HashSet, ops::Index, vec};

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

    let mut best_gain: f32 = 0.;
    let mut best_criteria: (usize, f32) = (0, 0.);
    let mut best_sets: (Vec<Vec<f32>>, Vec<i32>, Vec<Vec<f32>>, Vec<i32>) = (
        Vec::from(Vec::new()),
        Vec::new(),
        Vec::from(Vec::new()),
        Vec::new(),
    );
    let n_features: usize = x.index(0).capacity();
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
            let p = true_y.capacity() as f32 / y.capacity() as f32;
            let gain = current_entropy - p * true_entropy - (1.0 - p) * false_entropy;
            if gain > best_gain {
                best_gain = gain;
                best_criteria = (feature, value.into());
                best_sets = (true_x, true_y, false_x, false_y);
            }
        }
    }

    if best_gain > 0. {
        let true_branch = Some(Box::new(build_tree(best_sets.0, best_sets.1)));
        let false_branch = Some(Box::new(build_tree(best_sets.2, best_sets.3)));
        return Node {
            feature_index: best_criteria.0 as i32,
            val: best_criteria.1,
            true_branch: true_branch,
            false_branch: false_branch,
            result: y[0],
        };
    }

    return Node {
        result: y[0],
        true_branch: None,
        false_branch: None,
        feature_index: 0,
        val: 0.,
    };
}

fn split_data(
    x: Vec<Vec<f32>>,
    y: Vec<i32>,
    feature: i32,
    value: f32,
) -> (Vec<Vec<f32>>, Vec<i32>, Vec<Vec<f32>>, Vec<i32>) {
    let feature_arr = get_features_of_column(x.clone(), feature);

    let mut true_indices: Vec<i32> = Vec::new();
    for i in 0..feature_arr.capacity() {
        println!("{}, {}", i, feature_arr.capacity());
        if feature_arr[i] > value {
            true_indices.push(i.clone() as i32);
        }
    }
    let true_indices = true_indices;

    let mut false_indices: Vec<i32> = Vec::new();
    for i in 0..feature_arr.capacity() {
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

    for i in 0..datas[0].capacity() {
        features.push(datas[i][column_index as usize]);
    }

    features
}

fn entropy(data: Vec<i32>) -> f32 {
    let max = data.capacity() as f32;
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

fn main() {
    let x = vec![vec![1., 1.], vec![1., 0.], vec![0., 1.], vec![0., 0.]];
    let y = Vec::from([1, 1, 0, 0]);

    build_tree(x, y);
}
