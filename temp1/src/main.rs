fn if_statement()
  {
    let temp = 25;    // you can change this value for test

    if temp > 30
    {
      println!("really hot outside!");
    } 
    else if temp < 10
    {
      println!("really cold!");
    }
    else
    {
      println!("temperature is OK.");
    }
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}.", day);

    println!("is it {}?",
      if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"} );
    
    println!("it is {}.",
      if temp > 20 {
          if temp > 30 {"very hot"} else {"hot"} 
        } else if temp < 10 {"cold"} else {"OK"} );
    }


fn main() {
    if_statement();
}
