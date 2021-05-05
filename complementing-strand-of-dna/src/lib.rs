pub fn complementing_strand_of_dna(dna: &str) -> String {
    dna.chars()
        .rev()
        .map(|char| match char {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => char,
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::complementing_strand_of_dna;

    #[test]
    fn it_works() {
        let test_dna = "AAAACCCGGT";
        let reverse_complement = complementing_strand_of_dna(test_dna);
        assert_eq!(reverse_complement, "ACCGGGTTTT");
    }
}
