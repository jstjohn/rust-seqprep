extern crate bio;
use bio::alphabets;
use bio::data_structures::suffix_array::suffix_array;
use bio::data_structures::bwt::{bwt, less, Occ};
use bio::data_structures::fmindex::{FMIndex, FMIndexable};
use bio::utils::TextSlice;

pub fn example(input: TextSlice) {
    let text = b"ACAGCTCGATCGGTA";
    let alphabet = alphabets::dna::iupac_alphabet();
    let pos = suffix_array(text);
    let bwt = bwt(text, &pos);
    let less = less(&bwt, &alphabet);
    let occ = Occ::new(&bwt, 3, &alphabet);
    let fmindex = FMIndex::new(&bwt, &less, &occ);
    if alphabet.is_word(input){
        let interval = fmindex.backward_search(input.iter());
        let positions = interval.occ(&pos);
        println!("Positions {:?}", positions);
    }
}
