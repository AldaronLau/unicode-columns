use unicode_width::UnicodeWidthChar;

const ZWJ: u32 = 0x200d;

/// Get the column width of a string
pub fn width(string: &str) -> usize {
    let mut cw = 0;
    let mut iter = string.chars();
    while let Some(c) = iter.next() {
        if u32::from(c) == ZWJ {
            iter.next();
            continue;
        }
        cw += c.width().unwrap_or(0);
    }
    cw
}

/// Truncate a string to a specific column width
pub fn truncate(string: &str, width: usize) -> &str {
    let mut cw = 0;
    let mut iter = string.char_indices();
    while let Some((i, c)) = iter.next() {
        if u32::from(c) == ZWJ {
            iter.next();
            continue;
        }
        cw += c.width().unwrap_or(0);
        if cw > width {
            return &string[..i];
        }
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width() {
        // basic tests
        assert_eq!(width("teststring"), 10);
        // full-width (2 column) characters test
        assert_eq!(width("잘라야"), 6);
        // combining characters (zalgo text) test
        assert_eq!(width("ę̵̡̛̮̹̼̝̲͓̳̣͉̞͔̳̥̝͍̩̣̹͙̘̼̥̗̼͈̯͎̮̥̤̪̻̮͕̩̮͓͔̟͈͇͎̣͉͇̦͔̝̣͎͎͔͇̭͈̌̂̈̄̈́̾͑̀̈̓̂͗̾̉͊͒̆̽͊̽͘̕͜͜͝͠ :width"), 8);
        // zero-width-joiner (emoji) test
        assert_eq!(width("👨‍👩‍👦:width"), 8);
    }

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
