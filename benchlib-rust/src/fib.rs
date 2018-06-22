pub fn fib(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2) 
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;   


     #[test]
    fn test_fib() {
       assert_eq!(fib(1), 1);
       assert_eq!(fib(2), 1);
       assert_eq!(fib(3), 2);
       assert_eq!(fib(4), 3);
       assert_eq!(fib(5), 5);
       assert_eq!(fib(20), 6765);
       assert_eq!(fib(43), 433494437)
    }
    
   // #[bench]
    fn bench_fib(b: &mut Bencher) {
       
        b.iter(||fib(43));
    }

}