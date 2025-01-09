pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let packed_lines = Self::pack_lines(words, max_width);
        let len = packed_lines.len();

        Self::justify_lines(packed_lines, len - 1, max_width)
    }

    pub fn pack_lines(words: Vec<String>, max_width: i32) -> Vec<Vec<String>> {
        let mut lines: Vec<Vec<String>> = Vec::new();

        let mut line: Vec<String> = Vec::new();
        for i in 0..words.len() {
            if !Self::can_fit_word(max_width, &words[i], &line.as_slice()) {
                lines.push(line.clone());
                line.clear();
            }
            line.push(words[i].clone())
        }

        if !line.is_empty() {
            lines.push(line);
        }

        lines
    }

    pub fn justify_lines(
        lines: Vec<Vec<String>>,
        lines_to_justify: usize,
        max_width: i32,
    ) -> Vec<String> {
        lines.iter().enumerate().fold(
            Vec::<String>::with_capacity(lines.len()),
            |mut acc, (i, line)| {
                if i < lines_to_justify {
                    let total_chars = line.iter().map(|s| s.len()).sum::<usize>() as i32;
                    let padding_to_distribute = max_width - total_chars;
                    let gaps = line.len() as i32 - 1;
                    let (spaces_per_gap, extra_spaces) = if gaps > 0 {
                        (padding_to_distribute / gaps, padding_to_distribute % gaps)
                    } else {
                        (0, padding_to_distribute)
                    };

                    let mut justified = String::new();
                    for (j, word) in line.iter().enumerate() {
                        justified.push_str(word);
                        if j < line.len() - 1 {
                            let spaces =
                                spaces_per_gap + if j < extra_spaces as usize { 1 } else { 0 };
                            justified.push_str(&" ".repeat(spaces as usize));
                        } else if gaps == 0 {
                            justified.push_str(&" ".repeat(extra_spaces as usize));
                        }
                    }
                    acc.push(justified);
                } else {
                    let mut last_line = line.join(" ");
                    last_line.push_str(&" ".repeat((max_width - last_line.len() as i32) as usize));
                    acc.push(last_line);
                }

                acc
            },
        )
    }

    fn length_of_unpadded_line(line: &[String]) -> i32 {
        // The real length of an unpadded line accounts for a single space between
        // words.
        if line.len() <= 1 {
            // Single-word line has no inter-word spaces
            line.iter().fold(0i32, |acc, w| acc + w.len() as i32)
        } else {
            // Multiple words: add spaces between words
            line.iter().fold(0i32, |acc, w| acc + w.len() as i32) + (line.len() - 1) as i32
        }
    }

    fn can_fit_word(max_width: i32, word: &String, line: &[String]) -> bool {
        if line.is_empty() {
            word.len() as i32 <= max_width
        } else {
            max_width >= Self::length_of_unpadded_line(line) + word.len() as i32 + 1
        }
    }

    pub fn unjustify(justified_lines: Vec<String>) -> Vec<String> {
        justified_lines
            .iter()
            .flat_map(|line| {
                // Split by any number of spaces and filter out empty strings
                line.split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // Given
        let words = vec![
            "What".to_string(),
            "must".to_string(),
            "be".to_string(),
            "acknowledgment".to_string(),
            "shall".to_string(),
            "be".to_string(),
        ];

        // When
        let result = Solution::full_justify(words, 16);

        // Then
        assert_eq!(
            result,
            vec![
                "What   must   be".to_string(),
                "acknowledgment  ".to_string(),
                "shall be        ".to_string(),
            ]
        );
    }

    #[test]
    fn test_unjustify() {
        // Given
        let justified = vec![
            "What   must   be".to_string(),
            "acknowledgment  ".to_string(),
            "shall be        ".to_string(),
        ];

        // When
        let result = Solution::unjustify(justified);

        // Then
        assert_eq!(
            result,
            vec!["What", "must", "be", "acknowledgment", "shall", "be"]
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>()
        );
    }
}
