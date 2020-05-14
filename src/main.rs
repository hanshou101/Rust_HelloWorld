use std::num::ParseIntError;
use std::ops::Mul;

fn main() {
    let str = "0.12345789";
    let a = digitLength(str);
    let int_a = match str.parse::<f64>() {
        Ok(n) => { float2Fixed(n) }
        Err(_) => 0,
    };
    println!("{}", a);
    println!("{}", int_a)
}

/// 转化【小数】为【整数】
fn float2Fixed(num: f64) -> i64 {
    let str = num.to_string();
    println!("str {}", str);
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
fn digitLength(str: &str) -> i8 {
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
