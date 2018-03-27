mod sort;

mod utils;

use utils::{choose_comparison_closure, choose_sorting_method, generate_random_vector,
            get_user_input_int};

fn main() {
    let items_count = get_user_input_int("Enter sorting items count")
        .expect("You must enter a valid integer for items count!");

    let mut initial_vec = generate_random_vector(items_count);

    let comparison_closure = choose_comparison_closure();

    let sorting_method = choose_sorting_method();

    sorting_method(&mut initial_vec, &comparison_closure);

    println!("Sorted items: {:?}", initial_vec);
}
