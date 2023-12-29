const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn soma(a:i32, b:i32) -> i32
{
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);
    }

    println!("fora, a = {}", a);
}

fn escopo() {
    println!("PI = {}", PI);

    unsafe {
        println!("variavel_global = {}", GLOBAL);
    }

    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
    let variavel:i32 = 301;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let mut booleana:bool = true;
    println!("Booleana = {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}

fn main() {
    escopo();
    sombra();

    println!("Soma = {}", soma(2, 2));
    // println!("decimal = {}", decimal);
}   
