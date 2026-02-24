use num::integer::gcd;
// For two strings s and t, we say "t divides s" if and only if s = t + t + t + ... + t + t 
// (i.e., t is concatenated with itself one or more times).

// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.

pub fn gcd_of_strings<'a>(s: &'a str, t: &'a str) -> &'a str {
    let l_s = s.len();
    let l_t = t.len();

    if format!("{}{}", s, t) != format!("{}{}", t, s) {
        return "";
    }

    let d = gcd(l_s, l_t);

    &s[0..d]
}

#[cfg(test)]
mod tests{
    use super::gcd_of_strings;

    #[test]
    fn test_gcd_of_strings(){
        let s = String::from("ABCABC");
        let t = String::from("ABC");
        let result = gcd_of_strings(&s, &t);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_not_equal_gcd_of_strings(){
        let s = String::from("LEET");
        let t = String::from("CODE");
        let result = gcd_of_strings(&s, &t);
        assert_eq!(result, "");
    }
}