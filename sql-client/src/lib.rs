#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod common;
mod sql_connection_string;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
