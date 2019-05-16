#![feature(async_await)]

mod connection;

pub use connection::{Connection, Context};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
