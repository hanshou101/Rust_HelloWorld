extern crate wasm_bindgen;
extern crate js_sys;
extern crate bigdecimal;

use wasm_bindgen::prelude::*;
// use js_sys::Array;
use bigdecimal::{BigDecimal, Zero, FromPrimitive, ToPrimitive};

#[test]
fn test() {
    println!("自定义计算 {}", 57_168_619_999_999_995 / 1.9 as i64);
    println!("自定义计算 {}", 57168619999999995.0 / 1.0);
    println!("自定义计算 {}", 57168619999999995.0 * 1.0);
    println!("自定义计算 {}", 57168619999999995 * (1 as i64));
    println!("自定义计算 {}", 1 / 2 as i64);
    println!("自定义计算 {}", 1 / 2 as i64);
    test____times();
    test____divide();
    test____plus();
    test____minus();
}

#[test]
fn test____times() {
    println!("test____times");
    assert_eq!(17_220_000_000_000.0, times(210_000.0, 10_000.0, vec![1_000.0, 8.2]));        // 常见出错值1
    assert_eq!(0.000000012345, times(0.012345, 0.000001, vec![]));                      // 常见出错值2

    assert_eq!(51206.0, times(512.06, 100.0, vec![]));                           // 常见出错值3
    assert_eq!(0.121, times(0.11, 1.1, vec![]));                           // 常见出错值3
    assert_eq!(11.1, times(1.11, 10.0, vec![]));                           // 常见出错值3
    assert_eq!(51206.0, times(512.06, 100.0, vec![]));                           // 常见出错值3
    assert_eq!(0.051, times(1.02, 0.05, vec![]));                           // 常见出错值3
    assert_eq!(57569782.0, times(575697.82, 100.0, vec![]));                           // 常见出错值3
    assert_eq!(0.02, times(0.1, 0.2, vec![]));                           // 常见出错值3
    assert_eq!(17_999.205, times(2090.5, 8.61, vec![]));                           // 常见出错值3
    assert_eq!(0.08, times(0.2, 0.4, vec![]));                           // 常见出错值3
    assert_eq!(1.2, times(6.0, 0.2, vec![]));                           // 常见出错值3
    assert_eq!(600.9, times(200.3, 3.0, vec![]));                           // 常见出错值3
    assert_eq!(1802.7, times(600.9, 3.0, vec![]));                           // 常见出错值3
    //
    assert_eq!(0.000000000000000002, times(0.000_000_001, 0.000_000_002, vec![]));            // 超小值 * 超小值
    assert_eq!(1.000000002, times(0.000_000_001, 1_000_000_002.0, vec![]));          // 超小值 * 超大值

    // TIP 正确的精准值
    // assert_eq!(15_241_578_780_673_678.515_622_620_750_191, times(123456789.123456789, 123456789.123456789, vec![]));// 超大值 * 超大值
    // TIP 小数浮点的估算值
    assert_eq!(15_241_578_780_673_680.0, times(123456789.123456789, 123456789.123456789, vec![]));// 超大值 * 超大值
}

#[test]
fn test____minus() {
    println!("test____minus");
    assert_eq!(0.1, minus(1.1, 1.0, vec![]));
    assert_eq!(46_081.339_999_999_995, minus(57_168.619_999_999_995, 11087.28, vec![]));
    assert_eq!(-0.1, minus(-0.09, 0.01, vec![]));
    assert_eq!(0.9, minus(2.0, 1.10, vec![]));
    assert_eq!(0.1, minus(0.3, 0.2, vec![]));
}

#[test]
fn test____plus() {
    println!("test____plus");
    println!("{}", plus(0.1, 0.2, vec![]));
    println!("{}", plus(0.28, 0.34, vec![]));
    println!("{}", plus(2412.02, 11087.64, vec![8338.28, 5580.0]));
    println!("{}", plus(2.28, 2.34, vec![]));
    println!("{}", plus(33.28, 3.34, vec![]));
    println!("{}", plus(3.28, 3.34, vec![]));
    println!("{}", plus(4.28, 4.34, vec![]));
    println!("{}", plus(5.28, 5.34, vec![]));
    println!("{}", plus(8.28, 8.34, vec![]));
    println!("{}", plus(33.28, 9.34, vec![]));
}

#[test]
fn test____divide() {
    println!("test____divide");
    println!("{}", divide(0.000001, 0.0001, vec![]));
    println!("{}", divide(0.3, 0.2, vec![]));
}

//
//
//
//
//
//

/// 精确加法
#[wasm_bindgen]
pub fn plus(a: f64, b: f64, vars: Vec<f64>) -> f64 {
    // (BigDecimal::new(a, 10) + BigDecimal::new(b, 10)).to_f64()
    /*
            let bigA = BigDecimal::from_f64(a).unwrap();
            let bigB = BigDecimal::from_f64(b).unwrap();
            (bigA + bigB).to_f64().unwrap()
    */
    if vars.len() > 0 {
        return plus(plus(a, b, vec![]), vars[0],
                    vars.clone().splice(1.., [].iter().cloned()).collect(),
        );
    }
    (
        BigDecimal::from_f64(a).unwrap() + BigDecimal::from_f64(b).unwrap()
    ).to_f64().unwrap()
}

/// 精确减法
#[wasm_bindgen]
pub fn minus(a: f64, b: f64, mut vars: Vec<f64>) -> f64 {
    if vars.len() > 0 {
        return minus(minus(a, b, vec![]), vars[0],
                     vars.clone().splice(1.., [].iter().cloned()).collect(),
        );
    }
    (
        BigDecimal::from_f64(a).unwrap() - BigDecimal::from_f64(b).unwrap()
    ).to_f64().unwrap()
}

/// 精确乘法
#[wasm_bindgen]
pub fn times(a: f64, b: f64, vars: Vec<f64>) -> f64 {
    if vars.len() > 0 {
        return times(times(a, b, vec![]), vars[0],
                     vars.clone().splice(1.., [].iter().cloned()).collect(),
        );
    }
    (
        BigDecimal::from_f64(a).unwrap() * BigDecimal::from_f64(b).unwrap()
    ).to_f64().unwrap()
}

/// 精确除法
#[wasm_bindgen]
pub fn divide(a: f64, b: f64, vars: Vec<f64>) -> f64 {
    if vars.len() > 0 {
        return divide(divide(a, b, vec![]), vars[0],
                      // // 此处，简单的克隆并截取一下。然后取返回值（被替换掉的一段）
                      vars.clone().splice(1.., [].iter().cloned()).collect(),
        );
    }
    (
        BigDecimal::from_f64(a).unwrap() / BigDecimal::from_f64(b).unwrap()
    ).to_f64().unwrap()
}