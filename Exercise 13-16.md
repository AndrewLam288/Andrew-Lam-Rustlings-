# Do Rustlings Sections 13 to 16

## Exercise 1 lifetimes.rs

## Code Snippeet

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}

## Code Explanation

The code will define a function longest where it would take two string slices, returning the longest out of the two string. Comparing by doing an if statement where if x length > y length then return the string x, else return y. And the following is tests cases to test if it accurate picking out the longest.

## My own write up

This however is causing compiling error, as str in the function longest is expecting named lifetime parameter:
The error:
fn longest(x: &str, y: &str) -> &str
  |               ----     ----    ^ expected named lifetime parameter

The function longest here is taking two string slices x and y, so a lifetime parameter 'a would ensures that the returned reference is valid as long as both input references are valid.
I fix this compile error by adding 'a (a lifetime parameter) to each string slice, below is the new solution code snippet:

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
