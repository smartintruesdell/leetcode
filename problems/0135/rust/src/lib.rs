pub struct Solution;

/**
 * 135. Candy
 *
 * There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.
 *
 * You are giving candies to these children subjected to the following requirements:
 *
 * - Each child must have at least one candy.
 * - Children with a higher rating get more candies than their neighbors.
 *
 * Return the minimum number of candies you need to have to distribute the candies to the children.
 */

impl Solution {
    pub fn solve(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies: Vec<i32> = vec![1; n];

        // Forward pass: compare with left neighbor
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        // Backward pass: compare with right neighbor
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // Given
        let ratings = vec![1, 2, 87, 87, 87, 2, 1];

        // When
        let result = Solution::solve(ratings);

        // Then
        assert_eq!(result, 13);
    }
}
