#[derive(Debug)]
pub enum LoaderError {
    OpenFailed,
    IncorrectHeader,
    UnsupportedFormat,
}
