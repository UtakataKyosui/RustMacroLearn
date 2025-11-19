use hello::HelloMacro;
use hello_derive::HelloMacro;
   
struct Pancakes;

// impl HelloMacro for Pancakes {
// 	fn hello_macro() {
// 		println!("Hello, Macro! My name is Pancakes!");
// 	}
// }

#[derive(HelloMacro)]
struct HotCake;

fn main() {
	// Pancakes::hello_macro();
	HotCake::hello_macro();
}