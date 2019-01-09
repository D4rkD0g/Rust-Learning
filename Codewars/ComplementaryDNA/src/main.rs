
fn DNA_strand(dna: &str) -> String {
  // Translate the DNA strand
  let mut ans: String = "".to_string();
    for c in dna.chars() {
        match c {
            'A' => ans.push('T'),
            'T' => ans.push('A'),
            'C' => ans.push('G'),
            'G' => ans.push('C'),
            _   => {}
        }
    }
    ans
}

#[test]
fn returns_expected() {
  assert_eq!(DNA_strand("AAAA"),"TTTT");
  assert_eq!(DNA_strand("ATTGC"),"TAACG");
  assert_eq!(DNA_strand("GTAT"),"CATA");
}
