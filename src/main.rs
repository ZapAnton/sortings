mod sort;

mod utils;

use utils::{choose_sorting_method, generate_random_vector};

fn main() {
    let mut initial_vec = generate_random_vector();

    /*let comparison_closure = choose_comparison_closure();


    sorting_method(&mut initial_vec, &comparison_closure);
    */

    let sorter = choose_sorting_method();

    let compare = Box::new(|x, y| x > y) as Box<Fn(i32, i32) -> bool>;

    sorter.sort(&mut initial_vec, &compare);

    println!("Sorted items: {:?}", initial_vec);
}
