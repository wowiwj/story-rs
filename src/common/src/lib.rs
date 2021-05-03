#[macro_use]
extern crate lazy_static;

pub mod state;
pub mod setting;
pub mod format;
pub mod api;
pub mod jwt;
pub mod hash;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
