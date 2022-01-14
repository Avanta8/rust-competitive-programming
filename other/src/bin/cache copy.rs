use std::collections::HashMap;

fn expensive(n: i64) -> i64 {
    println!("expensive: n = {}", n);
    n * 2
}

struct Cacher {
    f: fn(i64) -> i64,
    cache: HashMap<i64, i64>,
}

impl Cacher {
    fn new(f: fn(i64) -> i64) -> Self {
        Self {
            f,
            cache: HashMap::new(),
        }
    }

    fn call(&mut self, n: i64) -> i64 {
        println!("calling {}", n);
        *self.cache.entry(n).or_insert_with(|| (self.f)(n))
    }
}

fn fib(n: i64) -> i64 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    // let mut cache = HashMap::new();

    // let mut cacher = Cacher::new(expensive);

    // let five = cacher.call(5);
    // let five = cacher.call(5);
    // let five = cacher.call(5);

    // println!("{}", five);

    let v = fib(50);

    println!("{}", v);
}
