extern crate rand;

use std;

use sort::{bubble_sort, insertion_sort, quick_sort, selection_sort};

fn get_user_input() -> Result<String, std::io::Error> {
    use std::io::{stdin, stdout, Write};

    let _ = stdout().flush();

    let mut input_string = String::new();

    stdin().read_line(&mut input_string)?;

    Ok(input_string.trim().to_string())
}

pub fn get_user_input_int(input_message: &str) -> Result<i32, std::num::ParseIntError> {
    print!("{}: ", input_message);

    let user_input = get_user_input().expect("Error getting user input!");

    user_input.parse::<i32>()
}

pub fn generate_random_vector(items_count: i32) -> Vec<i32> {
    use self::rand::Rng;

    (0..items_count)
        .map(|_| rand::thread_rng().gen_range(-1000, 1001))
        .collect()
}

pub fn choose_comparison_closure() -> Box<Fn(i32, i32) -> bool> {
    print!(
        "\nChoose comparison method:\n\
         1. > (Greater than)\n\
         2. < (Less than)\n\
         3. >= (Greater than or equals)\n\
         4. <= (Less than or equals)\n"
    );

    let chosen_option = get_user_input_int("").expect("You must enter a comparison method number!");

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

pub fn choose_sorting_method() -> fn(&mut [i32], &Box<Fn(i32, i32) -> bool>) -> () {
    print!(
        "\nChoose sorting method:\n\
         1. Bubble sort\n\
         2. Insertion sort\n\
         3. Quicksort\n\
         4. Selection sort\n"
    );

    let chosen_option = get_user_input_int("").expect("You must enter a sorting method number!");

    match chosen_option {
        1 => bubble_sort,
        2 => insertion_sort,
        3 => quick_sort,
        4 => selection_sort,
        _ => {
            println!("Wrong option. Choosing default method: quicksort");
            quick_sort
        }
    }
}
