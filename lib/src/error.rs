use minidom::error::Error as MiniError;
use reqwest::Error as ReqError;
use std::fmt::Error as FmtError;

#[derive(Debug)]
pub enum Error {
    FmtError(FmtError),
    ReqwestError(ReqError),
    MiniDomError(MiniError),
    GenericError(String),
}

impl From<ReqError> for Error {
    fn from(e: ReqError) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<FmtError> for Error {
    fn from(e: FmtError) -> Self {
        Self::FmtError(e)
    }
}

impl From<MiniError> for Error {
    fn from(e: MiniError) -> Self {
        Self::MiniDomError(e)
    }
}
