use std::io;

fn main() {
    println!("Enter a number: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
              
    println!("Fibonacci number is: {}", fib(number.trim().parse().unwrap()));

}

fn fib(n: i32) -> i32{
    let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
    dp[0] = 0;
    dp[1] = 1;

    for i in 2..=n{ 
        dp[i as usize] = dp[(i-1) as usize] + dp[(i-2) as usize];
        println!("dp[{}] is: {}", i, dp[i as usize]);
    }
    dp[n as usize]
}
