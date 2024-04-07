struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
enum PokerSuit {
    Diamonds,
    Hearts,
}
pub(crate) fn lesson3() {
    println!("\n lesson3 : 数据类型");
    //有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)
    let a: i32 = 100;
    let b: i32 = -200;
    let c: u32 = 100;
    let d: f32 = -100.0;
    let e: f32 = 100.0;
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    //1..=5 表示1到5的整数, 1..5 表示1到4的整数
    for i in 1..=5 {
        println!("{}",i);
    }

    //字符类型(char)、字符串类型(String)，两种类型只有String可以进行字符串的拼接
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, g: {}, heart_eyed_cat: {}", c, z, g, heart_eyed_cat);

    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {}, world: {}", hello, world);

    let slice1 = &s[..2];
    let slice2 = &s[..];
    println!("slice1: {}, slice2: {}", slice1, slice2);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    //结构体
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("email: {}", user1.email);
    println!("username: {}", user1.username);
    println!("active: {}", user1.active);
    println!("sign_in_count: {}", user1.sign_in_count);
    println!();

    //枚举
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    println!("heart: {:?}, diamond: {:?}", heart, diamond);
    println!();

    //数组
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    //数组切片
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    println!("slice: {:?}, slice.len: {}", slice, slice.len())

}