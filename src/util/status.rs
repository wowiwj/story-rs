pub(crate) type Status = (u32, String);

lazy_static! {
    pub(crate) static ref OK: Status = (1000, String::from("success"));
    pub(crate) static ref SYS_ERROR: Status = (1010, String::from("system error"));
    pub(crate) static ref BAD_REQUEST: Status = (1011, String::from("bad request"));
    pub(crate) static ref UNAUTH: Status = (1012, String::from("Unauthorized"));
    pub(crate) static ref TIME_OUT: Status = (1013, String::from("Time out"));
    pub(crate) static ref UNKNOWN: Status = (1020, String::from("UNKNOWN"));
}