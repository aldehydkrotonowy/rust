fn main(){
    println!("Hello Rust");
    println!("{}, {}", "Norbert Gajda", "rust");
    println!("{0}, {1}", "first part", "second Part");
    println!("{greeting}, {name}", greeting="Good morning", name="Jon");

    println!("{:?}", [1,2,3,4,5]);
    println!("{:#?}", [5,6,7,8,9]);
    
    let x = format!("{}, {}!", "ala", "ma kot");
    println!("{}", x);

    let y = String::from("Hello, ") + "world!";
    println!("{}",y)
}
