fn main() {
    println!("123");
    digitLength("abcdefghijklmn");
}

// 转化【小数】为【整数】
fn float2Fixed() {}

// 获取【小数位】长度
fn digitLength(str: &str) {
    let v: Vec<&str> = str.split(|c| c == 'e' || c == 'E').collect();
    // println!("{:?}",v)  ;
    let part_a = "";
    match &v.get(0) {
        Some(e) => println!("11"),
        None => println!("12"),
    };
    let mut part_b = "";
    match &v.get(1) {
        Some(e) => println!("21"),
        None => println!("22"),
    }


    let frontV: Vec<&str> = part_a.split('.').collect();
    let mut str_front = "";
    match &frontV.get(1) {
        Some(third) => str_front = third,
        None => println!("not found"),
    };

    // if str_front == false {
    //     dlen_front =
    // }
    // let dLen_end = "";

    println!("--{:?}--{}--{}", v, part_a, part_b);

    println!("{}", str_front);
}
