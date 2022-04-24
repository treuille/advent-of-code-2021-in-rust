pub mod regex_to_tuple {
    use regex::Regex;

    pub trait FromStr<'a> {
        fn from_str(s: &'a str) -> Self;
    }

    impl<'a> FromStr<'a> for &'a str {
        fn from_str(s: &'a str) -> Self {
            s
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

    pub trait ToTuple<'a, Tuple> {
        fn to_tuple(&self, s: &'a str) -> Tuple;
    }

    impl<'a, T1, T2> ToTuple<'a, (T1, T2)> for Regex
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
    {
        fn to_tuple(&self, s: &'a str) -> (T1, T2) {
            let captures = self.captures(s).unwrap();
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3> ToTuple<'a, (T1, T2, T3)> for Regex
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
    {
        fn to_tuple(&self, s: &'a str) -> (T1, T2, T3) {
            let captures = self.captures(s).unwrap();
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
            )
        }
    }

    impl<'a, T1, T2, T3, T4> ToTuple<'a, (T1, T2, T3, T4)> for Regex
    where
        T1: FromStr<'a>,
        T2: FromStr<'a>,
        T3: FromStr<'a>,
        T4: FromStr<'a>,
    {
        fn to_tuple(&self, s: &'a str) -> (T1, T2, T3, T4) {
            let captures = self.captures(s).unwrap();
            (
                T1::from_str(captures.get(1).unwrap().as_str()),
                T2::from_str(captures.get(2).unwrap().as_str()),
                T3::from_str(captures.get(3).unwrap().as_str()),
                T4::from_str(captures.get(4).unwrap().as_str()),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::regex_to_tuple::ToTuple;
    use regex::Regex;

    #[test]
    fn pair_to_tuple() {
        let re = Regex::new(r"(\d+) -> ([a-z]+)").unwrap();
        let (a, b): (usize, &str) = re.to_tuple("1 -> abc");
        assert_eq!(a, 1);
        assert_eq!(b, "abc");
    }

    #[test]
    fn triple_to_tuple() {
        let re = Regex::new(r"(\d+) \+\+ ([a-z]+) \+\+ ([a-z])").unwrap();
        let (a, b, c): (usize, &str, char) = re.to_tuple("99 ++ xyz ++ q");
        assert_eq!(a, 99);
        assert_eq!(b, "xyz");
        assert_eq!(c, 'q');
    }

    #[test]
    fn quadruple_to_tuple() {
        let re = Regex::new(r"(\d+),(\d+),(\d+),(\d+)").unwrap();
        let (a, b, c, d): (usize, usize, usize, usize) = re.to_tuple("0,1,2,4");
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 2);
        assert_eq!(d, 4);
    }
}
