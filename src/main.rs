fn main() {
    /* ------ */
    let _x = 5; //不可变变量
    let _y = 5;//不可变变量，代码中没有使用到的可以用下划线开头
    let mut _z = 10;//可变变量，关键字:mut
    const THIS_IS_CONSTAND: u32 = 1000_000;//常量声明,rust中数字可以使用下划线来提高可读性
    let x = 17;
    {
        if x == 5 {
            //TODO:zheshiyigejiangyaowanchendedaima
            println!("{}", THIS_IS_CONSTAND)
        }
    }

    println!("zheshi iyge xiugai");
}
