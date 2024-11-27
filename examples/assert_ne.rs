use cleanass::assert_ne;

pub fn main() {
    // If assert succeeds nothing is printed since cleanup function does not run
    {
        let a = 1;
        let b = 2;
        assert_ne!(a, b, || eprintln!("Cleanup: {} != {} succeeded", a, b));
    }
    // If assert fails the cleanup function runs and prints the message
    {
        let a = 1;
        let b = 1;
        assert_ne!(
            a, b,
            || eprintln!("Cleanup: {} != {} failed", a, b),
            "Should not be equal"
        );
    }
}