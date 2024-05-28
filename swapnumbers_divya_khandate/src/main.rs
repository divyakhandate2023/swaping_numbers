// Function to swap two numbers 
fn swap(a: i32, b: i32) -> (i32, i32) 
{
    (b, a)
}

fn main() 
{
    let a = 33;
    let b = 66;

    println!("Before swapping: a = {}, b = {}", a, b);

    // Swapping the values using the swap function
    let (a, b) = swap(a, b);

    println!("After swapping: a = {}, b = {}", a, b);
}

// Unit tests for the swap function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let (a, b) = swap(5, 10);
        assert_eq!((a, b), (10, 5));
    }

    #[test]
    fn test_swap_same_values() {
        let (a, b) = swap(7, 7);
        assert_eq!((a, b), (7, 7));
    }

    #[test]
    fn test_swap_negative_values() {
        let (a, b) = swap(-3, 8);
        assert_eq!((a, b), (8, -3));
    }

    #[test]
    fn test_swap_zero() {
        let (a, b) = swap(0, 10);
        assert_eq!((a, b), (10, 0));
    }

    #[test]
    fn test_swap_large_numbers() {
        let (a, b) = swap(1_000_000, 2_000_000);
        assert_eq!((a, b), (2_000_000, 1_000_000));
    }

    #[test]
    fn test_swap_min_max_i32() {
        let (a, b) = swap(i32::MIN, i32::MAX);
        assert_eq!((a, b), (i32::MAX, i32::MIN));
    }

    #[test]
    fn test_swap_in_quick_succession() {
        let (a, b) = swap(1, 2);
        let (a, b) = swap(a, b);
        assert_eq!((a, b), (1, 2)); // Should swap back to original
    }
    
    #[test]
    fn test_swap_large_negative_values() {
        let (a, b) = swap(-1_000_000, -2_000_000);
        assert_eq!((a, b), (-2_000_000, -1_000_000));
    }
    
    #[test]
    fn test_swap_max_and_zero() {
        let (a, b) = swap(i32::MAX, 0);
        assert_eq!((a, b), (0, i32::MAX));
    }
}
