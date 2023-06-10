use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid Notion API Token: ")]
    InvalidApiToken,
}
