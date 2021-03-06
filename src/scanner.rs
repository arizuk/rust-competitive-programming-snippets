// https://github.com/kenkoooo/competitive-programming-rs/blob/3152ff8749e0879f310a65e064a6fa7bb9fb69bd/src/utils/scanner.rs#L1
// Thanks kenkoooo!

#[snippet = "scanner"]
pub struct Scanner<R> {
    pub reader: R,
}

#[snippet = "scanner"]
impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf: String = self
            .reader
            .by_ref()
            .bytes()
            .map(|b| b.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect();
        buf.parse::<T>().ok().expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

#[test]
fn test_scanner() {
    use std::io::BufReader;
    {
        let r = BufReader::new("12\n34".as_bytes());
        let mut scanner = Scanner { reader: r };
        let n: usize = scanner.read();
        assert_eq!(n, 12);

        let n: usize = scanner.read();
        assert_eq!(n, 34);
    }
    {
        let r = BufReader::new("ab\n34".as_bytes());
        let mut scanner = Scanner { reader: r };
        let chars = scanner.chars();
        assert_eq!(chars, ['a', 'b']);
    }
    {
        let r = BufReader::new("1 2 3".as_bytes());
        let mut scanner = Scanner { reader: r };
        let ns: Vec<usize> = scanner.read_vec(3);
        assert_eq!(ns, [1, 2, 3]);
    }
}
