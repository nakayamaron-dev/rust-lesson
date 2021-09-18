pub fn run() {
    //所有権・参照・借用
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    let i1 = 1;
    let i2 = i1;

    println!("{} {}", i1, i2);
    println!("Static address of i1: {:p}", &i1);
    println!("Static address of i2: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Static address of i1: {:p}", &sl1);
    println!("Static address of i2: {:p}", &sl2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);
    println!("Static address of i1: {:p}", &s3);
    println!("Static address of i2: {:p}", &s4);
    println!("Heap memory address of s3: {:?}", s3.as_ptr());
    println!("Heap memory address of s4: {:?}", s4.as_ptr());

    let s5 = String::from("hello");
    println!("Static address of s5: {:p}", &s5);
    println!("Heap memory address of s5.1: {:?}", s5.as_ptr());
    println!("Len of s5 is: {}", s5.len());
    println!("Capacity of s5 is: {}", s5.capacity());
    take_ownership(s5);
    // println!("{}", s5); // 所有権が移動したため、エラーになる

    let s6 = String::from("hello");
    println!("Static address of s6: {:p}", &s6);
    println!("Heap memory address of s6.1: {:?}", s6.as_ptr());
    println!("Len of s6 is: {}", s6.len());
    let s7 = take_giveback_ownership(s6);
    println!("Static address of s7: {:p}", &s7);
    println!("Heap memory address of s7.1: {:?}", s7.as_ptr());
    println!("Len of s7 is: {}", s7.len());

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    // println!("{} {}", r1, r2);

    // let mut s11 = String::from("hello");
    // let r1 = &mut s11;
    // //println!の順序が逆であれば問題なく通る。
    // println!("{}", s11);
    // println!("{}", r1);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello_updated");
    println!("{}", s12);
}

fn take_ownership(s: String) {
    println!("Static address of s: {:p}", &s);
    println!("Heap memory address of s.1: {:?}", s.as_ptr());
    println!("Len of s is: {}", s.len());
    println!("Capacity of s is: {}", s.capacity());
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    //最後に;が無いものが自動的に戻り値になる。
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
