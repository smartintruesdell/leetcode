pub struct Solution;

/**
 * 238. Product of Array Except Self
 */
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left_product: Vec<i32> = vec![1; nums.len()];
        let mut result: Vec<i32> = vec![1; nums.len()];

        // fill with left product
        Self::left_product(&nums, &mut left_product);

        // compute right products and combine with left
        let mut right_running_product = 1;
        for j in (0..nums.len()).rev() {
            result[j] = left_product[j] * right_running_product;
            right_running_product = right_running_product * nums[j];
        }

        result
    }

    fn left_product(nums: &Vec<i32>, target: &mut Vec<i32>) {
        for i in 0..nums.len() {
            target[i] = if i > 0 {
                target[i - 1] * nums[i - 1]
            } else {
                1
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // Given
        let values = vec![1, 2, 3, 4];

        // When
        let result = Solution::product_except_self(values);

        // Then
        assert_eq!(result, vec![24, 12, 8, 6]);
    }
}
