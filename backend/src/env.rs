const IS_PRODUCTION_STR: &[u8] = std::env!("IS_PRODUCTION").as_bytes();
const COMPARE_FOR_FALSE: &[u8] = "false".as_bytes();

pub const IS_PRODUCTION: bool = !compile_time_compare(IS_PRODUCTION_STR, COMPARE_FOR_FALSE);
pub const STORAGE_BUCKET_NAME: &str = std::env!("STORAGE_BUCKET_NAME");

const fn compile_time_compare(a: &[u8], b: &[u8]) -> bool{
    if a.len() != b.len() {
        return false;
    }
    let mut i: usize = 0;
    while i < a.len(){
        if a[i] != b[i]{
            return false;
        }
        i += 1;
    }
    return true;
}
