fn bubble_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        for j in i..array.len() {
            if array[i] > array[j] {
                let temp = array[i];
                array[i] = array[j];
                array[j] = temp;
            }
        }
    }
}

fn main() {
    let mut initial_array = [1, 100, 8, 9, -87, 33, 68, 1, 1025, 999, 43];

    bubble_sort(&mut initial_array);

    println!("{:?}", initial_array);
}
