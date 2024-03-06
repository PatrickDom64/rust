fn main(){
    const PI:f32 = 3.14;

    println!("PI = {}",PI);

    let variavel:i32 = 300;
    println!("Inteiro = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.6;
    println!("Decimal = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    let mut booleano:bool = true;
    println!("booleana = {}, tamanho = {} bytes", booleano, std::mem::size_of_val(&booleano));

    let letra:char = 'C';
    println!("Letra = {}, tamanho = {} bytes", letra, std::mem::size_of_val(&letra));
}