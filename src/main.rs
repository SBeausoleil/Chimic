mod concentrate;
mod element;
mod errors;
mod expression;
mod molecule;
mod reaction;

fn main() {
    let hydrogen = element::new(1, String::from("H"), String::from("Hydrogen"), 1, 1, 1.0);
    let chlorine = element::new(
        17,
        String::from("Cl"),
        String::from("Chlorine"),
        2,
        7,
        35.45,
    );
    println!("{} and {}", hydrogen, chlorine);
    let hcl = molecule::new(vec![(hydrogen, 1), (chlorine, 1)]);
    println!("{}", hcl);
}
