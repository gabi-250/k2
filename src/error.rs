/// An error describing why an experiment failed.
#[derive(Debug)]
pub enum K2Error {
    Unknown,
    ExecutionFailed,
    RerunError,
}
