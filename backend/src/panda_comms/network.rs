use p2panda_core::{Hash, PrivateKey};
use p2panda_net::{address_book::AddressBookError, AddressBook};
use thiserror::Error;

pub struct Network {}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error(transparent)]
    AddressBook(#[from] AddressBookError),
}

impl Network {
    pub async fn new(network_id: &Hash, private_key: &PrivateKey) -> Result<Self, NetworkError> {
        let address_book = AddressBook::builder().spawn().await?;

        Ok(Network {})
    }
}
