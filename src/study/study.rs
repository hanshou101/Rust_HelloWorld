extern crate wasm_bindgen;
extern crate js_sys;

use wasm_bindgen::prelude::*;

// use std::num::ParseIntError;
// use std::ops::Mul;
use std::cmp;

use js_sys::Array;


#[test]
fn test() {
    println!("自定义计算 {}", 57_168_619_999_999_995 / 1.9 as i64);
    println!("自定义计算 {}", 57168619999999995.0 / 1.0);
    println!("自定义计算 {}", 57168619999999995.0 * 1.0);
    println!("自定义计算 {}", 57168619999999995 * (1 as i64));
    println!("自定义计算 {}", 1 / 2 as i64);
    println!("自定义计算 {}", 1 / 2 as i64);
    test____times_powsRatio();
    test____float2Fixed();
    test____digitLength();
    test____times();
    test____divide();
    test____plus();
    test____minus();
}

#[test]
fn test____times_powsRatio() {
    println!("test____times_powsRatio");
    assert_eq!(times_powsRatio(2.18, 2_i8), 218.0);
    assert_eq!(times_powsRatio(0.28, 2_i8), 28.0);
    assert_eq!(times_powsRatio(1.790234, 1_i8), 17.90234);
    assert_eq!(times_powsRatio(1.790234, 3_i8), 1790.234);
    assert_eq!(times_powsRatio(1.790234, 5_i8), 179023.4);
}

#[test]
fn test____float2Fixed() {
    println!("test____float2Fixed");
    let str = "0.123456789";
    let int_a = match str.parse::<f64>() {
        Ok(n) => { float2Fixed(n) }
        Err(_) => 0,
    };
    assert_eq!(123456789, int_a);
}

#[test]
fn test____digitLength() {
    println!("test____digitLength");
    assert_eq!(9, digitLength("0.123456789"));
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
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//


///
/// 精确加法
///     1.
///
#[test]
// #[wasm_bindgen]
pub fn plus(a: f64, b: f64, vars: Vec<f64>) -> f64 {
    if vars.len() > 0 {
        return plus(plus(a, b, vec![]), vars[0],
                    vars.clone().splice(1.., [].iter().cloned()).collect(),
        );
    }
    // 小数位，取最大值
    let scaleNum = cmp::max(digitLength(&a.to_string()), digitLength(&b.to_string()));
    let powRatio = 10__f64.powf(scaleNum as f64);

    println!("a {}", a);
    println!("a * powRatio {}", a * powRatio);
    println!("powRatio {}", powRatio);
    println!("b {}", b);
    println!("b * powRatio {}", b * powRatio);

    (
        // TIP 此处，特别处理一下JS原代码，存在的BUG！！！
        // TIP 此处，特别处理一下JS原代码，存在的BUG！！！
        // TIP 此处，特别处理一下JS原代码，存在的BUG！！！
        /// 尝试使用【times_powsRatio】，去单独进行处理（如果用常规的times，则倍数被放大4次，稍微大一点的数都无法承担）
        /// 尝试使用【times_powsRatio】，去单独进行处理（如果用常规的times，则倍数被放大4次，稍微大一点的数都无法承担）
        /// 尝试使用【times_powsRatio】，去单独进行处理（如果用常规的times，则倍数被放大4次，稍微大一点的数都无法承担）
        times_powsRatio(a, scaleNum) + times_powsRatio(b, scaleNum)
        // a * powRatio + b * powRatio
    ) / powRatio
}

///
/// 精确减法
///     1.
///
#[test]
// #[wasm_bindgen]
pub fn minus(a: f64, b: f64, mut vars: Vec<f64>) -> f64 {
    if vars.len() > 0 {
        return minus(minus(a, b, vec![]), vars[0],
                     vars.clone().splice(1.., [].iter().cloned()).collect(),
        );
    }
    // 小数位，取最大值
    let scaleNum = cmp::max(digitLength(&a.to_string()), digitLength(&b.to_string()));
    let powRatio = 10__f64.powf(scaleNum as f64);

    println!("a {}", a);
    println!("a * powRatio {}", a * powRatio);
    println!("powRatio {}", powRatio);
    println!("b {}", b);
    println!("b * powRatio {}", b * powRatio);

    (
        // TIP 此处，特别处理一下JS原代码，存在的BUG！！！
        // TIP 此处，特别处理一下JS原代码，存在的BUG！！！
        // TIP 此处，特别处理一下JS原代码，存在的BUG！！！
        /// 尝试使用【times_powsRatio】，去单独进行处理（如果用常规的times，则倍数被放大4次，稍微大一点的数都无法承担）
        /// 尝试使用【times_powsRatio】，去单独进行处理（如果用常规的times，则倍数被放大4次，稍微大一点的数都无法承担）
        /// 尝试使用【times_powsRatio】，去单独进行处理（如果用常规的times，则倍数被放大4次，稍微大一点的数都无法承担）
        times_powsRatio(a, scaleNum) - times_powsRatio(b, scaleNum)
        // a * powRatio - b * powRatio
    ) / powRatio
}

///
/// 精确乘法
///     1.对于Vec的性能，可以做进一步优化
///
#[test]
// #[wasm_bindgen]
pub fn times(a: f64, b: f64, vars: Vec<f64>) -> f64 {
    let isTwoFacInteger = rust_determineIfInteger(a) && rust_determineIfInteger(b);
    let moreVars = vars.len() > 0;
    let simpleSum = a * b;

    // TIP 特殊情况一
    if isTwoFacInteger && !moreVars {
        return simpleSum;
    }

    // TIP 特殊情况二：有后续多个算子
    if moreVars {
        let sum = if isTwoFacInteger {
            simpleSum
        } else {
            times(a, b, vec![])
        };
        return times(sum, vars[0],
                     // // 此处，简单的克隆并截取一下。然后取返回值（被替换掉的一段）
                     vars.clone().splice(1.., [].iter().cloned()).collect(),
        );
    }

    // TIP 特殊情况三：在【加法/减法】中，有一个值，为10的n次幂
    // 先写一个，性能较低的。
    /// 此时，联想到一点，a、b这两个值，分别进行迭代处理的时候，效率会极低。
    /// 此时，联想到一点，a、b这两个值，分别进行迭代处理的时候，效率会极低。
    /// 此时，联想到一点，a、b这两个值，分别进行迭代处理的时候，效率会极低。
    ///

    // TIP 其它的情况四：按照以前的

    // 放大为整数
    let zoom_a = float2Fixed(a);
    let zoom_b = float2Fixed(b);
    // 小数位相加
    let scaleNum = digitLength(&a.to_string()) + digitLength(&b.to_string());

    // 此处，可能导致溢出？？？
    // 此处，可能导致溢出？？？
    // 此处，可能导致溢出？？？
    // let leftV = (zoom_a * zoom_b) as f64;
    let leftV = match zoom_a.checked_mul(zoom_b) {
        Some(n) => n,
        None => {
            println!("a {}", a);
            println!("b {}", b);
            println!("zoom_a {}", zoom_a);
            println!("zoom_b {}", zoom_b);
            println!("溢出了！！！采取非精确算法");
            return a * b;
        }
    };
    println!("          times {}", leftV);
    println!("          times {}", scaleNum);
    let powRatio = 10__f64.powf(scaleNum as f64);
    println!("          times {}", powRatio);

    // TIP 此处，这是原先的代码，在计算【10.pow(12)】时，会报错溢出。
    // leftV / ((10__i32.pow(scaleNum as u32)) as f64)

    (leftV as f64) / (powRatio as f64)
}


///
/// 精确除法
///     1.
///
#[test]
// #[wasm_bindgen]
pub fn divide(a: f64, b: f64, vars: Vec<f64>) -> f64 {
    if vars.len() > 0 {
        return divide(divide(a, b, vec![]), vars[0],
                      // // 此处，简单的克隆并截取一下。然后取返回值（被替换掉的一段）
                      vars.clone().splice(1.., [].iter().cloned()).collect(),
        );
    }
    // 放大为整数
    let zoom_a = float2Fixed(a) as f64;
    let zoom_b = float2Fixed(b) as f64;
    // 小数位相减
    let scaleNum = digitLength(&a.to_string()) - digitLength(&b.to_string());

    let result = (zoom_a / zoom_b) as f64;

    println!("          times {}", result);
    println!("          times {}", scaleNum);

    let powRatio = 10__f64.powf(scaleNum as f64);
    println!("          times {}", powRatio);

    times(result, powRatio, vec![])
}

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

///
/// 转化【小数】为【整数】
///
#[test]
// #[wasm_bindgen]
pub fn float2Fixed(num: f64) -> i64 {
    let str = num.to_string();
    println!("          float2Fixed str {}", str);
    let isSci: bool = str.contains('e') || str.contains('E');
    if !isSci {             // 不是科学计数法
        match str.replace('.', "").parse::<i64>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    } else {                // 是科学计数法
        let dLen = digitLength(&str);
        if dLen > 0 {   // 自身是小数
            (num * (10__i8.pow(dLen as u32) as f64)) as i64
        } else {        // 自身就是整数
            num as i64
        }
    }
}

/// 获取【小数位】长度
///     1.此处，采用纯字符串，而不是正则分割
///         1.原因：更加节省空间。
///
#[test]
// #[wasm_bindgen]
pub fn digitLength(str: &str) -> i8 {
    let eSplit: Vec<&str> = str.split(|c| c == 'e' || c == 'E').collect();

    let front = match eSplit.get(0) {
        None => "",
        Some(&str) => &str,
    };
    let end = match eSplit.get(1) {
        None => "0",
        Some(&str) => &str,
    };

    let front_v: Vec<&str> = front.split('.').collect();
    let front_len = match front_v.get(1) {                  // 小数位，部分
        None => 0,                                  // 0
        // TIP 【*&】等于【什么都不写】
        Some(&str) => *&str.chars().count() as i8,   // 具体大小
    };
    let env_len = match end.parse::<i8>() {
        Ok(n) => n,
        Err(_) => 0,
    };

    front_len + env_len

    // match eSplit.get(1) {
    //     Some(&str) => {                // 科学计数法
    //         let front = str;
    //     }
    //     None => {                   // 并非科学计数法
    //     }
    // }
    //
    // let partA = eSplit.get(0);
    //
    // let mut lenA = 0 as &usize;
    // if let Some(&e) = partA {        // 有值
    //     let sDecimal: Vec<&str> = e.split('.').collect();
    //
    //     if let Some(&b) = sDecimal.get(0) {
    //         lenA = &b.chars().count();
    //     } else {}
    // } else {                               // 无值
    // }


    // let part_a = "";
    // match &v.get(0) {
    //     Some(e) => println!("11"),
    //     None => println!("12"),
    // };
    // let mut part_b = "";
    // match &v.get(1) {
    //     Some(e) => println!("21"),
    //     None => println!("22"),
    // }
    //
    //
    // let frontV: Vec<&str> = part_a.split('.').collect();
    // let mut str_front = "initial";
    // match &frontV.get(1) {
    //     Some(third) => str_front = third,
    //     None => println!("not found"),
    // };
    //
    // // if str_front == false {
    // //     dlen_front =
    // // }
    // // let dLen_end = "";
    //
    // // println!("--{:?}--{}--{}", v, part_a, part_b);
    // // 带换行格式
    // println!("{:#?}{}{}", v, part_a, part_b);
    // println!("{}", str_front);
}

/// 检查，是否整数
#[test]
fn rust_determineIfInteger(a: f64) -> bool {
    a.trunc() == a
}

///
/// 1.首先，基数放大。
/// 2.然后，减去基数放大的倍数？？？
///
#[test]
fn times_powsRatio(base: f64, scaleNum: i8) -> f64 {
    println!("<------ base {} scaleNum {}", base, scaleNum);
    let scaleRatio = 10_f64.powf(scaleNum as f64);
    // 放大为整数
    let zoom_a = float2Fixed(base);

    let enlargeRatio = (zoom_a as f64 / base).round();      // 转化为整型后，变化了的倍数
    // let min = cmp::max(enlargeRatio, scaleRatio);

    println!("zoom_a {} enlargeRatio {} scaleRatio {}", zoom_a, enlargeRatio, scaleRatio);
    println!("enlargeRatio / scaleRatio {}", enlargeRatio / scaleRatio);
    println!("scaleRatio / enlargeRatio {}", scaleRatio / enlargeRatio);


    /// FIXME 在这一步，【57168619999999995】从i64，转化为f64；就直接丢失了精度！！！
    /// FIXME 在这一步，【57168619999999995】从i64，转化为f64；就直接丢失了精度！！！
    /// FIXME 在这一步，【57168619999999995】从i64，转化为f64；就直接丢失了精度！！！
    /// FIXME 在这一步，【57168619999999995】从i64，转化为f64；就直接丢失了精度！！！
    /// FIXME 在这一步，【57168619999999995】从i64，转化为f64；就直接丢失了精度！！！
    /// FIXME 在这一步，【57168619999999995】从i64，转化为f64；就直接丢失了精度！！！
    /// FIXME 所以，因为类似的各种各样边界条件的限制；各种尝试，在这里结束。
    println!("zoom_a f64 {}", zoom_a as f64);

    let res_a = divide(zoom_a as f64, (enlargeRatio / scaleRatio), vec![]);
    let res_b = (zoom_a * (scaleRatio as i64 / enlargeRatio as i64)) as f64;

    let result = if enlargeRatio > scaleRatio {
        // zoom_a / (enlargeRatio as i64 / scaleRatio as i64)           // 用【整数除】补足倍数
        res_a               // 用【整数除】补足倍数
    } else {
        // zoom_a * (scaleRatio as i64 / enlargeRatio as i64)           // 用【整数乘】补足倍数
        /// TIP 此处，乘法，用【两个整数】成，没有问题
        res_b         // 用【整数乘】补足倍数
    };
    println!("result {}  zoom_a/1.0 {} ------>", result, result / 1.0);

    result as f64
}
