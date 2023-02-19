use std::cmp::{min, Ordering};

use ndarray::Array2;

use super::ui::Picker;

pub fn sort_by_score(picker: &mut Picker) {
    for pick in &mut picker.picks {
        pick.score = score(&picker.input, &pick.element);
    }

    // TODO what the hell is this
    picker.picks.sort_by(|x, y| {
        if x.score == y.score {
            if x.element.len() <= y.element.len() {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        if x.score > y.score {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

pub fn score(input: &String, pick: &String) -> usize {
    if input.len() == 0 {
        return 0;
    }
    return smith_waterman_score(input, pick);
    // return wagner_fischer_score(input, pick);
}

// Higher score = better
pub fn smith_waterman_score(s1: &String, s2: &String) -> usize {
    const MATCH: i32 = 16;
    const MISMATCH: i32 = i32::MIN;
    const GAP: i32 = -1;

    let s1_chars: Vec<char> = s1.to_lowercase().chars().collect();
    let s2_chars: Vec<char> = s2.to_lowercase().chars().collect();
    let s1_len = s1_chars.len();
    let s2_len = s2_chars.len();
    // eprintln!("Edit distance between {} and {}", s1, s2);
    let mut matrix = Array2::<i32>::zeros((s1_len + 1, s2_len + 1));

    for i in 0..(s1_len + 1) {
        matrix[[i, 0]] = 0;
    }

    for i in 0..(s2_len + 1) {
        matrix[[0, i]] = 0;
    }

    let mut max_score = 0;

    for i in 1..(s1_len + 1) {
        for j in 1..(s2_len + 1) {
            let match_score = if s1_chars[i - 1] == s2_chars[j - 1] {
                MATCH
            } else {
                MISMATCH
            };
            let score = matrix[[i - 1, j - 1]] + match_score;
            let delete = matrix[[i - 1, j]] + GAP;
            let insert = matrix[[i, j - 1]] + GAP;
            matrix[[i, j]] = score.max(delete).max(insert).max(0);
            max_score = max_score.max(matrix[[i, j]]);
        }
    }

    // eprintln!("{:?}", matrix);
    return max_score as usize;
}

// Lower score = better
pub fn wagner_fischer_score(s1: &String, s2: &String) -> usize {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();
    // eprintln!("Edit distance between {} and {}", s1, s2);
    let mut matrix = Array2::<usize>::zeros((s1_len + 1, s2_len + 1));

    for i in 0..(s1_len + 1) {
        matrix[[i, 0]] = i;
    }

    for i in 0..(s2_len + 1) {
        matrix[[0, i]] = i;
    }

    for i in 1..(s1_len + 1) {
        for j in 1..(s2_len + 1) {
            if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
                matrix[[i, j]] = matrix[[i - 1, j - 1]];
            } else {
                matrix[[i, j]] = min(
                    matrix[[i - 1, j - 1]],
                    min(matrix[[i - 1, j]], matrix[[i, j - 1]]),
                ) + 1;
            }
        }
    }

    // eprintln!("{:?}", matrix);
    return matrix[[s1_len, s2_len]];
}
