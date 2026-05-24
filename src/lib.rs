use std::fmt::Debug;

// O(n^2) BIG O is n square
pub fn bubble_short<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        println!("{:?}", v);
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_short() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_short(&mut v);
        println!("{:?}", v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
