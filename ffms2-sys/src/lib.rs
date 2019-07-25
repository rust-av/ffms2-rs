#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/ffms2.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version() {
        let version = unsafe { FFMS_GetVersion() };
        println!("ffms2 version {}", version);
    }
}
