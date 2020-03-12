#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd_of_strings() {
        let result = gcd_of_strings(String::from("ABCABCABC"), String::from("ABC"));
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

/// 求两个字符串的最大公约字符串
///
/// # Arguments
///
/// * `str1` - 第一个字符串
/// * `str2` - 第二个字符串
/// 
/// # Examples
///
/// ```rust
/// use rust_algorithm::gcd_of_strings;
/// let result = gcd_of_strings(String::from("ABCABCABC"), String::from("ABC"));
/// assert_eq!(result.unwrap(), "ABC");
/// ```
pub fn gcd_of_strings(str1: String, str2: String) -> Option<String> {
    // 先算出两者长度的最大公约数
    let str1_len = str1.len();
    let str2_len = str2.len();
    let gcd_result = gcd(str1_len as i32, str2_len as i32);
    // 取出公约数长度的前缀串
    let prefix_str: String = str1.chars().take(gcd_result as usize).collect();
    // 两个字符串必须都可以通过前缀串自我拼接多次得到
    if gcd_of_strings_check(prefix_str.clone(), str1, gcd_result) && gcd_of_strings_check(prefix_str.clone(), str2, gcd_result) {
        return Some(prefix_str);
    }
    return None;
}

fn gcd_of_strings_check (prefix_str: String, str1: String, gcd_result: i32) -> bool {
    let mut temp_str: String = String::from("");
    let mut result: bool = false;
    for _ in 1..=str1.len() as i32 / gcd_result {
        temp_str += &prefix_str; // &String可以自动转换为一个&str。这个功能叫做Deref转换
        if str1 == temp_str {
            result = true;
            break;
        }
    }
    return result;
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
/// use rust_algorithm::gcd;
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

