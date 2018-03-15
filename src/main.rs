fn bubble_sort(array: &[i32]) {
    println!("{:?}", array);
}

fn main() {
    let initial_array = [1, 100, 8, 9, -87, 33, 68, 1025, 999, 43];

    bubble_sort(&initial_array);
}
