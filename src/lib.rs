#![feature(async_await)]

extern crate ferriscord_derive;

mod connection;
mod discord_datatypes;

pub use connection::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
