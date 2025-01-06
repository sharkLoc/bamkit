use rust_htslib::bam::{self, IndexedReader, Read};
use crate::utils::*;

pub fn targets(
    bam: &str,
    reg: &str,
    sam: bool,
    bamo: Option<&str>,
) -> Result<(), rust_htslib::errors::Error> {

    let mut bam = IndexedReader::from_path(bam)?;
    let headers = bam::Header::from_template(bam.header());

    let region = parse_reg(reg);
    bam.fetch(region)?;

    let mut fo = if sam {
        bam_writer(bamo, headers, bam::Format::Sam)?
    } else {
        bam_writer(bamo, headers, bam::Format::Bam)?
    };
    for rec in bam.records().flatten() {
        fo.write(&rec)?;
    }

    Ok(())
}

fn parse_reg(reg: &str) -> (&str, u64, u64) {
    let info = reg.split(':').collect::<Vec<&str>>();
    let pos = info[1].split('-').collect::<Vec<&str>>();
    let s = pos[0]
        .parse::<u64>()
        .expect("error: invalid start postion !");
    let e = pos[1].parse::<u64>().expect("error: invalid end postion !");
    (info[0], s, e)
}