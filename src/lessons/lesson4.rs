pub(crate) fn lesson4() {
    println!("\n lesson4 : 流程控制");

    let s = 5;
    if s > 10 {
        println!("s > 10, is {}", s);
    } else if s == 0 {
        println!("s == 0, is {}", s);
    } else {
        println!("s < 10, is {}", s);
    }

    for m in 1..=5 {
        println!("{}", m);
    }

    let collection = [1, 2, 3, 4, 5];
    for item in collection {
        println!("{}", item);
    }

    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    for j in 1..4 {
        if j == 2 {
            break;
        }
        println!("{}", j);
    }

    let mut n = 0;
    while n <= 5  {
        println!("{}!", n);
        n = n + 1;
    }
    println!("我出来了！");

    //死循环
    // loop {
    //     println!("again!");
    // }
}