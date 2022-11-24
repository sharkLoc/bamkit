use rust_htslib::bam;

pub fn bam_reader(bamf: &Option<&str>) -> Result<bam::Reader, rust_htslib::errors::Error> {
    if bamf.is_some() {
        bam::Reader::from_path(bamf.unwrap())
    } else {
        bam::Reader::from_stdin()
    }
}

pub fn bam_writer(
    bamf: &Option<&str>,
    header: bam::Header,
    format: bam::Format,
) -> Result<bam::Writer, rust_htslib::errors::Error> {
    if bamf.is_some() {
        bam::Writer::from_path(bamf.unwrap(), &header, format)
    } else {
        bam::Writer::from_stdout(&header, format)
    }
}
