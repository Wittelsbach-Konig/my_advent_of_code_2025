pub fn floor(a: i32, b: i32) -> i32 {
    let q = a / b;
    let r = a % b;
    if r != 0 && (a ^ b) < 0 { q - 1 } else { q }
}

pub fn ceil(a: i32, b: i32) -> i32 {
    let q = a / b;
    let r = a % b;
    if r != 0 && (a ^ b) >= 0 { q + 1 } else { q }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor() {
        assert_eq!(floor(8, 3), 2);
        assert_eq!(floor(8, -3), -3);
        assert_eq!(floor(-8, 3), -3);
        assert_eq!(floor(-8, -3), 2);

        assert_eq!(floor(1, 2), 0);
        assert_eq!(floor(1, -2), -1);
        assert_eq!(floor(-1, 2), -1);
        assert_eq!(floor(-1, -2), 0);
    }

    #[test]
    fn test_ceil() {
        assert_eq!(ceil(8, 3), 3);
        assert_eq!(ceil(8, -3), -2);
        assert_eq!(ceil(-8, 3), -2);
        assert_eq!(ceil(-8, -3), 3);

        assert_eq!(ceil(1, 2), 1);
        assert_eq!(ceil(1, -2), 0);
        assert_eq!(ceil(-1, 2), 0);
        assert_eq!(ceil(-1, -2), 1);
    }
}
