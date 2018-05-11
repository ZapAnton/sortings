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
                if compare(slice[i], slice[j]) {
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
