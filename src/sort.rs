pub fn bubble_sort(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) {
    for i in 0..array.len() {
        for j in i..array.len() {
            if comparison_closure(array[i], array[j]) {
                array.swap(i, j);
            }
        }
    }
}

fn partition(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) -> usize {
    let len = array.len();

    let pivot_index = len / 2;

    array.swap(pivot_index, len - 1);

    let mut store_index = 0;

    for i in 0..len - 1 {
        if comparison_closure(array[i], array[len - 1]) {
            array.swap(i, store_index);

            store_index += 1;
        }
    }

    array.swap(store_index, len - 1);

    store_index
}

pub fn quick_sort(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) {
    let len = array.len();

    if len >= 2 {
        let pivot_index = partition(array, comparison_closure);

        quick_sort(&mut array[0..pivot_index], comparison_closure);

        quick_sort(&mut array[pivot_index + 1..len], comparison_closure);
    }
}

pub fn insertion_sort(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) {
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

pub fn selection_sort(array: &mut [i32], comparison_closure: &Box<Fn(i32, i32) -> bool>) {
    for i in 0..array.len() {
        let mut minimum_index = i;

        for j in (i + 1)..array.len() {
            if comparison_closure(array[j], array[minimum_index]) {
                minimum_index = j;
            }
        }

        if i != minimum_index {
            array.swap(minimum_index, i);
        }
    }
}
