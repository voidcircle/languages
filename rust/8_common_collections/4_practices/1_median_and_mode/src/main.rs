use std::collections::HashMap;

fn main() {
    // assume that the length of the list is odd.

    let example_vec: Vec<i32> = vec![20, 20, -10, 5, 100, 60, -10, -22, -10];

    let median_of_example_vec = get_median(&example_vec);
    let mode_of_example_vec = get_mode(&example_vec);

    dbg!(example_vec);
    println!(
        "Median: {}, Mode: {}",
        median_of_example_vec, mode_of_example_vec
    );
}

// you can replace &Vec<i32> into &[i32]
fn get_median(int_vec: &[i32]) -> i32 {
    let mut sorted_vec = int_vec.to_vec();
    sorted_vec.sort();
    sorted_vec[int_vec.len() / 2]
}

fn get_mode(int_vec: &[i32]) -> i32 {
    let mut element_counter: HashMap<i32, u32> = HashMap::new();

    // AI...
    for &current_element in int_vec {
        *element_counter.entry(current_element).or_insert(1) += 1;
    }

    element_counter
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap_or(0)

    // its me
    //
    //
    //
    // let mut element_counter: HashMap<i32, u32> = HashMap::new();
    //
    // for current_element in int_vec {
    //     element_counter
    //         .entry(*current_element)
    //         .and_modify(|e| *e += 1)
    //         .or_insert(1);
    // }
    //
    // let mut most_frequent: (i32, u32) = (0, 0);
    //
    // for current_element in element_counter {
    //     let is_actual_element_counted_more = current_element.1 > most_frequent.1;
    //
    //     if is_actual_element_counted_more {
    //         most_frequent = current_element;
    //     }
    // }
}
