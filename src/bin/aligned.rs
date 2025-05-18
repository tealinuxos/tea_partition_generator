fn main() {

    let value = 4097;
    let alignment = 2048;

    let ret = (value + alignment - 1) & !(alignment - 1);

    println!("{}", ret); 
}