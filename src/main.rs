use clap::Parser;

mod flag;
mod insert;
mod utils;
mod view;

use flag::*;
use insert::*;
use view::*;

#[derive(Parser, Debug)]
#[command(
    author = "size_t",
    version = "version 0.1.0",
    about = "bamkit: a simple program for bam file manipulation",
    long_about = None
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

        /// show sam file header only
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
}

fn main() {
    let arg = Args::parse();
    match arg.command {
        Subcli::view {
            input,
            Header,
            bam,
            out,
        } => {
            if input.is_some() {
                if out.is_some() {
                    let _x = view(
                        &Some(input.unwrap().as_str()),
                        Header,
                        bam,
                        &Some(out.unwrap().as_str()),
                    );
                } else {
                    let _x = view(&Some(input.unwrap().as_str()), Header, bam, &None);
                }
            } else {
                if out.is_some() {
                    let _x = view(&None, Header, bam, &Some(out.unwrap().as_str()));
                } else {
                    let _x = view(&None, Header, bam, /* sam,*/ &None);
                }
            }
        }
        Subcli::region { bam, reg, sam, out } => {
            if out.is_some() {
                let _x = target(&bam, &reg, sam, &Some(out.unwrap().as_str()));
            } else {
                let _x = target(&bam, &reg, sam, &None);
            }
        }
        Subcli::flags { flag } => {
            show_flag(flag);
        }
        Subcli::insert { bam, max, name } => {
            if bam.is_some() {
                let _x = insert_size(&Some(bam.unwrap().as_str()), max, &name);
            } else {
                let _x = insert_size(&None, max, &name);
            }
        }
    }
}
