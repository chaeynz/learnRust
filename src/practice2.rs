fn main() {
    let x: i32 = 3;
    {
        let y: i32 = 5;
        println!("Hi1 ", x, y);
    }
    println!("Hi2 ", x, y );
}