
Original post:
https://chrismorgan.info/blog/rust-fizzbuzz/


simple FizzBuzz implementation

Benchmark: 

1..1_000_000
Elapsed time: 14.222478597s

```rust
for i in 1..101 {
    if i % 15 == 0 {
        println!("FizzBuzz");
    } else if i % 5 == 0 {
        println!("Buzz");
    } else if i % 3 == 0 {
        println!("Fizz");
    } else {
        println!("{}", i);
    }
}
```


Benchmark:

1..1_000_000
Elapsed time: 4.861693918s

After optimize:

```rust
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let mut fizz_buzz = String::with_capacity(9_000_000); // Preallocate a buffer for the FizzBuzz sequence

    for i in 1..=1_000_000 {
        if i % 15 == 0 {
            fizz_buzz.push_str("FizzBuzz\n");
        } else if i % 5 == 0 {
            fizz_buzz.push_str("Buzz\n");
        } else if i % 3 == 0 {
            fizz_buzz.push_str("Fizz\n");
        } else {
            fizz_buzz.push_str(&format!("{}\n", i));
        }
    }

    println!("{}", fizz_buzz);

    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);
}
```


taking a reference as in but storing the owned string in the outer scope

```rust
for i in 1..101 {
    let x;
    let result = if i % 15 == 0 {
        "FizzBuzz"
    } else if i % 5 == 0 {
        "Buzz"
    } else if i % 3 == 0 {
        "Fizz"
    } else {
        x = i.to_string();
        &*x
    };
    println!("{}", result);
}
```


String all the way:this works, at a runtime cost

```rust
for i in 1..101 {
    let result = if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else {
        i.to_string()
    };
    println!("{}", result);
}
```



fizz_buzz function, with Strings.

```rust
fn fizz_buzz(i: i32) -> String {
    if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else {
        i.to_string()
    }
}

for i in 1..101 {
    println!("{}", fizz_buzz(i));
}
```


FizzBuzz function returning Cow<'static, str>

```rust
use std::borrow::Cow;

                        // &'static str
fn fizz_buzz(i: i32) -> Cow<'static, str> {
    if i % 15 == 0 {
        "FizzBuzz".into()
    } else if i % 5 == 0 {
        "Buzz".into()
    } else if i % 3 == 0 {
        "Fizz".into()
    } else {
        i.to_string().into()
    }
}

for i in 1..101 {
    println!("{}", fizz_buzz(i));
}
```


implementing `std::fmt::Display`

Benchmark:

1..1_000_000
Elapsed time: 14.05603776s

```rust
use std::fmt;

enum Term {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(i32),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Fizz => f.write_str("Fizz"),
            Term::Buzz => f.write_str("Buzz"),
            Term::FizzBuzz => f.write_str("FizzBuzz"),
            Term::Number(num) => write!(f, "{}", num),
        }
    }
}

// Useless. We can remove it.
impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

fn fizz_buzz(i: i32) -> Term {
    if i % 15 == 0 {
        Term::FizzBuzz
    } else if i % 5 == 0 {
        Term::Buzz
    } else if i % 3 == 0 {
        Term::Fizz
    } else {
        Term::Number(i)
    }
}

for i in 1..101 {
    println!("{}", fizz_buzz(i));
}
```

...or implement this loop instead in `std::fmt::Display`

```rust
for f in (1..101).map(fizz_buzz) {
    println!("{}", f);
}
```


Using a match expression:

```rust
fn fizzbuzz(n: u32) {
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}

fn main() {
    fizzbuzz(100);
}
```


Using a ternary operator:

```rust
fn fizzbuzz(n: u32) {
    for i in 1..=n {
        let result = if i % 3 == 0 && i % 5 == 0 {
            "FizzBuzz".to_string()
        } else if i % 3 == 0 {
            "Fizz".to_string()
        } else if i % 5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        };
        println!("{}", result);
    }
}
```


Using a loop and conditional statements:

```rust
fn fizzbuzz(n: u32) {
    let mut i = 1;
    while i <= n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
        i += 1;
    }
}
```


Using an iterator and map:

```rust
fn fizzbuzz(n: u32) {
    (1..=n).for_each(|i| {
        let result = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => i.to_string(),
        };
        println!("{}", result);
    });
}
```


Using a recursive function:

```rust
fn fizzbuzz_recursive(n: u32) {
    fizzbuzz_recursive_helper(1, n);
}

fn fizzbuzz_recursive_helper(current: u32, n: u32) {
    if current > n {
        return;
    }

    if current % 3 == 0 && current % 5 == 0 {
        println!("FizzBuzz");
    } else if current % 3 == 0 {
        println!("Fizz");
    } else if current % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", current);
    }

    fizzbuzz_recursive_helper(current + 1, n);
}
```



Using a for loop and a match expression with ranges:

```rust
fn fizzbuzz(n: u32) {
    for i in 1..=n {
        match i {
            x if x % 3 == 0 && x % 5 == 0 => println!("FizzBuzz"),
            x if x % 3 == 0 => println!("Fizz"),
            x if x % 5 == 0 => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}
```


Using an iterator, map, and collect:

```rust
fn fizzbuzz(n: u32) {
    let fizzbuzz_vec: Vec<String> = (1..=n)
        .map(|i| {
            if i % 3 == 0 && i % 5 == 0 {
                "FizzBuzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else {
                i.to_string()
            }
        })
        .collect();
    
    for item in fizzbuzz_vec {
        println!("{}", item);
    }
}
```


Using an iterator, map, and for_each:

```rust
fn fizzbuzz(n: u32) {
    (1..=n).map(|i| {
        match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => i.to_string(),
        }
    }).for_each(|item| println!("{}", item));
}
```


Using a recursive function and pattern matching:

```rust
fn fizzbuzz_recursive(n: u32) {
    fizzbuzz_recursive_helper(1, n);
}

fn fizzbuzz_recursive_helper(current: u32, n: u32) {
    match (current % 3, current % 5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        (_, _) => println!("{}", current),
    }

    if current < n {
        fizzbuzz_recursive_helper(current + 1, n);
    }
}
```



FizzBuzz using an `enum` and the `impl` keyword

```rust
enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl FizzBuzz {
    fn new(num: u32) -> FizzBuzz {
        if num % 3 == 0 && num % 5 == 0 {
            FizzBuzz::FizzBuzz
        } else if num % 3 == 0 {
            FizzBuzz::Fizz
        } else if num % 5 == 0 {
            FizzBuzz::Buzz
        } else {
            FizzBuzz::Number(num)
        }
    }

    fn to_string(&self) -> String {
        match self {
            FizzBuzz::Fizz => String::from("Fizz"),
            FizzBuzz::Buzz => String::from("Buzz"),
            FizzBuzz::FizzBuzz => String::from("FizzBuzz"),
            FizzBuzz::Number(num) => num.to_string(),
        }
    }
}

fn fizzbuzz(n: u32) {
    for i in 1..=n {
        let fb = FizzBuzz::new(i);
        println!("{}", fb.to_string());
    }
}
```


FizzBuzz Implementation with Generic FizzBuzzable Trait

```rust
trait FizzBuzzable {
    fn fizzbuzz(&self) -> String;
}

impl FizzBuzzable for u32 {
    fn fizzbuzz(&self) -> String {
        self.to_string()
    }
}

impl FizzBuzzable for &str {
    fn fizzbuzz(&self) -> String {
        self.to_string()
    }
}

fn fizzbuzz<T: FizzBuzzable>(n: T) {
    let n: u32 = match n.fizzbuzz().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    for i in 1..=n {
        let divisible_by_3 = i % 3 == 0;
        let divisible_by_5 = i % 5 == 0;

        match (divisible_by_3, divisible_by_5) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", i.fizzbuzz()),
        }
    }
}

fn main() {
    fizzbuzz(100u32);
    fizzbuzz("Hello, World!");
}
```



fizzbuzz with pattern matching, iterators, closures, and a custom-defined struct

```rust
struct FizzBuzz {
    current: u32,
    max: u32,
}

impl FizzBuzz {
    fn new(max: u32) -> Self {
        FizzBuzz {
            current: 1,
            max,
        }
    }
}

impl Iterator for FizzBuzz {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }

        let result = match (self.current % 3, self.current % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => self.current.to_string(),
        };

        self.current += 1;
        Some(result)
    }
}

fn main() {
    let fizz_buzz = FizzBuzz::new(100);

    for item in fizz_buzz {
        println!("{}", item);
    }
}
```



fizzbuzz with `where` keyword:

```rust
struct FizzBuzz<T>
where
    T: Iterator<Item = u32>,
{
    iterator: T,
}

impl<T> FizzBuzz<T>
where
    T: Iterator<Item = u32>,
{
    fn new(iterator: T) -> Self {
        FizzBuzz { iterator }
    }
}

impl<T> Iterator for FizzBuzz<T>
where
    T: Iterator<Item = u32>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(num) = self.iterator.next() {
            let result = match (num % 3, num % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                _ => num.to_string(),
            };

            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let numbers = 1..=100;
    let fizz_buzz = FizzBuzz::new(numbers);

    for item in fizz_buzz {
        println!("{}", item);
    }
}
```