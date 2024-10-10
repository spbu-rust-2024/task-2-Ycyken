use std::cmp::{max, min};

pub fn longest_palindromic_substring(s: &String) -> &str {
    if s.is_empty() {
        return &s
    }

    let manacher = manacher(s);
    let mut max_len: i64 = 0;
    let mut max_palindrome_index: i64 = 0;
    for (i, &len) in manacher.iter().enumerate() {
        if len > max_len {
            max_len = len;
            max_palindrome_index = i as i64;
        }
    }
    let l = (max_palindrome_index - (max_len - 1) / 2) as usize;
    let r = (max_palindrome_index + max_len / 2) as usize;
    &s[l..=r]
}

fn manacher(input: &String) -> Vec<i64> {
    let man_odd = manacher_odd(input);
    let man_even = manacher_even(input);
    let mut longest_palindrome: Vec<i64> = vec![0; man_odd.len()];
    for i in 0..longest_palindrome.len() {
        longest_palindrome[i] = max(man_even[i] * 2, man_odd[i] * 2 + 1);
    }

    longest_palindrome
}

fn manacher_odd(input: &String) -> Vec<i64> {
    let input_chars: Vec<char> = input.chars().collect();
    let len = input_chars.len();
    let mut odd_radii: Vec<i64> = vec![0; len];

    let (mut l, mut r) = (0i64, 0i64);
    for i in 0..len{
        let i_i64 = i as i64;
        if i_i64 < r {
            odd_radii[i] = min(r - i_i64, odd_radii[(l + r - i_i64) as usize]);
        }

        while i_i64 - 1 - odd_radii[i] >= 0
            && i_i64 + 1 + odd_radii[i] < len as i64
            && input_chars[(i_i64 - 1 - odd_radii[i]) as usize] == input_chars[(i_i64 + 1 + odd_radii[i]) as usize]
        {
            odd_radii[i] += 1;
        }
        if i_i64 + odd_radii[i] > r {
            r = i_i64 + odd_radii[i];
            l = i_i64 - odd_radii[i];
        }
    }
    odd_radii
}

fn manacher_even(input: &String) -> Vec<i64> {
    let input_chars: Vec<char> = input.chars().collect();
    let len = input_chars.len();
    let mut even_radii: Vec<i64> = vec![0; len];

    let (mut l, mut r) = (0i64, 0i64);
    for i in 0..len {
        let i_i64 = i as i64;
        if i_i64 < r {
            even_radii[i] = min(r - i_i64, even_radii[(l + r - i_i64 - 1) as usize]);
        }

        while i as i64 - even_radii[i] >= 0
            && i_i64 + even_radii[i] + 1 < len as i64
            && input_chars[(i_i64 - even_radii[i]) as usize] == input_chars[(i_i64 + 1 + even_radii[i]) as usize]
        {
            even_radii[i] += 1;
        }
        if i_i64 + even_radii[i] > r {
            r = i_i64 + even_radii[i];
            l = i_i64 + 1 - even_radii[i];
        }
    }
    even_radii
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindromic_basic() {
        let s = String::from("eeeabcdcbae01");
        let result = longest_palindromic_substring(&s);
        assert_eq!(result, "eabcdcbae");
    }

    #[test]
    fn longest_palindromic_odd()  {
        let s = String::from("hghaabcdcb4");
        let result = longest_palindromic_substring(&s);
        assert_eq!(result, "bcdcb");
    }

    #[test]
    fn empty_string(){
        let s  = String::from("");
        let result = longest_palindromic_substring(&s);
        assert_eq!(result, "")
    }
}