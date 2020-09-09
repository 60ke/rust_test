use hello_macro_derive::HelloMacro;
use hello_macro::HelloMacro;


#[derive(HelloMacro)]
struct Pancakes;

fn main(){
    let a = Pancakes::hello_macro();
    print!("{:?}",a);
}