     
     
     
     fn factorial(n: i32) -> Result<i32, i32> {
            fn f(n: i32) -> i32 {
                if n == 0 { 1 } else { n * f(n - 1) }
            }
            if n >= 0 {
                Ok(f(n))
            } else {
                Err(n)
            }
        }