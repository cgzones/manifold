use std::cmp::min;
use std::sync::Arc;

use crate::highlighter::Highlight;

const FOUR_KB: usize = 4 * 1024; // 4 KiB

/// Applies a given function to the unhighlighted parts of an input string, preserving any existing highlighting.
pub fn apply_only_to_unhighlighted(input: &str, highlighter: &Arc<dyn Highlight>) -> String {
    let chunks = split_into_chunks(input);
    let mut result = allocate_string(input);

    for chunk in chunks {
        match chunk {
            Chunk::NotHighlighted(text) => {
                result.push_str(&highlighter.apply(text));
            }
            Chunk::AlreadyHighlighted(text) => {
                result.push_str(text);
            }
        }
    }

    result
}

fn allocate_string(input: &str) -> String {
    let input_length_times_3 = input.len().saturating_mul(3);
    let allocation_size = min(input_length_times_3, FOUR_KB);

    String::with_capacity(allocation_size)
}

enum Chunk<'a> {
    NotHighlighted(&'a str),
    AlreadyHighlighted(&'a str),
}

fn split_into_chunks(input: &str) -> Vec<Chunk> {
    const RESET_CODE: &str = "\x1b[0m";
    const ESCAPE_CODE: &str = "\x1b[";

    let mut chunks = Vec::new();
    let mut start = 0;
    let mut inside_escape = false;

    while let Some(i) = if inside_escape {
        input[start..].find(RESET_CODE)
    } else {
        input[start..].find(ESCAPE_CODE)
    } {
        let i = i + start;
        if inside_escape {
            chunks.push(Chunk::AlreadyHighlighted(&input[start..(i + RESET_CODE.len())]));
            start = i + RESET_CODE.len();
        } else {
            if i != start {
                chunks.push(Chunk::NotHighlighted(&input[start..i]));
            }
            start = i;
        }
        inside_escape = !inside_escape;
    }

    if start != input.len() {
        chunks.push(if inside_escape {
            Chunk::AlreadyHighlighted(&input[start..])
        } else {
            Chunk::NotHighlighted(&input[start..])
        });
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_chunks() {
        let input = "Here is a date \x1b[31m2023-06-24\x1b[0m, and here is a number 12345.";
        let chunks = split_into_chunks(input);

        assert_eq!(chunks.len(), 3);
        match &chunks[0] {
            Chunk::NotHighlighted(text) => assert_eq!(*text, "Here is a date "),
            Chunk::AlreadyHighlighted(_) => panic!("Unexpected chunk type."),
        }
        match &chunks[1] {
            Chunk::AlreadyHighlighted(text) => assert_eq!(*text, "\x1b[31m2023-06-24\x1b[0m"),
            Chunk::NotHighlighted(_) => panic!("Unexpected chunk type."),
        }
        match &chunks[2] {
            Chunk::NotHighlighted(text) => assert_eq!(*text, ", and here is a number 12345."),
            Chunk::AlreadyHighlighted(_) => panic!("Unexpected chunk type."),
        }
    }
}
