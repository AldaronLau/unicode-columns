use unicode_width::UnicodeWidthChar;

/// Truncate a string to a specific column width
pub fn truncate(string: &str, width: usize) -> &str {
    let mut cw = 0;
    let mut iter = string.char_indices();
    while let Some((i, c)) = iter.next() {
        if u32::from(c) == /* ZWJ */ 0x200d {
            iter.next();
            continue;
        }
        let nw = cw + c.width().unwrap_or(0);
        if nw > width {
            return &string[..i];
        }
        cw = nw;
    }
    string
}

#[cfg(test)]
mod tests {
    use crate::truncate;

    #[test]
    fn test_truncation() {
        // basic tests
        assert_eq!(truncate("teststring", 50), "teststring");
        assert_eq!(truncate("teststring", 5), "tests");
        assert_eq!(truncate("teststring", 0), "");
        // full-width (2 column) characters test
        assert_eq!(truncate("잘라야", 4), "잘라");
        // combining characters (zalgo text) test
        assert_eq!(truncate("ę̵̡̛̮̹̼̝̲͓̳̣͉̞͔̳̥̝͍̩̣̹͙̘̼̥̗̼͈̯͎̮̥̤̪̻̮͕̩̮͓͔̟͈͇͎̣͉͇̦͔̝̣͎͎͔͇̭͈̌̂̈̄̈́̾͑̀̈̓̂͗̾̉͊͒̆̽͊̽͘̕͜͜͝͠ :trunc", 3), "ę̵̡̛̮̹̼̝̲͓̳̣͉̞͔̳̥̝͍̩̣̹͙̘̼̥̗̼͈̯͎̮̥̤̪̻̮͕̩̮͓͔̟͈͇͎̣͉͇̦͔̝̣͎͎͔͇̭͈̌̂̈̄̈́̾͑̀̈̓̂͗̾̉͊͒̆̽͊̽͘̕͜͜͝͠ :");
        // zero-width-joiner (emoji) test
        assert_eq!(truncate("👨‍👩‍👦:trunc", 3), "👨‍👩‍👦:");
    }
}
