fn while_and_loop()
{
    let mut xay = 1;

    while xay < 1000
    {
        xay *= 2;

        if xay == 64 { continue; } //without x = 64

        println!("xay = {}", xay);
    }

    println!("----------");

    let mut yes = 1;
    
    loop
    {
        yes *= 2;

        println!("yes = {}", yes);

        if yes == 1 << 10 { break; } // 2^10 = 1024
    }
}
fn main() {
    while_and_loop();
}
