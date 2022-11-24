use rust_htslib::{bam, bam::Read};
use std::process::exit;

use crate::utils::*;

pub fn depth(
    bamin: &Option<&str>,
) -> Result<(), rust_htslib::errors::Error> {
    let mut bam = bam_reader(bamin)?;
    for r in bam.records().flatten() {
        println!("{:?}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}", 
            std::str::from_utf8(r.qname()).unwrap(), 
            r.flags(), r.cigar(),r.insert_size(),
            r.mtid(),r.mpos(),r.mapq(),r.pos(), r.seq_len(),r.seq()
        );
        //println!("{}\t{}\t{}\t{}\t{}\t{:?}",r.mtid(),r.mpos(),r.mapq(),r.pos(), r.seq_len(),r.seq());    
    }
    /*for pileup in bam.pileup().flatten() {
        //print!("{:?}\t",p);
        //println!("{}:{}:{}",p.tid(),p.depth(),p.pos()+1);
        println!("{}:{} depth {}", pileup.tid(), pileup.pos(), pileup.depth());

        for alignment in pileup.alignments() {
            match alignment.indel() {
                bam::pileup::Indel::Ins(len) => println!("Insertion of length {}", len),
                bam::pileup::Indel::Del(len) => println!("Deletion of length {}", len),
                _ => println!("Base {}", alignment.record().seq()[alignment.qpos().unwrap()])
            }
        }
    }*/
    Ok(())
}

