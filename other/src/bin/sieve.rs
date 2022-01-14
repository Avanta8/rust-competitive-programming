static mut SIEVE: Vec<bool> = vec![];
// static mut SIEVE: Vec<bool> = sieve(100);

fn do_sieve(size: usize) {
    unsafe {
        SIEVE = vec![true; size];

        SIEVE[0] = false;
        SIEVE[1] = false;
        for (n, _) in SIEVE.iter().enumerate().filter(|(_, x)| **x) {
            for i in (n..size).step_by(n).skip(1) {
                SIEVE[i] = false;
            }
        }
    }
}

fn get_sieve(size: usize) -> Vec<bool> {
    let mut sieve = vec![true; size];

    sieve[0] = false;
    sieve[1] = false;
    for n in 2..size {
        if !sieve[n] {
            continue;
        }
        for i in (n..size).step_by(n).skip(1) {
            sieve[i] = false;
        }
    }
    sieve
}

fn main() {
    let mut s = &vec![];

    unsafe {
        s = &SIEVE;
    }

    let size = 100;
    do_sieve(size);
    let sieve = get_sieve(size);

    // for i in 0..size {
    //     if sieve[i] {
    //         println!("{} is prime", i);
    //     }
    // }

    // println!("{:?}", s);
    // println!("{:?}", sieve);
    assert_eq!(s, &sieve);
    println!("{}", s == &sieve);

    // unsafe {
    //     for i in 0..size {
    //         if primes[i] {
    //             println!("{} is prime", i);
    //         }
    //     }

    //     println!("{:?}", primes);
    // }
}
