//Ownership

fn ownership(){
    let uma_string = String::from("Patrick");
    rouba(&mut uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &mut String){
    string.push_str("Dom");
    println!("{}", string);
}

fn main(){
    //ownership();
}
