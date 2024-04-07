pub(crate) struct Struct {
    e: i32,
}

pub(crate) fn lesson1() {
    println!("lesson1 : 变量");
    // 添加mut关键字使变量可以被赋值
    let mut x = 5;
    println!("x value is {}", x);
    x = 6;
    println!("x value is {}", x);

    //带下划线的变量没被使用可以不被编译器警告未使用该变量
    let _x = 5;

    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);


    //变量赋值的多种形式
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    println!("{} {} {} {} {}", a, b, c, d, e);

    //变量遮蔽，相当于重新创建了这个变量
    let t = 5;
    let t = t + 1;
    println!("t = {}", t)
}