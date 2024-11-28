use std::str::FromStr;

const DIGIT_NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// 是否为空白字符串
///
/// # 例子
/// ```
/// use rutool::str_util::is_blank;
///
/// let val = "";
/// assert!(is_blank(val));
///
/// let val = " ";
/// assert!(is_blank(val));
/// ```
pub fn is_blank(val: &str) -> bool {
    val.is_empty() || val.chars().filter(|x| !x.is_whitespace()).count() <= 0
}

/// 是否为非空白字符串
/// is_empty的逆逻辑
pub fn is_not_blank(val: &str) -> bool {
    !is_blank(val)
}

/// 判断是否为十进制数字
/// ```
/// use rutool::str_util::is_digit_number;
///
/// assert!(is_digit_number("1"));
/// assert!(is_digit_number("+1"));
/// assert!(is_digit_number("-1"));
/// assert!(is_digit_number(".1"));
/// assert!(is_digit_number("1.1"));
/// assert!(!is_digit_number("#"));
/// assert!(!is_digit_number("1-"));
/// assert!(!is_digit_number("1+"));
/// assert!(!is_digit_number("1.2.3"));
///
/// ```
pub fn is_digit_number(val: &str) -> bool {
    if is_blank(val) {
        false
    }else {
        let chars = val.chars();
        let mut point_cnt: usize = 0;
        for (idx, c) in chars.enumerate() {
            if c == '.' {
                point_cnt += 1;
            }
            if point_cnt > 1 {
                // 大于1个小数点
                return false;
            }
            if idx == 0 {
                let leg = c == '+' || c == '-' || c == '.' || DIGIT_NUMBERS.contains(&c);
                if !leg {
                    return false;
                }
                continue;
            }
            let leg = c == '.' || DIGIT_NUMBERS.contains(&c);
            if !leg {
                return false;
            }
        }
        true
    }
}