pub struct Solution;

/**
 * 88. Merge Sorted Array
 *
 * You are given two integer arrays nums1 and nums2, sorted in non-decreasing order,
 * and two integers m and n, representing the number of elements in nums1 and nums2
 * respectively.
 *
 * Merge nums1 and nums2 into a single array sorted in non-decreasing order.
 *
 * The final sorted array should not be returned by the function, but instead be
 * stored inside the array nums1. To accommodate this, nums1 has a length of m + n,
 * where the first m elements denote the elements that should be merged, and the last n
 * elements are set to 0 and should be ignored. nums2 has a length of n.
 */

impl Solution {
    pub fn solve_v0(
        numbers: &mut Vec<i32>,
        numbers_size: i32,
        numbers_to_merge: &mut Vec<i32>,
        numbers_to_merge_size: i32,
    ) {
        // Cursor moves through numbers, which we're told is pre-sorted.
        let mut cursor: usize = 0;
        let mut filled_count: usize = numbers_size as usize;

        // Take each number from `numbers_to_merge`
        for to_merge in numbers_to_merge {
            // Find the cursor position where the next number is bigger than to_merge or the end of the list.
            while cursor < filled_count && numbers[cursor] < *to_merge {
                cursor += 1;
            }

            // Insert the number at the cursor position.
            numbers.insert(cursor.into(), *to_merge);
            cursor += 1;
            filled_count += 1;
        }

        // Remove the extra elements from the end of the list.
        numbers.truncate(numbers_size as usize + numbers_to_merge_size as usize);
    }

    pub fn solve(
        numbers: &mut Vec<i32>,
        numbers_size: i32,
        numbers_to_merge: &mut Vec<i32>,
        numbers_to_merge_size: i32,
    ) {
        let mut write_idx = (numbers_size + numbers_to_merge_size - 1) as usize;
        let mut i = numbers_size - 1;
        let mut j = numbers_to_merge_size - 1;

        // Work backwards, placing larger elements at the end
        while i >= 0 && j >= 0 {
            if numbers[i as usize] > numbers_to_merge[j as usize] {
                numbers[write_idx] = numbers[i as usize];
                i -= 1;
            } else {
                numbers[write_idx] = numbers_to_merge[j as usize];
                j -= 1;
            }
            write_idx = write_idx.wrapping_sub(1);
        }

        // Copy any remaining elements from numbers_to_merge
        while j >= 0 {
            numbers[write_idx] = numbers_to_merge[j as usize];
            j -= 1;
            write_idx = write_idx.wrapping_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // Given
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        // When
        Solution::solve(&mut nums1, m, &mut nums2, n);

        // Then
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
        assert_eq!(nums2, vec![2, 5, 6]);
    }

    #[test]
    fn test_solution_2() {
        // Given
        let mut nums1 = vec![1, 4, 5, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        // When
        Solution::solve(&mut nums1, m, &mut nums2, n);

        // Then
        assert_eq!(nums1, vec![1, 2, 4, 5, 5, 6]);
        assert_eq!(nums2, vec![2, 5, 6]);
    }
}
