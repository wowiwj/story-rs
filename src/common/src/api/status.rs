pub type Status = (u32, String);

lazy_static! {
    pub static ref OK: Status = (1000, String::from("success"));
    pub static ref SYS_ERROR: Status = (1010, String::from("system error"));
    pub static ref BAD_REQUEST: Status = (1011, String::from("bad request"));
    pub static ref UNAUTH: Status = (1012, String::from("Unauthorized"));
    pub static ref TIME_OUT: Status = (1013, String::from("Time out"));
    pub static ref UNKNOWN: Status = (1020, String::from("UNKNOWN"));
}