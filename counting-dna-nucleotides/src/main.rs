use std::io;

use counting_dna_nucleotides::get_dna_nucleotides_count;

fn main() {
    println!("Please input DNA string:");

    let mut dna = String::new();

    io::stdin()
        .read_line(&mut dna)
        .expect("Failed to read line");

    let (a, c, g, t) = get_dna_nucleotides_count(&dna);

    println!("Counting DNA Nucleotides:");
    println!("{} {} {} {}", a, c, g, t);
}
