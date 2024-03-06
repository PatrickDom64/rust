const PI:f32 = 3.14;
static mut Variavel_global:u8 = 1;

fn sombra{
    let a = 123;

    {
        let b =456;
        println!("dentro, b = {}",b);
    }
      //  println!("fora, b = {}",b);
    println!("a = {}",a);
}

fn escopo(){
    println!("PI = {}",PI);
   
    unsafe{
        println!("Global = {}",Variavel_global);
    }

    let variavel:i32 = 300;
    println!("Inteiro = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.6;
    println!("Decimal = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    let mut booleano:bool = true;
    println!("booleana = {}, tamanho = {} bytes", booleano, std::mem::size_of_val(&booleano));

    let letra:char = 'C';
    println!("Letra = {}, tamanho = {} bytes", letra, std::mem::size_of_val(&letra));
}

fn main(){
    escopo();
    sombra();
   
}