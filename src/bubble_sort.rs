#[cfg(test)]
mod tests {
    use crate::algos::sort::bubble_sort::sort;

    #[test]
    fn exploration() {
        assert_eq!(
            true,
            ([1, 2, 3, 4, 5].to_vec() == sort([2, 3, 5, 1, 4].to_vec()))
        );
    }
}

fn sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                // let tmp = array[j];
                // array[j] = array[j + 1];
                // array[j + 1] = tmp;
                array.swap(j, j + 1);
            }
        }
    }
    return array;
}
