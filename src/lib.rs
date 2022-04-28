pub mod parse_regex {
    use regex::Regex;

    pub fn from_regex<'a, T>(re: &Regex, s: &'a str) -> T
        where T: FromRegex<'a> {
            T::parse(re, s)
        }

    // TODO; Try removeing `pub`
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

    // TODO; Try removeing `pub`
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

    // pub trait ToTuple<'a, Tuple>
    // where
    //     Self: Sized,
    // {
    //     fn parse_regex(&self, s: &'a str) -> Tuple;

    //     fn parse_lines(&'a self, lines: &'a str) -> LinesToTuples<'a, Tuple, Self> {
    //         LinesToTuples {
    //             str_parser: self,
    //             lines: lines.trim().lines(),
    //             _phantom: PhantomData,
    //         }
    //     }
    // }

    // pub struct LinesToTuples<'a, Tuple, ToTupleType>
    // where
    //     ToTupleType: ToTuple<'a, Tuple>,
    // {
    //     str_parser: &'a ToTupleType,
    //     lines: Lines<'a>,
    //     _phantom: PhantomData<Tuple>,
    // }

    // impl<'a, Tuple, ToTupleType> Iterator for LinesToTuples<'a, Tuple, ToTupleType>
    // where
    //     ToTupleType: ToTuple<'a, Tuple>,
    // {
    //     type Item = Tuple;

    //     fn next(&mut self) -> Option<Tuple> {
    //         self.lines.next().map(|s| self.str_parser.to_tuple(s))
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::parse_regex::from_regex;
    use regex::Regex;

    #[test]
    fn pair_parse_regex() {
        let re = Regex::new(r"(\d+) -> ([a-z]+)").unwrap();
        let (a, b): (usize, &str) = from_regex(&re, "1 -> abc");
        assert_eq!(a, 1);
        assert_eq!(b, "abc");
    }

    #[test]
    fn triple_parse_regex() {
        let re = Regex::new(r"(\d+) \+\+ ([a-z]+) \+\+ ([a-z])").unwrap();
        let (a, b, c): (usize, &str, char) = from_regex(&re, "99 ++ xyz ++ q");
        assert_eq!(a, 99);
        assert_eq!(b, "xyz");
        assert_eq!(c, 'q');
    }

    #[test]
    fn quadruple_parse_regex() {
        let re = Regex::new(r"(\-?\d+),(\d+),(\d+),(\d+)").unwrap();
        let (a, b, c, d): (isize, usize, usize, usize) = from_regex(&re, "-1,0,1,2");
        assert_eq!(a, -1);
        assert_eq!(b, 0);
        assert_eq!(c, 1);
        assert_eq!(d, 2);
    }

    #[test]
    fn lines_parse_regexs() {
        todo!("Do the line iterator.")
        // let input  "
        // a => x 
        // b => y 
        // c => z 
        // ";

        // let re = Regex::new(r"([a-z]) => ([a-z])").unwrap();
        // let mut iter = re.parse_lines(input);

        // assert_eq!(Some(('a', 'x')), iter.next());
        // assert_eq!(Some(('b', 'y')), iter.next());
        // assert_eq!(Some(('c', 'z')), iter.next());
        // assert_eq!(None, iter.next());
    }
}
