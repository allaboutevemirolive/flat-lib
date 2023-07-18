
// https://gist.github.com/jonhoo/2a7fdcf79be03e51a5f95cd326f2a1e8
// https://www.youtube.com/watch?v=rAl-9HwD858&t=1660s


#![warn(rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        // println!("123");
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            // 1. `let until_delimiter = &remainder[..delim_start];`: This line creates a new variable
            // `until_delimiter`, which is a reference to a slice of the `remainder` string. The slice
            // starts from the beginning of the `remainder` and extends up to, but not including, the
            // character at index `delim_start`. It effectively represents the substring from the
            // current position up to the delimiter.

            // Note that the type of `until_delimiter` is `&str`, which means it's an immutable borrow
            // of the `remainder` string.
            let until_delimiter = &remainder[..delim_start];
            // TODO: unused line?

            // 2. `*remainder = &remainder[delim_end..];`: This line updates the `remainder` of the 
            // `StrSplit` struct to point to a new slice of the `remainder` string. The slice starts 
            // from the character at index `delim_end` and extends to the end of the `remainder`. 
            
            // This has the effect of removing the substring until the delimiter from the `remainder`, 
            // leaving only the part of the string after the delimiter for the next iteration.

            // The `*remainder` on the left side of the expression dereferences the `Option<&str>` 
            // contained in `remainder`, allowing us to update the value it points to.

            // After this line executes, the `remainder` now points to the part of the original 
            // string that comes after the delimiter, effectively advancing the iterator to the next 
            // part of the string.
            
            // NOTE: delim_end is exclusive
            *remainder = &remainder[delim_end..];
            println!("{}", remainder);
            
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            // The `find` method is applied to the iterator returned by `s.char_indices()`.
            // It searches for the first occurrence of a character that satisfies the given closure condition.
            // In this case, the closure takes a tuple of `(usize, char)`, but only the `char` (`c`)
            // is used in the condition.
            // The `self` in the closure refers to the character passed to the `find_next` function.
            // The closure checks if the current character `c` is equal to the given character `self`.
            // When it finds the first character that matches the condition,
            // it returns `Some((byte_offset, character))`,
            // where `byte_offset` is the byte offset of the character in the original string `s`,
            // and `character` is the character itself. If no match is found, it returns `None`.
            .find(|(_, c)| c == self)

            // (start, start + self.len_utf8())

            // 1. `start`: This is a variable or expression representing the starting byte
            // offset of a character in the original string. The value of `start` is an `usize`,
            // which indicates the byte offset where the character begins in the string.

            // 2. `self.len_utf8()`: This is a method call on the variable `self`, where `self`
            // is a character (type `char`). The method `len_utf8()` returns the number of bytes
            // the character occupies in its UTF-8 representation.

            // 3. `start + self.len_utf8()`: This performs addition between `start` and the result of
            // `self.len_utf8()`. The sum represents the ending byte offset of the character in the
            // original string. Since `start` represents the starting byte offset, adding the number
            // of bytes the character occupies gives the byte offset immediately after the character,
            // effectively representing the range of bytes occupied by the character in the string.
            // ===================================================================================
            // 3. `.map(|(start, _)| (start, start + self.len_utf8()))`: The `map` method is applied
            // to the result of the `find` method. It transforms the `Option` containing the matched
            // character's information into another `Option` containing a tuple `(usize, usize)`
            // representing the byte offset range of the matched character.

            // The closure takes the tuple `(usize, char)` (the result of `find`) as input, but since
            // we are only interested in the `usize` representing the byte offset (`start`), the `_`
            // is used to ignore the character (`char`). The `self` in this closure still refers to
            // the character passed to the `find_next` function.

            // The closure then computes the end byte offset of the matched character by adding the
            // byte length of the character (in UTF-8 representation) to the `start` byte offset.
            // This is done using the `self.len_utf8()` method, which returns the number of bytes the
            // character occupies in UTF-8 encoding.

            // The final result of the `map` is an `Option` containing a tuple `(usize, usize)`, where
            // the first element is the `start` byte offset of the matched character, and the second
            // element is the `end` byte offset, representing the range in the original string where
            // the matched character is located.

            // The overall purpose of this line is to find the first occurrence of the given character
            // `self` in the string `s`, return its byte offset range, and eventually use this
            // information to split the string at the matched character. This is how the `StrSplit`
            // struct works to split the string using a delimiter.
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    let input_str = "hello world";
    let delimiter = 'o';

    assert_eq!(until_char(input_str, delimiter), "hell");

    // let mut splitter = StrSplit::new(input_str, delimiter);

    // // Print the remainder inside the test
    // if let Some(remainder) = splitter.next() {
    //     println!("Remainder: {}", remainder);
    // } else {
    //     println!("No remainder.");
    // }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

fn main() {}
