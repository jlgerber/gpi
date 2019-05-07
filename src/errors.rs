use failure::Fail;

#[derive(Debug, Fail)]
pub enum GpiError {
    #[fail(display = "UnknownVcs: {}", _0)]
    UnknownVcs(String)
}
