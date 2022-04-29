pub mod parse_regex {
    use std::marker::PhantomData;
    use std::str::Lines;
    use regex::Regex;

    pub fn parse_line<'a, T>(re: &Regex, s: &'a str) -> T
        where T: FromRegex<'a>
    {
        T::parse(re, s)
    }

    // pub fn parse_lines<'a, T>(re: &'a Regex, s: &'a str) -> ParseLines<'a, Lines<'a>, T>
    pub fn parse_lines<'a, T>(re: &'a Regex, s: &'a str) -> ParseLines<'a, Lines<'a>, T>
        where T: FromRegex<'a> ,
    {
        let str_iter = s.trim().lines();
        let _phantom =  PhantomData;
        ParseLines { re, str_iter, _phantom }
    }

    pub trait FromStr<'a> {
        fn from_str(s: &'a str) -> Self;
    }

    impl<'a> FromStr<'a> for &'a str {
        fn from_str(s: &'a str) -> Self {
            s
        }
    }

    impl FromStr<'_> for isize {
        fn from_str(s: &'_ str) -> Self {
            s.parse().unwrap()
        }
    }

    impl FromStr<'_> for usize {
        fn from_str(s: &'_ str) -> Self {
            s.parse().unwrap()
        }
    }

    impl FromStr<'_> for char {
        fn from_str(s: &'_ str) -> Self {
            s.parse().unwrap()
        }
    }

    pub trait FromRegex<'a> {
        fn parse(re: &Regex, s: &'a str) -> Self;
    }

    impl<'a, T1, T2> FromRegex<'a> for (T1, T2)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re.captures(s).unwrap();
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3> FromRegex<'a> for (T1, T2, T3)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re.captures(s).unwrap();
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3, T4> FromRegex<'a> for (T1, T2, T3, T4)
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
        T4: FromStr<'a>,
    {
        fn parse(re: &Regex, s: &'a str) -> Self {
            let captures = re.captures(s).unwrap();
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
                T4::from_str(captures.get(4).unwrap().as_str()),
            )
        }
    }

    pub struct ParseLines<'a, StrIter, T>
    where
        StrIter: Iterator<Item=&'a str>,
        T: FromRegex<'a>,
    {
        re: &'a Regex,
        str_iter: StrIter,
        _phantom: PhantomData<T>,
    }

    impl<'a, StrIter, T> Iterator for ParseLines<'a, StrIter, T>
    where
        StrIter: Iterator<Item=&'a str>,
        T: FromRegex<'a>,
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            self.str_iter.next().map(|s| T::parse(self.re, s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_regex::{parse_line, parse_lines};
    use regex::Regex;

    #[test]
    fn pair_parse_regex() {
        let re = Regex::new(r"(\d+) -> ([a-z]+)").unwrap();
        let (a, b): (usize, &str) = parse_line(&re, "1 -> abc");
        assert_eq!(a, 1);
        assert_eq!(b, "abc");
    }

    #[test]
    fn triple_parse_regex() {
        let re = Regex::new(r"(\d+) \+\+ ([a-z]+) \+\+ ([a-z])").unwrap();
        let (a, b, c): (usize, &str, char) = parse_line(&re, "99 ++ xyz ++ q");
        assert_eq!(a, 99);
        assert_eq!(b, "xyz");
        assert_eq!(c, 'q');
    }

    #[test]
    fn quadruple_parse_regex() {
        let re = Regex::new(r"(\-?\d+),(\d+),(\d+),(\d+)").unwrap();
        let (a, b, c, d): (isize, usize, usize, usize) = parse_line(&re, "-1,0,1,2");
        assert_eq!(a, -1);
        assert_eq!(b, 0);
        assert_eq!(c, 1);
        assert_eq!(d, 2);
    }

    #[test]
    fn lines_parse_regexs() {
        // todo!("Do the line iterator.")
        let input = "
        a => x 
        b => y 
        c => z 
        ";

        let re = Regex::new(r"([a-z]) => ([a-z])").unwrap();
        let mut iter = parse_lines(&re, input);

        assert_eq!(Some(('a', 'x')), iter.next());
        assert_eq!(Some(('b', 'y')), iter.next());
        assert_eq!(Some(('c', 'z')), iter.next());
        assert_eq!(None, iter.next());
    }
}
