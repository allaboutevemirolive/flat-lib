trait Divisible {
    type Output: Default + Clone;

    fn divisible_by(&self, other: Self::Output) -> bool;
}

impl<T> Divisible for T
where
    T: PartialEq + Copy + std::ops::Rem<Output = T> + Default,
{
    type Output = T;

    fn divisible_by(&self, other: Self::Output) -> bool {
        if other == T::default() {
            return false;
        }

        *self % other == T::default()
    }
}

struct FizzBuzz<T> {
    current: T,
    max: T,
}

impl<T> FizzBuzz<T>
where
    T: Divisible<Output = T>
        + std::default::Default
        + std::fmt::Display
        + std::ops::AddAssign
        + std::cmp::PartialOrd
        + std::ops::Add<Output = T>,
{
    fn new(max: T) -> Self {
        FizzBuzz {
            current: T::default() + <T as Divisible>::Output::default(), // Initialize with 1
            max,
        }
    }
}

impl<T> Iterator for FizzBuzz<T>
where
    T: Divisible<Output = T>
        + std::default::Default
        + std::fmt::Display
        + std::ops::AddAssign
        + std::cmp::PartialOrd
        + Clone
        + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }

        let current = self.current.clone();

        self.current += T::default() + <T as Divisible>::Output::default();
        let divisible_by_fizz = current.divisible_by(<T as Divisible>::Output::default());
        let divisible_by_buzz = current.divisible_by(self.current.clone() - T::default());

        let result = if divisible_by_fizz && divisible_by_buzz {
            "FizzBuzz".to_string()
        } else if divisible_by_fizz {
            "Fizz".to_string()
        } else if divisible_by_buzz {
            "Buzz".to_string()
        } else {
            current.to_string()
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
