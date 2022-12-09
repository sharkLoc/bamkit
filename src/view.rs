use rand::{prelude::*, Rng};
use rand_pcg::Pcg64;
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

// fast mode but cost more memory
pub fn sample_bam_num(
    bamin: &Option<&str>,
    n: usize,
    seed: u64,
    out: &Option<&str>,
) -> Result<(), rust_htslib::errors::Error> {
    let mut rng = Pcg64::seed_from_u64(seed);
    let mut rec_bam: Vec<bam::Record> = Vec::with_capacity(n);

    let mut bam = bam_reader(bamin)?;
    for (order, rec) in bam.records().flatten().enumerate() {
        if order < n {
            rec_bam.push(rec);
        } else {
            let ret = rng.gen_range(0..=order);
            if ret < n {
                rec_bam[ret] = rec;
            }
        }
    }

    let headers = bam::Header::from_template(bam.header());
    let mut fo = bam_writer(out, headers, bam::Format::Bam)?;
    for x in rec_bam {
        fo.write(&x)?;
    }
    Ok(())
}

// reduce much memory but cost more time
pub fn sample_bam2_num(
    bamin: &Option<&str>,
    n: usize,
    seed: u64,
    out: &Option<&str>,
) -> Result<(), rust_htslib::errors::Error> {
    let mut rng = Pcg64::seed_from_u64(seed);
    let mut rec_order: Vec<usize> = Vec::with_capacity(n);

    let mut bam = bam_reader(bamin)?;
    for (order, _) in bam.records().flatten().enumerate() {
        if order < n {
            rec_order.push(order);
        } else {
            let ret = rng.gen_range(0..=order);
            if ret < n {
                rec_order[ret] = order;
            }
        }
    }

    let mut bam2 = bam_reader(bamin)?;
    let headers = bam::Header::from_template(bam2.header());
    let mut fo = bam_writer(out, headers, bam::Format::Bam)?;
    for (order, rec) in bam2.records().flatten().enumerate() {
        if rec_order.contains(&order) {
            fo.write(&rec)?;
        }
    }
    Ok(())
}

pub fn sample_bam_rate(
    bamin: &Option<&str>,
    rate: f64,
    seed: u64,
    rdc: bool,
    out: &Option<&str>,
) -> Result<(), rust_htslib::errors::Error> {
    let mut bam = bam_reader(bamin)?;
    let mut count = 0usize;
    for _ in bam.records() {
        count += 1;
    }

    let save = (count as f64 * rate) as usize;
    if rdc {
        let _x = sample_bam2_num(bamin, save, seed, out);
    } else {
        let _x = sample_bam_num(bamin, save, seed, out);
    }
    Ok(())
}
