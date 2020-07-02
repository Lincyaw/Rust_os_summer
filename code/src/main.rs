
#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// This function should calculate `a` divided by `b` if `a` is
// evenly divisible by b.
// Otherwise, it should return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0{
        Err(DivisionError::DivideByZero)
    }
    else if a%b==0{
        Ok(a/b)
    }
    else
    {
        Err(DivisionError::NotDivisible(NotDivisibleError{dividend:a, divisor:b}))
    }

}

fn main() {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));

    println!("{:?}",division_results);
    let x = division_results.iter();//... Fill in here!
    println!("{}",format!("{:?}", x));
}
