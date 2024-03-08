//Ownership

fn ownership(){
    let uma_string = String::from("Patrick");
    rouba(uma_string);

    println!("{}", uma_string);
}

fn rouba(string: String){
    println!("{}", string);
}

fn main(){
    ownership();
}