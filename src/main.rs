mod sort;

mod utils;

use utils::{choose_comparison_closure, choose_sorting_method, generate_random_vector};

fn main() {
    let mut initial_vec = generate_random_vector();

    let comparison_closure = choose_comparison_closure();

    let sorting_method = choose_sorting_method();

    sorting_method(&mut initial_vec, &comparison_closure);

    println!("Sorted items: {:?}", initial_vec);
}
