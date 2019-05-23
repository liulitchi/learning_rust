fn main() {
    let a = 12;
    {
        let a = 34;
    }     
    println!("a = {}", a);

    let b = 56;
    println!("The first b = {}", b);
    let b = 78;
    {
        let b = 90;
            println!("inside b = {}", b);
    }
    println!("outside b = {}", b);
    println!("b = {}", b);
}
