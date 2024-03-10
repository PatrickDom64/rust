//erros

fn main(){
    erros();
}

fn erros(){
    match resultado(){
        Ok(s) => println!("String de sucesso = {}",s);
        Err(numero) => println!("Codigo de erro = {}",numero)
    }
}

fn resultado() -> Result < String, u8>{
    Err(42)
}