//! # My Crate
//!
//! `my_crate`는 일부 연산을 더 쉽게 실행하기 위한 유틸리티 모음이다.

/// 주어진 숫자에 1을 더한다.
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
/// 
pub fn add_one(x: i32) -> i32 {
    x + 1
}
