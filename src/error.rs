#[derive(Debug)]
pub enum HypervisorError {
    InvalidParam,
    NoMemory,
    NotMapped,
    AlreadyMapped,
}

pub type HypervisorResult<T> = Result<T, HypervisorError>;
