extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Command, attributes(test_attr))]
pub fn derive_command_fn(_item: TokenStream) -> TokenStream {
    "fn command() { let meme = 21; println!(\"I purchased {} watermelons today\", meme); }"
        .parse()
        .unwrap()
}

#[proc_macro_derive(Event, attributes(test_attr))]
pub fn derive_event_fn(_item: TokenStream) -> TokenStream {
    "fn event() { let meme = 9999; println!(\"event {}\", meme); }"
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
