use compare::*;
use std::cmp::Ordering::*;

pub fn chop<T: std::cmp::Ord>(value: &T, data: &[T]) -> Option<usize> {
    let mut s = 0;
    let mut e = data.len();
    loop {
        if e <= s {
            break None;
        }
        let pos = s + (e - 1 - s) / 2;
        match natural().compare(&data[pos], &value) {
            Equal => break Some(pos),
            Less => s = pos + 1,
            Greater => e = pos,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn bin_search() {
        use crate::kata::kata1::chop;

        assert_eq!(None, chop(&3, &vec![]));
        assert_eq!(None, chop(&3, &vec![1]));
        assert_eq!(Some(0), chop(&1, &vec![1]));
        //
        assert_eq!(Some(0), chop(&1, &vec![1, 3, 5]));
        assert_eq!(Some(1), chop(&3, &vec![1, 3, 5]));
        assert_eq!(Some(2), chop(&5, &vec![1, 3, 5]));
        assert_eq!(None, chop(&0, &vec![1, 3, 5]));
        assert_eq!(None, chop(&2, &vec![1, 3, 5]));
        assert_eq!(None, chop(&4, &vec![1, 3, 5]));
        assert_eq!(None, chop(&6, &vec![1, 3, 5]));
    }
}
