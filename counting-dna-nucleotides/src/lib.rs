pub fn get_dna_nucleotides_count(dna: &str) -> (u64, u64, u64, u64) {
    let mut adenine = 0;
    let mut cytosine = 0;
    let mut guanine = 0;
    let mut thymine = 0;

    for char in dna.chars() {
        match char {
            'A' => adenine += 1,
            'C' => cytosine += 1,
            'G' => guanine += 1,
            'T' => thymine += 1,
            _ => {}
        }
    }

    return (adenine, cytosine, guanine, thymine);
}

#[cfg(test)]
mod tests {
    use crate::get_dna_nucleotides_count;

    #[test]
    fn it_works() {
        let test_dna = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        let nucleotides_count = get_dna_nucleotides_count(test_dna);
        assert_eq!(nucleotides_count, (20, 12, 17, 21));
    }
}
