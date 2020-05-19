extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// use std::num::ParseIntError;
// use std::ops::Mul;
use std::cmp;

/*

#[wasm_bindgen]
pub fn main() {
test____float2Fixed();
test____digitLength();
test____times();
test____divide();
test____plus();
test____minus();
}

fn test____float2Fixed() {
println!("test____float2Fixed");
let str = "0.12345789";
let int_a = match str.parse::<f64>() {
    Ok(n) => { float2Fixed(n) }
    Err(_) => 0,
};
println!("{}", int_a);
}

fn test____digitLength() {
println!("test____digitLength");
let str = "0.12345789";
let a = digitLength(str);
println!("{}", a);
}

fn test____times() {
println!("test____times");
println!("{}", times(21_0000.0, 1_0000.0, vec![&1000.0, &8.2]));        // 常见出错值1
println!("{}", times(0.012345, 0.000001, vec![]));                      // 常见出错值2
println!("{}", times(512.06, 100.0, vec![]));                           // 常见出错值3
println!("{}", times(0.11, 1.1, vec![]));                           // 常见出错值3
println!("{}", times(1.11, 10.0, vec![]));                           // 常见出错值3
println!("{}", times(512.06, 100.0, vec![]));                           // 常见出错值3
println!("{}", times(1.02, 0.05, vec![]));                           // 常见出错值3
println!("{}", times(575697.82, 100.0, vec![]));                           // 常见出错值3
println!("{}", times(0.1, 0.2, vec![]));                           // 常见出错值3
println!("{}", times(2090.5, 8.61, vec![]));                           // 常见出错值3
println!("{}", times(0.2, 0.4, vec![]));                           // 常见出错值3
println!("{}", times(6.0, 0.2, vec![]));                           // 常见出错值3
println!("{}", times(200.3, 3.0, vec![]));                           // 常见出错值3
println!("{}", times(600.9, 3.0, vec![]));                           // 常见出错值3
//
println!("{}", times(0.0000_0000_1, 0.0000_0000_2, vec![]));            // 超小值 * 超小值
println!("{}", times(0.0000_0000_1, 1_0000_0000_2.0, vec![]));          // 超小值 * 超大值
println!("{}", times(123456789.123456789, 123456789.123456789, vec![]));// 超大值 * 超大值
}

fn test____minus() {
println!("test____minus");
println!("{}", minus(1.1, 1.0, vec![]));
println!("{}", minus(57168.619999999995, 11087.28, vec![]));
println!("{}", minus(-0.09, 0.01, vec![]));
println!("{}", minus(2.0, 1.10, vec![]));
println!("{}", minus(0.3, 0.2, vec![]));
}

fn test____plus() {
println!("test____plus");
println!("{}", plus(0.1, 0.2, vec![]));
println!("{}", plus(0.28, 0.34, vec![]));
println!("{}", plus(2412.02, 11087.64, vec![&8338.28, &5580.0]));
println!("{}", plus(2.28, 2.34, vec![]));
println!("{}", plus(33.28, 3.34, vec![]));
println!("{}", plus(3.28, 3.34, vec![]));
println!("{}", plus(4.28, 4.34, vec![]));
println!("{}", plus(5.28, 5.34, vec![]));
println!("{}", plus(8.28, 8.34, vec![]));
println!("{}", plus(33.28, 9.34, vec![]));
}

fn test____divide() {
println!("test____divide");
println!("{}", divide(0.000001, 0.0001, vec![]));
println!("{}", divide(0.3, 0.2, vec![]));
}


*/

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
/// 精确减法
///     1.
///
#[wasm_bindgen]
pub fn minus(a: f64, b: f64, vars: Vec<&f64>) -> f64 {
    if vars.len() > 0 {
        return minus(minus(a, b, vec![]), *vars[0],
                     vars.clone().splice(1.., &[]).collect(),
        );
    }
    // 小数位，取最大值
    let scaleNum = cmp::max(digitLength(&a.to_string()), digitLength(&b.to_string()));
    let powRatio = 10__f64.powf(scaleNum as f64);

    (
        times(a, powRatio, vec![]) - times(b, powRatio, vec![])
    ) / powRatio
}

///
/// 精确加法
///     1.
///
#[wasm_bindgen]
pub fn plus(a: f64, b: f64, vars: Vec<&f64>) -> f64 {
    if vars.len() > 0 {
        return plus(plus(a, b, vec![]), *vars[0],
                    vars.clone().splice(1.., &[]).collect(),
        );
    }
    // 小数位，取最大值
    let scaleNum = cmp::max(digitLength(&a.to_string()), digitLength(&b.to_string()));
    let powRatio = 10__f64.powf(scaleNum as f64);

    (
        times(a, powRatio, vec![]) + times(b, powRatio, vec![])
    ) / powRatio
}


///
/// 精确减法
///     1.
///

///
/// 精确除法
///     1.
///
#[wasm_bindgen]
pub fn divide(a: f64, b: f64, vars: Vec<&f64>) -> f64 {
    if vars.len() > 0 {
        return divide(divide(a, b, vec![]), *vars[0],
                      // // 此处，简单的克隆并截取一下。然后取返回值（被替换掉的一段）
                      vars.clone().splice(1.., &[]).collect(),
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

///
/// 精确乘法
///     1.对于Vec的性能，可以做进一步优化
///
#[wasm_bindgen]
pub fn times(a: f64, b: f64, vars: Vec<&f64>) -> f64 {
    if vars.len() > 0 {
        return times(times(a, b, vec![]), *vars[0],
                     // // 此处，简单的克隆并截取一下。然后取返回值（被替换掉的一段）
                     vars.clone().splice(1.., &[]).collect(),
        );
    }
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
#[wasm_bindgen]
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
#[wasm_bindgen]
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
