pub mod mysql_activity_repository;

#[cfg(test)]
mod tests {
    #[test]
    fn test_closure() {
        let x = vec![1, 2, 3, 4];
        let vec1 = x.clone();
        let closure = move || {
            // let vec1 = x.clone(); 在里面clone的话是会报错的，因为x.clone()是调用到的时候执行的，而不是定义闭包的时候
            //执行的，所以运行的时候，x已经被move到闭包里面去了
            println!("closure vec {:?}", vec1)
        };
        println!("out vec {:?}", x);
        closure();
    }
    #[test]
    fn test_closure2() {
        let x = vec![1, 2, 3, 4];
        let closure = || println!("closure vec {:?}", x);//这里没有添加move，所以对于闭包的行为就是借用，
        //而不是所有权的转移
        closure();
        println!("out vec {:?}", x);
    }
}
