use std::collections::HashMap;

#[derive(Default)]
struct Cacher {
    cache: HashMap<i64, i64>,
}

impl Cacher {
    fn fib(&mut self, n: i64) -> i64 {
        self.cache.get(&n).copied().unwrap_or_else(|| {
            let result = self.calc(n);
            self.cache.insert(n, result);
            result
        })
    }

    fn calc(&mut self, n: i64) -> i64 {
        if n == 1 {
            0
        } else if n == 2 {
            1
        } else {
            self.fib(n - 1) + self.fib(n - 2)
        }
    }
}

fn main() {
    // let v = fib(50);

    // println!("{}", v);

    let mut cacher = Cacher::default();

    let v = cacher.fib(50);
    println!("{}", v);
}
