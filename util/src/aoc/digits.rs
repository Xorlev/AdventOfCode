pub struct Digits {
    n: u32,
    divisor: u32,
}

impl Digits {
    pub fn new(n: u32) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits { n, divisor }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
}