extern crate rand;

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

fn get_user_items_count() -> i32 {
    use std::io::{stdin, stdout, Write};

    print!("Enter the count of sorted items: ");

    let _ = stdout().flush();

    let mut input_string = String::new();

    stdin()
        .read_line(&mut input_string)
        .expect("Wrong string entered!");

    let input_string = input_string.trim();

    match input_string.parse() {
        Ok(items_count) => return items_count,
        Err(_) => {
            println!("You must enter a number!");
            return -1;
        }
    };
}

fn generate_random_vector(items_count: i32) -> Vec<i32> {
    use rand::Rng;

    (0..items_count)
        .map(|_| rand::thread_rng().gen_range(-1000, 1001))
        .collect()
}

fn main() {
    let items_count = get_user_items_count();

    if items_count <= 0 {
        return;
    }

    let mut initial_vec = generate_random_vector(items_count);

    let greater_closure = |x, y| x > y;

    bubble_sort(&mut initial_vec, greater_closure);

    println!("Sorted items: {:?}", initial_vec);
}
