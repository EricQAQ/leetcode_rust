mod add_two_numbers {
    // 给与两个非空链表, 代表两个非负整数, 数位是倒排的.
    // 以链表的形式输出相加的结果值
    use std::cmp::max;
    
    pub fn add_two_numbers(nums_1: &[usize], nums_2: &[usize]) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let length = max(nums_1.len(), nums_2.len());
        let mut flag = 0;
        for index in 0..length {
            // 这里的array实现了slice, 有get方法, 它接受的是value而不是借用,
            // 返回的是Option, 函数原型如下：
            // fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output> where I: SliceIndex<[T]>
            let x = *nums_1.get(index).unwrap_or(&0);
            let y = *nums_2.get(index).unwrap_or(&0);
            let digit_sum = x + y;
            if digit_sum >= 10 {
                result.push(digit_sum - 10 + flag);
                flag = 1;
            } else {
                result.push(digit_sum + flag);
                flag = 0;
            }
        }
        if flag == 1 {
            result.push(1);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::add_two_numbers;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers::add_two_numbers(&[1,2,3], &[1,2,3]), vec![2, 4, 6]);
        assert_eq!(add_two_numbers::add_two_numbers(&[9], &[9]), vec![8, 1]);
        assert_eq!(add_two_numbers::add_two_numbers(&[2, 2], &[5, 6, 4]), vec![7, 8, 4]);
    }
}
