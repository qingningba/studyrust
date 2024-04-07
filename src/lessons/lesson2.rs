pub(crate) fn lesson2() {
    println!("\n lesson2 : 作用域");

    let s1 = String::from("hello");
    let s2 = s1;

    //s1报错，此时s1已经不在作用域内了，所以报错
    //println!("s1 = {}, s2 = {}", s1, s2);
    println!("s2 = {}", s2);

    let x: &str = "hello, world";
    let y = x;
    println!("{},{}",x,y);

    //完全复制变量
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    //值的引用
    let x = 5;
    let y = &x;
    println!("x = {}, y = {}", x, *y);
}