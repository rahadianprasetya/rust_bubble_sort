use std::fmt::Debug;
mod b_rand;

// O(n^2) BIG O is n square
pub fn bubble_short<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half
    // sort the rigth half O(n * ln (n) ) 1024 * 10 < 1024 * 1024
    // bring the sorted half together O(n)
    println!("{:?}", v);
    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // bring them together which ever is lowest the front of a or the front of b
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

// Move first element to the correct place
// Everything lower should be before it,
// Everything higher should be after if,
// return it's location

pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;

    for i in 1..v.len() {
        if v[i] < v[p] {
            // move our pivot forward + 1, and puts this element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
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

    #[test]
    fn test_bubble_short_merge() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);
        println!("{:?}", v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut v);
        println!("{:?}", p);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }

        //assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        println!("{:?}", v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
        panic!();
    }
}
