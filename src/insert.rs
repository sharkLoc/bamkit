use plotly::{Histogram, Plot};
use rust_htslib::bam::Read;

use crate::utils::*;

pub fn insert_size(
    bamin: Option<&str>,
    max: i64,
    name: &str,
) -> Result<(), rust_htslib::errors::Error> {
    let mut bam = bam_reader(bamin)?;
    let mut size = Vec::<i64>::with_capacity(100000);
    for r in bam.records().flatten() {
        let n = r.insert_size();
        if n >= 0 && n <= max {
            size.push(n);
        }
    }
    basic_histogram(size, name);
    Ok(())
}

fn basic_histogram(
    data: Vec<i64>, 
    name: &str
) {
    let mut plot = Plot::new();
    let trace = Histogram::new(data).name("insert plot");
    plot.add_trace(trace);
    //plot.use_local_plotly();
    plot.write_html(format!("{}.html", name));
}
