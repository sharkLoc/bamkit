use rand::{prelude::*, Rng};
use rand_pcg::Pcg64;
use rust_htslib::bam::{self, Read};
use std::process::exit;
use crate::utils::*;


#[allow(non_snake_case)]
pub fn view(
    bamin: Option<&str>,
    Header: bool,
    bamf: bool,
    bamo: Option<&str>,
) -> Result<(), rust_htslib::errors::Error> {

    let mut bam = bam_reader(bamin)?;
    let header = bam::Header::from_template(bam.header());

    if Header {
        bam_writer(bamo, header, bam::Format::Sam)?;
        exit(0);
    }

    let mut writer = if bamf {
        bam_writer(bamo, header, bam::Format::Bam)?
    } else {
        bam_writer(bamo, header, bam::Format::Sam)?
    };

    for rec in bam.records().map_while(Result::ok) {
        writer.write(&rec)?;
    }

    Ok(())
}



// fast mode but cost more memory
pub fn sample_bam_num(
    bamin: Option<&str>,
    n: usize,
    seed: u64,
    out: Option<&str>,
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
    bamin: Option<&str>,
    n: usize,
    seed: u64,
    out: Option<&str>,
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
    bamin: Option<&str>,
    rate: f64,
    seed: u64,
    rdc: bool,
    out: Option<&str>,
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