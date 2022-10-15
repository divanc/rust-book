pub fn print() {
    {
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }

    {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    let number = if true { 5 } else { 6 };

}


fn count() {
  let counter = 0;
  
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }

    println!("Passing result is {counter}");
  } 
}

fn labeled_loops() {
  'outer: loop {
    'inner: loop {
      break 'outer;
    }
  }
}

fn while_loop() {
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFTOFF!!!");
}

fn iteration() {
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
      println!("the value is: {}", a[index]);

      index += 1;
  }

  for element in a {
    println!("the value is: {element}");
}

for number in (1..4).rev() {
  println!("{}!", number);
}