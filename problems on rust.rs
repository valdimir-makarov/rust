
// fn reverse(input : &str) -> String{
//     let  rev :  String = input.chars().rev().collect();
//      rev
// }
 fn recursion ( input : i32) {
 if(input==0){
     return ;
 }
 let int1 = input*(input-1);
  recursion(input-1);
 println!("{}",int1);
 
 
     
 }
 
fn main(){

let input =4;
 let ans =recursion(input);
    
}
