trait Divisible {
    type Output;

    fn divisible_by(self, other: Self) -> bool;
}

impl<T> Divisible for T
where
    T: PartialEq + Copy + std::ops::Rem<Output = T>,
{
    type Output = T;

    fn divisible_by(self, other: Self) -> bool {
        self % other == Self::Output::default()
    }
}

struct FizzBuzz<T> {
    current: T,
    max: T,
}

impl<T> FizzBuzz<T>
where
    T: Divisible
        + std::default::Default
        + std::fmt::Display
        + std::ops::AddAssign
        + std::cmp::PartialOrd,
{
    fn new(max: T) -> Self {
        FizzBuzz {
            current: T::default(),
            max,
        }
    }
}

impl<T> Iterator for FizzBuzz<T>
where
    T: Divisible
        + std::default::Default
        + std::fmt::Display
        + std::ops::AddAssign
        + std::cmp::PartialOrd,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            return None;
        }

        self.current += T::Output::default();
        let result = if self.current.divisible_by(3) && self.current.divisible_by(5) {
            "FizzBuzz".to_string()
        } else if self.current.divisible_by(3) {
            "Fizz".to_string()
        } else if self.current.divisible_by(5) {
            "Buzz".to_string()
        } else {
            self.current.to_string()
        };

        Some(result)
    }
}

fn main() {
    let fizz_buzz: FizzBuzz<u32> = FizzBuzz::new(100);

    for item in fizz_buzz {
        println!("{}", item);
    }
}
