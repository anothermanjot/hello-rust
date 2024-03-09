#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn exploration() {
        assert_eq!(
            true,
            ([1, 2, 3, 4, 5].to_vec() == sort([2, 3, 5, 1, 4].to_vec()))
        );
    }
}

fn sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 0..array.len() - 1 {
        let mut j: usize = i;
        loop {
            if array[j + 1] < array[j] {
                array.swap(j, j + 1);
            } else {
                break;
            }
            if j == 0 {
                break;
            }
            j = j - 1;
        }
        println!("{:?}", array);
    }

    return array;
}
