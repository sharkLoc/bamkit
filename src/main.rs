use clap::Parser;
use std::time::Instant;
use log::{info,error};

mod flag;
mod insert;
mod utils;
mod view;
mod region;

use flag::*;
use insert::*;
use view::*;
use region::*;

#[derive(Parser, Debug)]
#[command(
    author = "size_t",
    version = "version 0.3.0",
    about = "bamkit: a simple program for bam file manipulation",
    long_about = None,
    disable_help_flag = false,
    disable_version_flag = true,
    propagate_version = true,
)]
struct Args {
    #[clap(subcommand)]
    command: Subcli,
}

#[derive(Parser, Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
enum Subcli {
    /// sam bam conversion
    view {
        /// input bam[sam] file.
        input: Option<String>,

        /// show header and exit
        #[arg(short = 'H', long = "Header")]
        Header: bool,

        /// output is bam
        #[arg(short = 'b', long = "bam")]
        bam: bool,

        /// output file name or write to stdout
        #[arg(short = 'o', long = "out")]
        out: Option<String>,
    },
    /// get target region from bam file
    region {
        /// input sorted and indexed bam file.
        #[arg(short = 'b', long = "bam")]
        bam: String,

        /// bam[sam] file target postion, eg, chr1:100-300
        reg: String,

        /// output is sam
        #[arg(short = 's', long = "sam")]
        sam: bool,

        /// output file name or write to stdout, default bam format
        #[arg(short = 'o', long = "out")]
        out: Option<String>,
    },
    /// bam file flag value show
    flags {
        /// specify bam[sam] flag value
        flag: u64,
    },
    /// insert size plot for bam file
    insert {
        /// input bam[sam] file.
        bam: Option<String>,

        /// max insert szie length
        #[arg(short = 'm', long = "max", default_value_t = 1000)]
        max: i64,

        /// the html format plot file name
        #[arg(short = 'n', long = "name")]
        name: String,
    },
    /// sample sequences by number or fraktion
    sample {
        /// input bam[sam] file.
        bam: Option<String>,

        /// set rand seed.
        #[arg(short = 's', long = "seed", default_value_t = 69)]
        seed: u64,

        /// sample by number, use with -r on large bam file
        #[arg(short = 'n', long = "num")]
        num: Option<usize>,

        /// sample by fraktion, use with -r on large bam file
        #[arg(short = 'f', long = "frak")]
        frak: Option<f64>,

        /// reduce much memory but cost more time
        #[arg(short = 'r', long = "rdc")]
        rdc: bool,

        /// output bam file name or write to stdout
        #[arg(short = 'o', long = "out")]
        out: Option<String>,
    },
}

fn main() -> Result<(), rust_htslib::errors::Error> {
    let arg = Args::parse();
    
    let start = Instant::now();

    match arg.command {
        Subcli::view {
            input,
            Header,
            bam,
            out,
        } => {  
            if let Some(input) = input {
                if let Some(out) = out {
                    view(Some(&input), Header, bam, Some(&out))?;
                } else {
                    view(Some(&input), Header, bam, None)?;
                }
            } else {
                if let Some(out) = out {
                    view(None, Header, bam, Some(&out))?;
                } else {
                    view(None, Header, bam, None)?;
                }
            }
        }
        Subcli::region { bam, reg, sam, out } => {
            if let Some(out) = out {
                targets(&bam, &reg, sam, Some(&out))?;
            } else {
                targets(&bam, &reg, sam, None)?;
            }
        }
        Subcli::flags { flag } => {
            show_flag(flag);
        }
        Subcli::insert { bam, max, name } => {
            if let Some(bam) = bam {
                insert_size(Some(&bam), max, &name)?;   
            } else {
                insert_size(None, max, &name)?;
            }
        }
        Subcli::sample {
            bam,
            seed,
            num,
            frak,
            rdc,
            out,
        } => {
            if let Some(frak) = frak {
                if num.is_some() {
                    //eprintln!("{}", "[error]: opt -n can't use with -f.".red());
                    error!("opt -n can't use with -f.");
                    std::process::exit(1);
                } else {
                    if let Some(bam) = bam {
                        if let Some(out) = out {
                            sample_bam_rate(Some(&bam), frak, seed, rdc, Some(&out))?;
                        } else {
                            sample_bam_rate(Some(&bam), frak, seed, rdc, None)?;
                        }
                    } else {
                        if let Some(out) = out {
                            sample_bam_rate(None, frak, seed, rdc, Some(&out))?;
                        } else {
                            sample_bam_rate(None, frak, seed, rdc, None)?;
                        }
                    }
                }
            } else {
                if num.is_none() {
                    //eprintln!("{}", "[error]: one of option -n or -f must be used.".red());
                    error!("one of option -n or -f must be used.");
                    std::process::exit(1);
                }
                if let Some(bam) = bam {
                    if let Some(out) = out {
                        if rdc {
                            sample_bam2_num(Some(&bam), num.unwrap(), seed, Some(&out))?;
                        } else {
                            sample_bam_num(Some(&bam), num.unwrap(), seed, Some(&out))?;
                        };
                    } else {
                        if rdc {
                            sample_bam2_num(Some(&bam), num.unwrap(), seed, None)?;
                        } else {
                            sample_bam_num(Some(&bam), num.unwrap(), seed, None)?;
                        };
                    }
                } else {
                    if let Some(out) = out {
                        if rdc {
                            sample_bam2_num(None, num.unwrap(), seed, Some(&out))?;
                        } else {
                            sample_bam_num(None, num.unwrap(), seed, Some(&out))?;
                        };
                    } else {
                        if rdc {
                            sample_bam2_num(None, num.unwrap(), seed, None)?;
                        } else {
                            sample_bam_num(None, num.unwrap(), seed, None)?;
                        };
                    }
                }
            }
        }
    }

    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
