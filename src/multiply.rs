extern crate test;

// Implementations of multiplication from chapter 2.

use self::test::Bencher;

// multiply0 location 338
fn multiply0(n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    multiply0(n - 1, a) + a
}

#[bench]
fn bench_multiply0(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply0(3, 15);
            multiply0(15, 3);
            multiply0(16, 16);
            multiply0(41, 59);
        }
    })
}

fn odd(n: i64) -> bool { n & 0x1 == 1}
fn half(n: i64) -> i64 { n >> 1 }

// multiply1 location 372
fn multiply1(n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    let mut result = multiply1(half(n), a + a);
    if odd(n) {
        result = result + a;
    }
    result
}

#[bench]
fn bench_multiply1(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply1(3, 15);
            multiply1(15, 3);
            multiply1(16, 16);
            multiply1(41, 59);
        }
    })
}

// mult_acc0 location 426
fn mult_acc0(r: i64, n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    if odd(n) {
        mult_acc0(r + a, half(n), a + a)
    } else {
        mult_acc0(r, half(n), a + a)
    } 
}

// multiply2a = multiply using multi_acc0 at location 426
fn multiply2a(n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    mult_acc0(a, n - 1, a)
}


#[bench]
fn bench_multiply2a(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply2a(3, 15);
            multiply2a(15, 3);
            multiply2a(16, 16);
            multiply2a(41, 59);
        }
    })
}

// mult_acc0 location 426
fn mult_acc1(mut r: i64, n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    if odd(n) {
        r = r + a
    }
    mult_acc1(r, half(n), a + a)
}

// multiply2b = multiply using multi_acc1 at location 426
fn multiply2b(n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    mult_acc1(a, n - 1, a)
}


#[bench]
fn bench_multiply2b(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply2b(3, 15);
            multiply2b(15, 3);
            multiply2b(16, 16);
            multiply2b(41, 59);
        }
    })
}

// mult_acc2 location 438
fn mult_acc2(mut r: i64, n: i64, a: i64) -> i64 {
    if odd(n) {
        r = r + a;
        if n == 1 {
            return r;
        }
    }
    mult_acc2(r, half(n), a + a)
}

// multiply2c = multiply using multi_acc2 at location 438
fn multiply2c(n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    mult_acc2(a, n - 1, a)
}


#[bench]
fn bench_multiply2c(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply2c(3, 15);
            multiply2c(15, 3);
            multiply2c(16, 16);
            multiply2c(41, 59);
        }
    })
}

// mult_acc3 location 451
fn mult_acc3(mut r: i64, mut n: i64, mut a: i64) -> i64 {
    if odd(n) {
        r = r + a;
        if n == 1 {
            return r;
        }
    }
    n = half(n);
    a = a + a;
    mult_acc3(r, n, a)
}

// multiply2d = multiply using multi_acc2 at location 438
fn multiply2d(n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    mult_acc3(a, n - 1, a)
}


#[bench]
fn bench_multiply2d(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply2d(3, 15);
            multiply2d(15, 3);
            multiply2d(16, 16);
            multiply2d(41, 59);
        }
    })
}

// mult_acc4 location 451
fn mult_acc4(mut r: i64, mut n: i64, mut a: i64) -> i64 {
    loop {
        if odd(n) {
            r = r + a;
            if n == 1 {
                return r;
            }
        }
        n = half(n);
        a = a + a;
    }
}

// multiply2e = multiply using multi_acc2 at location 438
fn multiply2e(n: i64, a: i64) -> i64 {
    if n == 1 {
        return a;
    }
    mult_acc4(a, n - 1, a)
}

#[bench]
fn bench_multiply2e(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply2e(3, 15);
            multiply2e(15, 3);
            multiply2e(16, 16);
            multiply2e(41, 59);
        }
    })
}

// multiply3 location 463
fn multiply3(mut n: i64, mut a: i64) -> i64 {
    while !odd(n) {
        a = a + a;
        n = half(n);
    }
    if n == 1 {
        return a;
    }
    mult_acc4(a, n - 1, a)
}

#[bench]
fn bench_multiply3(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply3(3, 15);
            multiply3(15, 3);
            multiply3(16, 16);
            multiply3(41, 59);
        }
    })
}

// multiply4 location 476
fn multiply4(mut n: i64, mut a: i64) -> i64 {
    while !odd(n) {
        a = a + a;
        n = half(n);
    }
    if n == 1 {
        return a;
    }
    // even(n – 1) => n – 1 ≠ 1
    mult_acc4(a, half(n - 1), a + a)
}

#[bench]
fn bench_multiply4(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        for _ in 1..n {
            multiply4(3, 15);
            multiply4(15, 3);
            multiply4(16, 16);
            multiply4(41, 59);
        }
    })
}
