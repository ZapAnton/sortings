fn bubble_sort<F>(array: &mut [i32], compare_closure: F)
where
    F: Fn(i32, i32) -> bool,
{
    for i in 0..array.len() {
        for j in i..array.len() {
            if compare_closure(array[i], array[j]) {
                let temp = array[i];
                array[i] = array[j];
                array[j] = temp;
            }
        }
    }
}

fn main() {
    let mut initial_array = [1, 100, 8, 9, -87, 33, 68, 1, 1025, 999, 43];

    let greater_closure = |x, y| x > y;

    bubble_sort(&mut initial_array, greater_closure);

    println!("{:?}", initial_array);
}
