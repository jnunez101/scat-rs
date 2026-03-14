use crate::ScatParser;

#[path="./byte_parsers/diagheader.rs"]
mod diagheader;

pub struct QualcommDiagParser {

}
trait Parse {
    fn Parse(buf: &[u8]) -> Self;
}

impl ScatParser for QualcommDiagParser {

}
