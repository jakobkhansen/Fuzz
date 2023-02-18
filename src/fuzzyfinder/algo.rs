use std::cmp::{min, Ordering};

use ndarray::Array2;

use super::ui::Picker;

pub fn sort_by_score(picker: &mut Picker) {
    for pick in &mut picker.picks {
        pick.score = score(&picker.input, &pick.element);
    }
    picker.picks.sort_by(|x, y| {
        if x.score > y.score {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
}

pub fn score(input: &String, pick: &String) -> usize {
    if input.len() == 0 {
        return 0;
    }
    return edit_distance(input, pick);
}

pub fn edit_distance(s1: &String, s2: &String) -> usize {
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
