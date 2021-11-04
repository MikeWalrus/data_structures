use std::io::{self, BufRead, Write};

fn main() {
    let stdio = io::stdin();
    let lock = stdio.lock();
    let mut lines = lock.lines();
    print!("text   > ");
    io::stdout().flush().unwrap();
    let text = lines.next().unwrap().unwrap();
    print!("pattern> ");
    io::stdout().flush().unwrap();
    let pattern = lines.next().unwrap().unwrap();
    let find_match =
        |f: fn(&[u8], &[u8]) -> Option<usize>| match f(text.as_bytes(), pattern.as_bytes()) {
            Some(result) => println!("{}", result),
            None => println!("No match."),
        };
    println!("Brute-force:");
    find_match(matches_bf);
    println!("KMP:");
    find_match(matches_kmp);
}

fn matches_bf(text: &[u8], pattern: &[u8]) -> Option<usize> {
    let target_len = text.len();
    let pattern_len = pattern.len();
    if target_len < pattern_len {
        return None;
    }
    for i in 0..=(text.len() - pattern.len()) {
        let mut j = 0usize;
        loop {
            if text[i + j] != pattern[j] {
                break;
            }
            j += 1;
            if j == pattern_len {
                return Some(i);
            }
        }
    }
    None
}

fn matches_kmp(text: &[u8], pattern: &[u8]) -> Option<usize> {
    // KMP 1977
    // place pattern at left;
    // while pattern not fully matched
    //     and text not exhausted do
    // begin
    //     while pattern character differs from
    //         current text character
    //         do shift pattern appropriately;
    //     advance to next character of text;
    // end;

    let next = generate_next_table(pattern);
    let text_len = text.len();
    let pattern_len = pattern.len();

    // place pattern at left
    let mut k = 0; // used to access text
    let mut j = 0; // used to access pattern
    while k < text_len // while pattern not fully matched
          && j < pattern_len
    // and text not exhausted
    {
        while j < usize::MAX && text[k] != pattern[j] {
            // while pattern character differs from current character
            j = next[j]; // shift pattern appropriately, usize::MAX means we have to start over
        }
        // advance to next character of text
        k += 1;
        j = j.wrapping_add(1); // wrapping_add so that usize::MAX acts as a -1
    }
    if j >= pattern_len {
        Some(k - pattern_len)
    } else {
        None
    }
}

fn generate_next_table(pattern: &[u8]) -> Vec<usize> {
    let mut next: Vec<usize> = Vec::with_capacity(pattern.len());
    next.push(usize::MAX); //next[0] = usize::MAX;
    let mut j = 0; // used to access pattern
    let mut t = next[0]; // t = f[i] as in their paper
    let m = pattern.len();

    while j < m - 1 {
        while t < usize::MAX && pattern[j] != pattern[t] {
            t = next[t];
        }
        j += 1;
        t = t.wrapping_add(1);
        next.push(if pattern[j] != pattern[t] { t } else { next[t] }) // next[j]
    }
    next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bf() {
        test_matching(matches_bf);
    }

    #[test]
    fn test_kmp() {
        test_matching(matches_kmp)
    }

    fn test_matching(f: fn(&[u8], &[u8]) -> Option<usize>) {
        assert_eq!(f(&Vec::from("abcd"), &Vec::from("cd")), Some(2));
        assert_eq!(f(&Vec::from("abcd"), &Vec::from("abcd")), Some(0));
        assert_eq!(
            f(&Vec::from("spam, egg and spam"), &Vec::from("egg")),
            Some(6)
        );
        assert_eq!(
            f(&Vec::from("spam, egg and spam"), &Vec::from("bacon")),
            None
        );
        assert_eq!(
            f(&Vec::from("My name is 小明."), &Vec::from("小明")),
            Some(11)
        );
    }

    #[test]
    fn test_next() {
        assert_eq!(
            generate_next_table(&Vec::from("abcabcacab")),
            vec![
                usize::MAX,
                0,
                0,
                usize::MAX,
                0,
                0,
                usize::MAX,
                4,
                usize::MAX,
                0
            ]
        )
    }
}
