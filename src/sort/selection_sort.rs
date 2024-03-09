#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn testSort() {
        assert_eq!(
            true,
            ([1, 2, 3, 4, 5].to_vec() == sort([2, 3, 5, 1, 4].to_vec()))
        );
    }
}
fn sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 0..array.len() - 1 {
        let mut max = 0;
        for j in 0..array.len() - 2 - i {
            if (array[j] > array[j + 1]) {
                max = j;
            } else {
                max = j + 1;
            }
        }
        if (max != array.len() - 1 - i) {
            let lastIndex = array.len() - 1 - i;
            array.swap(max, lastIndex);
        }
    }
    return array;
}
