use colored::*;
use std::collections::BTreeMap;

pub fn show_flag(n: u64) {
    let s = format!("{:b}", n).chars().rev().collect::<Vec<char>>();
    let mut info = vec![];
    for (i, k) in s.iter().enumerate() {
        if *k as u64 - 48 == 1 {
            let x = 2u64.pow(i as u32);
            info.push(x);
        }
    }
    let txt = BTreeMap::from([
        (
            1,
            "0x1     PAIRED: paired-end (or multiple-segment) sequencing technology",
        ),
        (
            2,
            "0x2     PROPER_PAIR: each segment properly aligned according to the aligner",
        ),
        (4, "0x4     UNMAP: segment unmapped"),
        (8, "0x8     MUNMAP: next segment in the template unmapped"),
        (16, "0x10    REVERSE: SEQ is reverse complemented"),
        (
            32,
            "0x20    MREVERSE: SEQ of the next segment in the template is reverse complemented",
        ),
        (64, "0x40    READ1: the first segment in the template"),
        (128, "0x80    READ2: the last segment in the template"),
        (256, "0x100   SECONDARY: secondary alignment"),
        (512, "0x200   QCFAIL: not passing quality controls"),
        (1024, "0x400   DUP: PCR or optical duplicate"),
        (2048, "0x800   SUPPLEMENTARY: supplementary alignment"),
    ]);
    println!("0x{:x} {}: {:?}\n", n, n, info);
    println!("explain:\n    http://www.htslib.org/doc/1.11/samtools-flags.html\n");
    for (k, v) in txt {
        if info.contains(&k) {
            println!("{:>5}\t{}", k.to_string().blue(), v.red());
        } else {
            println!("{:>5}\t{}", k, v);
        }
    }
}
