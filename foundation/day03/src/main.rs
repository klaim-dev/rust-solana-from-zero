use crate::text_utils::summarize;

mod domain;
mod text_utils;

fn main() {
    let (chars, words, empty) = summarize("Hello World");
    println!("chars: {}, words: {}, empty: {}", chars, words, empty);
}
