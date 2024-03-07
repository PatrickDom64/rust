fn repeticoes{
    let multiplicador:u8 = 5;

    let mut contador:u8 = 0;

    while contador < 10{
        contador +=1;
        println!("{} X {} = {}", multiplicador,contador, multiplicador*contador);        
    }
}



fn soma(a:i32, b:i32 ) -> i32
{
    println!("{} + {} = {}", a,b,a+b);
}
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

    println!("Soma = {}",soma(2,2));

    let idade:u8 = 17
    let responsavel_autorizou = true;

    if idade > 18 || idade > 16 && responsavel_autorizou = true{
        println!("Pode entrar na balada!")
    }else{
        println!("Não pode entrar na balada!")
    }

    let condicao;
    if idade > 18{
        condicao = "maior";
    }else{
        condicao = "menor";
    }

    println!("É {} de idade",condicao);
}