fn main() {
    let some_vec: Vec<i32> = vec![10, 20, 30, 40, 100, 50];

    println!("{}", get_largest_element(&some_vec));
}

fn get_largest_element(int_vec: &[i32]) -> &i32 {
    let mut largest_number: &i32 = &int_vec[0];

    for current_element in int_vec {
        if current_element > largest_number {
            largest_number = current_element;
        }
    }

    largest_number
}
