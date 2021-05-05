use std::io;

use transcribing_dna_into_rna::transcribing_dna_into_rna;

fn main() {
    println!("Please input DNA string:");

    let mut dna = String::new();

    io::stdin()
        .read_line(&mut dna)
        .expect("Failed to read line");

    let rna = transcribing_dna_into_rna(&dna.trim());

    println!("Transcribing DNA into RNA:");
    println!("{}", rna);
}
