pub fn transcribing_dna_into_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

#[cfg(test)]
mod tests {
    use crate::transcribing_dna_into_rna;

    #[test]
    fn it_works() {
        let test_dna = "GATGGAACTTGACTACGTAAATT";
        let rna = transcribing_dna_into_rna(test_dna);
        assert_eq!(rna, "GAUGGAACUUGACUACGUAAAUU");
    }
}
