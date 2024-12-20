use std::io;
fn main() {
    let  mut x=String::new();
   println!("Enter Number x:{x} ");
   io::stdin()
   .read_line(&mut x)
   .expect("might be some error");

   let mut y=String::new();
   println!("Enter number y:{y}");
io::stdin()
.read_line(&mut y)
.expect("failed y");
//unwrap is used for converting into f32 if we will not use f32 no need to use is unwrap 
let x:f32=x.trim().parse().unwrap();
let y:f32=y.trim().parse().unwrap();


let sum =x+y;
let substraction: f32=x-y;
let multiplication :f32=x*y;
let divide :f32=x/y;

println!("sum is {sum}");
println!("sum is {substraction}");
println!("sum is {multiplication}");
println!("sum is {divide}");

}
