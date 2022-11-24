use rust_htslib::{bam, bam::IndexedReader, bam::Read};
use std::process::exit;

use crate::utils::*;

#[allow(non_snake_case)]
pub fn view(
    bamin: &Option<&str>,
    Header: bool,
    bamf: bool,
    bamo: &Option<&str>,
) -> Result<(), rust_htslib::errors::Error> {
    let mut bam = bam_reader(bamin)?;
    if Header {
        let headers = bam::Header::from_template(bam.header());
        bam_writer(bamo, headers, bam::Format::Sam)?;
        exit(0);
    }
    if bamf {
        let headers = bam::Header::from_template(bam.header());
        let mut fo = bam_writer(bamo, headers, bam::Format::Bam)?;
        for rec in bam.records().flatten() {
            fo.write(&rec)?;
        }
    } else {
        let headers = bam::Header::from_template(bam.header());
        let mut fo = bam_writer(bamo, headers, bam::Format::Sam)?;
        for rec in bam.records().flatten() {
            fo.write(&rec)?;
        }
    }
    Ok(())
}

pub fn target(
    bam: &str,
    reg: &str,
    sam: bool,
    bamo: &Option<&str>,
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
