use std::io;

use complementing_strand_of_dna::complementing_strand_of_dna;

fn main() {
    println!("Please input DNA string:");

    let mut dna = String::new();

    io::stdin()
        .read_line(&mut dna)
        .expect("Failed to read line");

    let reverse_complement = complementing_strand_of_dna(&dna);

    println!("Reverse complement:");
    println!("{}", reverse_complement);
}
