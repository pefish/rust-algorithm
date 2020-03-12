#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd_of_strings() {
        let result = gcd_of_strings(String::from("ABCABC"), String::from("ABC"));
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), "ABC");

        let result1 = gcd_of_strings(String::from("HAHA"), String::from("ABC"));
        assert_eq!(result1.is_none(), true);
    }

    #[test]
    fn test_gcd() {
        let result = gcd(8, 4);
        assert_eq!(result, 4);

        let result1 = gcd(8, 6);
        assert_eq!(result1, 2);
    }
}


pub fn gcd_of_strings(str1: String, str2: String) -> Option<String> {
    // 先算出两者长度的最大公约数
    let str1_len = str1.len();
    let str2_len = str2.len();
    
    return Some(String::from("ABC"));
}

/// 求两个数的最大公约数（greatest common divisor）
///
/// # Arguments
///
/// * `int1` - 第一个数
/// * `int2` - 第二个数
/// 
/// # Examples
///
/// ```rust
/// let result = gcd(8, 4);
/// assert_eq!(result, 4);
/// ```
pub fn gcd(int1: i32, int2: i32) -> i32 {
    if int1 % int2 == 0 {
        return int2;
    } else {
        return gcd(int2, int1 % int2);
    }
}

