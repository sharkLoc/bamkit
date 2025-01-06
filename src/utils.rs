use std::io::{self,BufRead,BufReader,BufWriter,Error, Write};
use std::fs::File;
use rust_htslib::{bam, errors::Error as Htserror};


#[allow(dead_code)]
pub fn file_reader(
    src: Option<&str>,
) -> Result<Box<dyn BufRead>, Error> {
    let reader: Box<dyn BufRead> = if let Some(path) = src {
        Box::new( File::open(path).map(BufReader::new)?)
    } else {
        Box::new( BufReader::new(io::stdin()) )
    };
    Ok(reader)
}

#[allow(dead_code)]
pub fn file_writer(
    src: Option<&str>,
) -> Result<Box<dyn Write>, Error> {
    let writer: Box<dyn Write>  = if let Some(path) = src {
        Box::new(File::create(path).map(BufWriter::new)?)
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };
    Ok(writer)
}


pub fn bam_reader(
    bamf: Option<&str>,
) -> Result<bam::Reader, Htserror> {
    let reader = if let Some(path) = bamf {
        bam::Reader::from_path(path)?
    } else {
        bam::Reader::from_stdin()?
    };
    Ok(reader) 
}

pub fn bam_writer(
    bamf: Option<&str>,
    header: bam::Header,
    format: bam::Format,
) -> Result<bam::Writer, Htserror> {
    let writer = if let Some(path) = bamf {
        bam::Writer::from_path(path,&header, format)?
    } else {
        bam::Writer::from_stdout(&header, format)?
    };
    Ok(writer)
}