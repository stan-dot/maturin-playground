use pyo3::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;


#[pyfunction]
fn guess_the_number(){
    println!("guess the number");
    let secret_number = rand::rng().random_range(1..101);
    loop {
        println!("please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_) =>continue,
        };
        println!("you guessed: {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("You win");
                break;
            }
        }
    }
}

#[pymodule]
fn guessing_game(m: &Bound<'_, PyModule>) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(guess_the_number, m)?)?;
    Ok(())
}