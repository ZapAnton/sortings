pub trait SortSlice {
    fn sort(&self, &mut [i32], &Box<Fn(i32, i32) -> bool>);
}

pub struct BubbleSorter;
pub struct InsertSorter;
pub struct SelectionSorter;
pub struct QuickSorter;

impl SortSlice for BubbleSorter {
    fn sort(&self, slice: &mut [i32], compare: &Box<Fn(i32, i32) -> bool>) {
        for i in 0..slice.len() {
            for j in i..slice.len() {
                if compare(slice[j], slice[i]) {
                    slice.swap(i, j);
                }
            }
        }
    }
}

impl SortSlice for SelectionSorter {
    fn sort(&self, slice: &mut [i32], compare: &Box<Fn(i32, i32) -> bool>) {
        for i in 0..slice.len() {
            let mut minimum_index = i;

            for j in (i + 1)..slice.len() {
                if compare(slice[j], slice[minimum_index]) {
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
    fn sort(&self, slice: &mut [i32], compare: &Box<Fn(i32, i32) -> bool>) {
        for i in 1..slice.len() {
            let key = slice[i];

            let mut j = i;

            while j > 0 && compare(key, slice[j - 1]) {
                slice[j] = slice[j - 1];

                j -= 1;
            }

            slice[j] = key;
        }
    }
}

fn partition_slice(slice: &mut [i32], compare: &Box<Fn(i32, i32) -> bool>) -> usize {
    let len = slice.len();

    let pivot_index = len / 2;

    slice.swap(pivot_index, len - 1);

    let mut store_index = 0;

    for i in 0..len - 1 {
        if compare(slice[i], slice[len - 1]) {
            slice.swap(i, store_index);

            store_index += 1;
        }
    }

    slice.swap(store_index, len - 1);

    store_index
}

impl SortSlice for QuickSorter {
    fn sort(&self, slice: &mut [i32], compare: &Box<Fn(i32, i32) -> bool>) {
        let len = slice.len();

        if len >= 2 {
            let pivot_index = partition_slice(slice, &compare);

            self.sort(&mut slice[0..pivot_index], &compare);

            self.sort(&mut slice[pivot_index + 1..len], &compare);
        }
    }
}

#[cfg(test)]
mod tests {

    extern crate rand;

    use super::*;

    use self::rand::Rng;

    #[test]
    fn selection_sort_test() {
        let mut test_vec = vec![3, 1, 2, 6, 5, 4];

        let expected_vec_greater = vec![1, 2, 3, 4, 5, 6];

        let expected_vec_less = vec![6, 5, 4, 3, 2, 1];

        let compare_less = Box::new(|x, y| x > y) as Box<Fn(i32, i32) -> bool>;

        let compare_greater = Box::new(|x, y| x < y) as Box<Fn(i32, i32) -> bool>;

        let sorter = SelectionSorter;

        sorter.sort(&mut test_vec, &compare_less);

        assert_eq!(&test_vec, &expected_vec_less);

        sorter.sort(&mut test_vec, &compare_greater);

        assert_eq!(&test_vec, &expected_vec_greater);
    }

    #[test]
    fn general_test() {
        let test_vec = (0..10)
            .map(|_| rand::thread_rng().gen_range(-1000, 1001))
            .collect::<Vec<i32>>();

        let sorters: Vec<Box<SortSlice>> = vec![
            Box::new(BubbleSorter),
            Box::new(SelectionSorter),
            Box::new(QuickSorter),
            Box::new(InsertSorter),
        ];

        let compare: Box<Fn(i32, i32) -> bool> = Box::new(|x, y| x < y);

        for sorter in &sorters {
            let mut test_vec_for_sorter = test_vec.clone();

            let mut test_vec_for_std = test_vec.clone();

            sorter.sort(&mut test_vec_for_sorter, &compare);

            test_vec_for_std.sort();

            assert_eq!(test_vec_for_sorter, test_vec_for_std);
        }
    }

}
