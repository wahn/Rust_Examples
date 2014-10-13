use std::rand;

static NTASKS: uint = 20;

fn sum_first_then_divide(slice: &[uint]) -> f32 {
    let len = slice.len();
    let mut sum = 0u;
    for i in range(0u, len) {
        sum += slice[i];
    }
    let fsum = sum as f32;
    let flen = len as f32;
    fsum / flen
}

fn recursive_sum(slice: &[uint]) -> f32 {
    let len = slice.len();
    let mut sum;
    match len {
        0u => {
            sum = 0.0f32;
        },
        1u => {
            sum = slice[0] as f32;
        },
        2u => {
            sum = (slice[0] + slice[1]) as f32 / 2.0f32;
        },
        _ => {
            let lm1 = (len - 1) as f32;
            let flen = len as f32;
            let f1 = lm1 / flen;
            let f2 = 1.0f32 / flen;
            let slm1 = slice[len-1] as f32;
            sum = f1 * recursive_sum(slice.slice(0, len-1)) + f2 * slm1;
        },
    };
    sum
}

fn do_sequential(id: uint) {
    // create array (filled with zeros)
    let mut a = [0u, ..7000u];
    for i in range(1u, 10_001) {
        // fill array (randomly) with values in [0,99]
        for i in range(0u, 7000u) {
            a[i] = rand::random::<uint>() % 1000u;
        }
        let b = a; // non-mutable copy
        // sum1
        let sum1 = sum_first_then_divide(b);
        // sum2
        let sum2 = recursive_sum(b);
        if i % 1000u == 0u {
            println!("({}, {}): sum1 = {}", id, i, sum1);
            println!("({}, {}): sum2 = {}", id, i, sum2);
        }
    }
}

fn do_in_parallel() {
    for p in range(0u, NTASKS) {
        spawn(proc() {
            do_sequential(p);
        });
    }
}

fn main() {
    println!("do_sequential()");
    do_sequential(0);
    println!("do_in_parallel()");
    do_in_parallel();
}
