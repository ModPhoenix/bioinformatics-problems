use std::collections::HashMap;

use counting_dna_nucleotides::get_dna_nucleotides_count;

pub fn gc_content(dna: &str) -> f64 {
    let (_, c, _, g) = get_dna_nucleotides_count(dna);

    let gc = g + c;

    (gc * 100) as f64 / (dna.len() - 1) as f64
}

pub fn fasta_format_parser(fasta: &str) -> Option<(String, f64)> {
    let mut fasta_map = HashMap::new();

    fasta.trim().split(">").skip(1).for_each(|item| {
        let item_data: Vec<&str> = item.trim().split("\n").collect();

        let label = item_data[0];
        let dna = format!("{}{}", item_data[1].trim(), item_data[2].trim());

        fasta_map.insert(label.to_string(), gc_content(&dna));
    });

    let max_gc_content = fasta_map
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    match max_gc_content {
        Some(value) => Some((value.0.into(), value.1.clone())),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{fasta_format_parser, gc_content};

    #[test]
    fn gc_content_works() {
        assert_eq!(gc_content("AGCTATAG"), 37.5);
    }

    #[test]
    fn fasta_format_parser_works() {
        let dataset = r#"
            >Rosalind_6404
            CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
            TCCCACTAATAATTCTGAGG
            >Rosalind_5959
            CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
            ATATCCATTTGTCAGCAGACACGC
            >Rosalind_0808
            CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
            TGGGAACCTGCGGGCAGTAGGTGGAAT
        "#;

        let max_gc_content = fasta_format_parser(&dataset);

        match max_gc_content {
            Some(value) => {
                println!("{}", value.0);
                println!("{}", value.1);
            }
            None => {}
        }

        assert_eq!(2 + 2, 5);
    }
}
