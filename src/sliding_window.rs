#[snippet = "sliding_window"]
pub mod ds {
    use std::collections::VecDeque;

    pub struct SlidingWindowQ<F> {
        n: usize,
        window: usize,
        q: VecDeque<usize>,
        cur: usize,
        f: F,
    }

    impl<F> SlidingWindowQ<F>
    where
        F: Fn(usize, usize) -> bool,
    {
        /// ```ignore
        /// let mut q = SlidingWindowQ::new(3, 2, |a, b| f(a) <= f(b));
        /// ```
        pub fn new(n: usize, window: usize, f: F) -> Self {
            SlidingWindowQ {
                n: n,
                q: VecDeque::new(),
                window: window,
                f: f,
                cur: 0,
            }
        }

        /// Returns the index of the window's front element
        pub fn next(&mut self) -> Option<usize> {
            let i = self.cur;
            if i == self.n {
                return None;
            }

            while self.q.len() > 0 {
                let j = *self.q.back().unwrap();

                if (self.f)(i, j) {
                    self.q.pop_back();
                } else {
                    break;
                }
            }
            self.q.push_back(i);

            let j = *self.q.front().unwrap();
            if i >= self.window && j == i - self.window {
                self.q.pop_front();
            }

            self.cur += 1;
            self.front()
        }

        pub fn front(&self) -> Option<usize> {
            self.q.front().map(|&v| v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ds::SlidingWindowQ;

    #[test]
    fn test_sliding_window_minimum() {
        let data = [5, 3, 2, 1, 4, 4, 4];
        let f = |a, b| data[a] <= data[b];
        let mut q = SlidingWindowQ::new(data.len(), 3, f);

        let mut ret = vec![];
        while let Some(idx) = q.next() {
            ret.push(idx);
        }
        assert_eq!(ret, vec![0, 1, 2, 3, 3, 3, 6]);
    }

    #[test]
    fn test_sliding_window_minimum_random() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let data_size = rng.gen_range(1, 1000);
            let mut data = vec![];
            for _ in 0..data_size {
                data.push(rng.gen_range(0, 1000));
            }
            let window = rng.gen_range(1, data_size + 1);
            let mut q = SlidingWindowQ::new(data_size, window, |a, b| data[a] <= data[b]);

            for i in 0..data_size {
                let idx = q.next().unwrap();
                let l = if i + 1 >= window { i + 1 - window } else { 0 };
                let r = i + 1;
                let expected = *(&data[l..r]).iter().min().unwrap();
                assert_eq!(data[idx], expected);
            }
        }
    }
}
