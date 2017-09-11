mod two_sum {
    use std::collections::HashMap;
    // 给与一个元素为int类型的数组和一个目标值
    // 返回两个值的索引, 使得这两个值相加等于这个目标值
    // 
    // 这里返回值的元素类型应该是 `usize`, 因为enumerate返回的index类型为usize
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
        let mut dict: HashMap<i32, usize> = HashMap::new();
        for (i, value) in nums.iter().enumerate() {
            if dict.contains_key(&(target - value)) {
                return Some((*dict.get(&(target - value)).unwrap(), i))
            } else {
                dict.insert(*value, i);
            }
        }
        return None
    }

    // 这里返回值的元素类型应该是 `usize`, 因为enumerate返回的index类型为usize
    pub fn new_two_sum(nums: Vec<i32>, target: i32) -> Option<[usize; 2]> {
        let mut dict: HashMap<i32, usize> = HashMap::new();
        for (i, value) in nums.iter().enumerate() {
            if dict.contains_key(&(target - value)) {
                // return Some([dict[&(target - value)], i)
                // HashMap::get接受的参数是key的引用, 返回的是对应值的引用
                return Some([*dict.get(&(target - value)).unwrap(), i])
            }
            else {
                dict.insert(*value, i);
            }
        }
        return None
    }
}

#[cfg(test)]
mod test {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum::two_sum(vec![2, 7, 11, 15], 18), Some((1, 2)));
        assert_eq!(two_sum::two_sum(vec![3, 2, 4], 6), Some((1, 2)));

        assert_eq!(two_sum::two_sum(vec![3, 2, 4], 7), Some((0, 2)));

        assert_eq!(two_sum::two_sum(vec![3, 2, 4], 8), None);

        assert_eq!(two_sum::two_sum(vec![3, 2, 11, 3], 5), Some((0, 1)));

        assert_eq!(two_sum::two_sum(vec![3], 5), None);
    }
}
