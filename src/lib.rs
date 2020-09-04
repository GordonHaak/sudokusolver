#[cfg(test)]
mod tests {

    fn chop(value: i32, data: &Vec<i32>) -> Option<usize> {
        let mut s = 0;
        let mut e = data.len();
        loop {
            if e <= s {
                break None
            }
            let pos = s + (e-1-s)/2;
            let v = data.get(pos).expect("not in range");
            use compare::*;
            use std::cmp::Ordering::*;
            match compare::natural().compare(v, &value) {
                Equal => break Some(pos),
                Less => s = pos + 1, 
                Greater => e = pos
            }
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(None, chop(3, &vec![]));
        assert_eq!(None, chop(3, &vec![1]));
        assert_eq!(Some(0), chop(1, &vec![1]));
        //
        assert_eq!(0, chop(1, &vec![1, 3, 5]));
        assert_eq!(1, chop(3, &vec![1, 3, 5]));
        assert_eq!(2, chop(5, &vec![1, 3, 5]));
        assert_eq!(-1, chop(0, &vec![1, 3, 5]));
        assert_eq!(-1, chop(2, &vec![1, 3, 5]));
        assert_eq!(-1, chop(4, &vec![1, 3, 5]));
        assert_eq!(-1, chop(6, &vec![1, 3, 5]));
    }
}
