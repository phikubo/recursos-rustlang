use std::io;
fn main() {
    println!("First Fibonacci in Rust!");
    loop{
        println!("Write how many Fbn numbers you want:");
        let mut inkeyboard = String::new();
        
        io::stdin()
            .read_line(&mut inkeyboard)
            .expect("Failed to get input!");
        println!("-----");

        let inkeyboard: u32 = match inkeyboard.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let mut suma:usize=0;
        let mut anterior=1;
        let mut counter=0;
        while counter!=inkeyboard {
            suma=suma+anterior;
            anterior=suma-anterior;
            counter+=1;
            println!("{}",suma);
        }
        //println!("La última iteración {}, resulta en el número fibonacci {}.", counter, suma);

    }
}
