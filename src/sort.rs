pub trait SortSlice {
    fn sort(&mut [i32]);
}

pub struct BubbleSorter;
pub struct InsertSorter;
pub struct SelectionSorter;
pub struct QuickSorter;

impl SortSlice for BubbleSorter {
    fn sort(slice: &mut [i32]) {
        for i in 0..slice.len() {
            for j in i..slice.len() {
                if slice[i] < slice[j] {
                    slice.swap(i, j);
                }
            }
        }
    }
}

impl SortSlice for SelectionSorter {
    fn sort(slice: &mut [i32]) {
        for i in 0..slice.len() {
            let mut minimum_index = i;

            for j in (i + 1)..slice.len() {
                if slice[j] < slice[minimum_index] {
                    minimum_index = j;
                }
            }

            if i != minimum_index {
                slice.swap(minimum_index, i);
            }
        }
    }
}

impl SortSlice for InsertSorter {
    fn sort(slice: &mut [i32]) {
        for i in 1..slice.len() {
            let key = slice[i];

            let mut j = i;

            while j > 0 && key < slice[j - 1] {
                slice[j] = slice[j - 1];

                j -= 1;
            }

            slice[j] = key;
        }
    }
}

fn partition_slice(slice: &mut [i32]) -> usize {
    let len = slice.len();

    let pivot_index = len / 2;

    slice.swap(pivot_index, len - 1);

    let mut store_index = 0;

    for i in 0..len - 1 {
        if slice[i] < slice[len - 1] {
            slice.swap(i, store_index);

            store_index += 1;
        }
    }

    slice.swap(store_index, len - 1);

    store_index
}

impl SortSlice for QuickSorter {
    fn sort(slice: &mut [i32]) {
        let len = slice.len();

        if len >= 2 {
            let pivot_index = partition_slice(slice);

            Self::sort(&mut slice[0..pivot_index]);

            Self::sort(&mut slice[pivot_index + 1..len]);
        }
    }
}

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

#[cfg(test)]
mod tests {

    use super::*;

    fn test_closure<F>(closure: F) -> bool
    where
        F: Fn(i32, i32) -> bool,
    {
        closure(4, 6)
    }

    #[test]
    fn selection_sort_test() {
        let mut test_array = [3, 1, 2, 6, 5, 4];

        let comparison_closure = |x, y| x < y;

        assert!(test_closure(&comparison_closure));
        assert!(test_closure(&comparison_closure));
    }

}
