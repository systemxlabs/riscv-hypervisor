#[derive(Debug)]
pub enum HypervisorError {
    InvalidParam,
    NoMemory,
}

pub type HypervisorResult<T> = Result<T, HypervisorError>;
