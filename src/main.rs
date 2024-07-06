use colour::*;
use std::io;

fn main() {
  menu();
}

fn menu(){
  loop {
    cyan_ln_bold!("=================");
    cyan_ln_bold!("   calculadora");
    cyan_ln_bold!("=================");


    let mut fst_num = String::new();
    let mut scd_num = String::new();
    let mut option = String::new();
    
    blue_ln_bold!("First number: ");
    io::stdin()
      .read_line(&mut fst_num)
      .expect("Failed to read 1st line");

    blue_ln_bold!("Second number: ");
    io::stdin()
      .read_line(&mut scd_num)
      .expect("Failed to read 2nd line");
    
    print!("{}[2J", 27 as char);
    blue_ln_bold!("Select option: ");
    blue_ln_bold!("1 - sum ");
    blue_ln_bold!("2 - minus ");
    blue_ln_bold!("3 - multiplication ");
    blue_ln_bold!("4 - division ");
    blue_ln_bold!("5 - quit ");

    io::stdin()
      .read_line(&mut option)
      .expect("Failed to read option");

    match option.trim().parse::<i32>().unwrap(){
      1=>sum(fst_num, scd_num),
      2=>minus(fst_num, scd_num),
      3=>multiplication(fst_num, scd_num),
      4=>division(fst_num, scd_num),
      5=>{
        green_ln_bold!("\nBye");
        break;
      },
      _=>red_ln_bold!("Invalid option"),
    }
  }
}

fn sum(a: String, b: String) {
  let n1 = a.trim().parse::<i32>().unwrap();
  let n2 = b.trim().parse::<i32>().unwrap();
  let r: i32 = n1 + n2;
  green_ln_bold!("{n1} + {n2} = {r}");
}

fn minus(a: String, b: String) {
  let n1 = a.trim().parse::<i32>().unwrap();
  let n2 = b.trim().parse::<i32>().unwrap();
  let r: i32 = n1 - n2;
  green_ln_bold!("{n1} - {n2} = {r}");
}

fn multiplication(a: String, b: String) {
  let n1 = a.trim().parse::<f32>().unwrap();
  let n2 = b.trim().parse::<f32>().unwrap();
  let r: f32 = n1 * n2;
  green_ln_bold!("{n1} x {n2} = {r}");
}

fn division(a: String, b: String) {
  let n1 = a.trim().parse::<f64>().unwrap();
  let n2 = b.trim().parse::<f64>().unwrap();
  let r: f64 = n1 / n2;
  green_ln_bold!("{n1} / {n2} = {r}");
}
