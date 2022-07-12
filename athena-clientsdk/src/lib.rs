use lazy_static::lazy_static;

mod types;
mod client;


lazy_static! {
 pub static ref ATHENA_CLIENT: client::AthenaClient = client::AthenaClient::init();
}