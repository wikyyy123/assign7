mod lib;
use Ass7;

mod first {
        pub mod sub_mod{
            pub fn mod_function(){
            println!("Hi this function is from first module");
            }
        }
    }

fn main() {
    println!("Welcome to Chapter 7 Assignment");
    println!("Question # 1 ");
    first::sub_mod::mod_function();

    println!("Welcome to Chapter 7 Assignment");
    println!("Question # 2 ");
    lib::mod_lib::sub_mod::mod_function();

    println!("Welcome to Chapter 7 Assignment");
    println!("Question # 3 ");
    Ass7::lib_pkg_mod::my_function();
}
