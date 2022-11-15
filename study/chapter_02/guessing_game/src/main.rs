use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // HEADER
    println!("Seja bem vindo ao jogo");
    
     // USING RAND LIB TO GET A RANDOM NUMBER
    let secret_random_number = rand::thread_rng().gen_range(1, 101);

    loop { //LOOP É SEMELHANTE AO `WHILE TRUE`
        // GETTING INPUT WITH HANDLER ERROR
        let mut guess = String::new();

        println!("Por-Favor, digite o seu palpite.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha!");

        // REDECLARANDO A VARIAVEL (GUESS) CONVERTENDO PARA OUTRO TIPO (SHADOWING / SOMBREAMENTO) 
        let guess: u32 = match guess
            .trim() //APAGA ESPACOS
            .parse() //CONVERTE PARA O TIPO INFERIDO
            {
                Ok(num) => num,
                Err(_) => continue
            };

       //COMPARING USING ENUMERATIONS
        match guess.cmp(&secret_random_number) {
            Ordering::Less => println!("Muito pequeno"),
            Ordering::Greater => println!("Muito grande"),
            Ordering::Equal => { 
                println!("Você ganhou!");
                break
            }
        }
    }
}
