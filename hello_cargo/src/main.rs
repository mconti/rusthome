// Appunti disordinati di Rust
// https://docs.google.com/document/d/1qENpz3XHRNK9uYw7naONtsvFWH_rfwxjEPPi7zk2Kzw/edit

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Indovina il numero!\n");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Il numero segreto è: {}", secret_number);
    
    loop {
        println!("Inserisci un numero...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Errore nella lettura del carattere");

        // Rust ha un strumento chiamato shadow che consente di annotare su una stessa variabile
        // (guess di tipo stringa) un altro sottotipo, ad esempio int32 (u32 unsigned 32bit)
        let guess: u32 = guess.trim().parse().expect("Devi inserire un numero!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // ora guess può essere usata sia come stringa sia come unsigned int 32
        println!("Tu hai ipotizzato: {}", guess);

        // Rust ha anche strumneti come match che consentono di gestire correttamente
        // tutti i valori di risorno di una chiamata multipla.
        // Una specie di switch
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("troppo piccolo!"),
            Ordering::Greater => println!("troppo grande!"),
            Ordering::Equal => {
                println!("Indovinato!");
                break;
            },
        }
    }
}
