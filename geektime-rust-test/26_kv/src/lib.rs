mod error;
pub mod pb;
mod service;
mod storage;

pub use error::KvError;
pub use pb::abi::*;
pub use service::*;
pub use storage::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
