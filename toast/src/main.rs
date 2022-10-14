const TOAST_ASCII:&str = r#"  ###### ######
 #......#......#
 #.............#
  #...........#
  #...........#
  #...........#
  #...........#
   ###########"#;

pub fn hello_toast(){
    println!("Welcome to Toast!");
    println!("=================\n{TOAST_ASCII}\n=================");
}

fn main(){
    hello_toast();
}