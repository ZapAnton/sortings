extern crate rand;

fn bubble_sort(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) {
    for i in 0..array.len() {
        for j in i..array.len() {
            if comparison_closure(array[i], array[j]) {
                let temp = array[i];
                array[i] = array[j];
                array[j] = temp;
            }
        }
    }
}

fn partition(array: &mut [i32], low: usize, high:usize) -> usize {
    1
}

fn quick_sort_p(array: &mut [i32], low: usize, high: usize) {
    if low < high {
        let partition_index = partition(array, low, high);
        
        quick_sort_p(array, low, partition_index - 1); 

        quick_sort_p(array, partition_index + 1, high); 
    }
}

fn quick_sort(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) {
    let n = array.len();

    quick_sort_p(array, 0, n);
}

fn insertion_sort(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) {
    for i in 1..array.len() {
        let key = array[i];

        let mut j = i;

        while j > 0 && comparison_closure(key, array[j - 1]) {
            array[j] = array[j - 1];

            j -= 1;
        }

        array[j] = key;
    }
}

fn selection_sort(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) {
    for i in 0..array.len() {
        let mut minimum_index = i;

        for j in (i + 1)..array.len() {
            if comparison_closure(array[j], array[minimum_index]) {
                minimum_index = j;
            }
        }

        if i != minimum_index {
            let temp = array[minimum_index];

            array[minimum_index] = array[i];

            array[i] = temp;
        }
    }
}

fn get_user_input() -> String {
    use std::io::{stdin, stdout, Write};

    let _ = stdout().flush();

    let mut input_string = String::new();

    stdin()
        .read_line(&mut input_string)
        .expect("Wrong string entered!");

    input_string.trim().to_string()
}

fn get_user_items_count() -> i32 {
    print!("Enter the count of sorted items: ");

    let input_string = get_user_input();

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

fn choose_comparison_closure() -> Box<Fn(i32, i32) -> bool> {
    print!(
        "\nChoose comparison method:\n\
         1. > (Greater than)\n\
         2. < (Less than)\n\
         3. >= (Greater than or equals)\n\
         4. <= (Less than or equals)\n> "
    );

    let input_string = get_user_input();

    let chosen_option: i32 = input_string.parse().unwrap();

    match chosen_option {
        1 => Box::new(|x, y| x > y),
        2 => Box::new(|x, y| x < y),
        3 => Box::new(|x, y| x >= y),
        4 => Box::new(|x, y| x <= y),
        _ => {
            println!("Wrong option. Choosing default method: > (Greater than)");
            Box::new(|x, y| x > y)
        }
    }
}

fn main() {
    let items_count = get_user_items_count();

    if items_count <= 0 {
        return;
    }

    let mut initial_vec = generate_random_vector(items_count);

    let comparison_closure = choose_comparison_closure();

    insertion_sort(&mut initial_vec, &comparison_closure);

    println!("Sorted items: {:?}", initial_vec);
}
