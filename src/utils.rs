extern crate rand;

use std;

use sort::{BubbleSorter, InsertSorter, QuickSorter, SelectionSorter, SortSlice};

fn get_user_input() -> Result<String, std::io::Error> {
    use std::io::{stdin, stdout, Write};

    let _ = stdout().flush();

    let mut input_string = String::new();

    stdin().read_line(&mut input_string)?;

    Ok(input_string.trim().to_string())
}

fn get_user_input_int(input_message: &str) -> Result<i32, std::num::ParseIntError> {
    print!("{}: ", input_message);

    let user_input = get_user_input().expect("Error getting user input!");

    user_input.parse::<i32>()
}

pub fn generate_random_vector() -> Vec<i32> {
    use self::rand::Rng;

    let items_count = get_user_input_int("Enter sorting items count")
        .expect("You must enter a valid integer for items count!");

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

pub fn choose_sorting_method() -> Box<SortSlice> {
    print!(
        "\nChoose sorting method:\n\
         1. Bubble sort\n\
         2. Insertion sort\n\
         3. Quicksort\n\
         4. Selection sort\n"
    );

    let chosen_option = get_user_input_int("").expect("You must enter a sorting method number!");

    match chosen_option {
        1 => Box::new(BubbleSorter) as Box<SortSlice>,
        2 => Box::new(InsertSorter) as Box<SortSlice>,
        3 => Box::new(QuickSorter) as Box<SortSlice>,
        4 => Box::new(SelectionSorter) as Box<SortSlice>,
        _ => {
            println!("Wrong option. Choosing default method: quicksort");
            Box::new(QuickSorter) as Box<SortSlice>
        }
    }
}
