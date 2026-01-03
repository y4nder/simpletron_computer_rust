use core::fmt;

#[derive(Debug)]
pub enum SimpletronError {
    StoreDataError(String),
    InvalidAddressError(String),
}

impl fmt::Display for SimpletronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpletronError::StoreDataError(err) => write!(f, "{}", err),
            SimpletronError::InvalidAddressError(invalid_address) => {
                write!(f, "{} is an invalid address", invalid_address)
            }
        }
    }
}
