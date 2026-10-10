use std::io;
 
// Reads one number typed by the user
fn read_number(prompt: &str) -> f64 {
    println!("   {} ", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
 
fn trapezium() -> f64 {
    let base1 = read_number("Base 1  :");
    let base2 = read_number("Base 2  :");
    let height = read_number("Height  :");
    height / 2.0 * (base1 + base2)
}
 
fn rhombus() -> f64 {
    let d1 = read_number("Diagonal 1:");
    let d2 = read_number("Diagonal 2:");
    0.5 * d1 * d2
}
 
fn parallelogram() -> f64 {
    let base = read_number("Base     :");
    let altitude = read_number("Altitude :");
    base * altitude
}
 
fn cube() -> f64 {
    let side = read_number("Side :");
    6.0 * side * side
}
 
fn cylinder() -> f64 {
    let radius = read_number("Radius :");
    let height = read_number("Height :");
    3.14159 * radius * radius * height
}
 

fn main() {
    println!();
    println!("====================================");
    println!("       THE SHAPE CALCULATOR         ");
    println!("====================================");
    println!("   1. Trapezium      (area)         ");
    println!("   2. Rhombus        (area)         ");
    println!("   3. Parallelogram  (area)         ");
    println!("   4. Cube           (surface area) ");
    println!("   5. Cylinder       (volume)       ");
    println!("====================================");
 
    println!("");
    let choice = read_number("Choose a shape (1-5):");
    println!("");
 
   
    let (name, kind, answer, unit) = if choice == 1.0 {
        println!("── Trapezium ──");
        ("Trapezium", "Area", trapezium(), "centimeter square")
    } else if choice == 2.0 {
        println!("── Rhombus ──");
        ("Rhombus", "Area", rhombus(), "centimeter square")
    } else if choice == 3.0 {
        println!("── Parallelogram ──");
        ("Parallelogram", "Area", parallelogram(), "centimeter square")
    } else if choice == 4.0 {
        println!("── Cube ──");
        ("Cube", "Surface area", cube(), "centimeter square ")
    } else if choice == 5.0 {
        println!("── Cylinder ──");
        ("Cylinder", "Volume", cylinder(), "centimeter square")
    } else {
        println!("Invalid choice. Please run again and pick 1-5.");
        return;
    };

    println!(" ");
    println!(" {} {} == {:.2} , {}", name, kind, answer, unit);
} 
