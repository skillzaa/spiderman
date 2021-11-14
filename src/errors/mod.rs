use thiserror::Error;


#[derive(Error, Debug)]
pub enum SpiderErrors {
    #[error("the trigger handler function provided by the user has returned false")]
    UserReturnedFalse,

}




// #[error("Read error")]
// ReadError { source: std::io::Error },

// /// Represents all other cases of `std::io::Error`.
// #[error(transparent)]
// IOError(#[from] std::io::Error),