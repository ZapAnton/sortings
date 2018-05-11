mod sort;

mod utils;

use utils::{choose_comparison_closure, choose_sorting_method, generate_random_vector};

fn main() {
    let mut initial_vec = generate_random_vector();

    let compare = choose_comparison_closure();

    let sorter = choose_sorting_method();

    sorter.sort(&mut initial_vec, &compare);

    println!("Sorted items: {:?}", initial_vec);
}
