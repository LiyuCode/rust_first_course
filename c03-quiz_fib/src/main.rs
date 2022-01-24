fn next_fib(a: &mut i8, b: &mut i8) {
    let c = *a + *b;
    *a = *b;
    *b = c;
}

fn fib_while(n: u8) {
    println!("fib_while...");
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i <= n {
        next_fib(&mut a, &mut b);
        i += 1;

        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    println!("fib_for...");

    let (mut a, mut b) = (1, 1);

    for _i in 2..n + 1 {
        next_fib(&mut a, &mut b);

        println!("next val is {}", b);
    }
}

fn fib_loop(n: u8) {
    println!("fib_loop...");

    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        next_fib(&mut a, &mut b);

        println!("next val is {}", b);
        
        i += 1;
        if i > n {
            break;
        }
    }

}

fn main() {
    fib_loop(8);
    fib_while(8);
    fib_for(8);

    println!("Hello, world!");
}
