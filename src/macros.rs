macro_rules! errno {
    ($ret_expr:expr) => {
        match $ret_expr {
            i if i == 0 => Ok(()),
            i if i < 0 => Err($crate::err::BlkidErr::IO(std::io::Error::from_raw_os_error(i))),
            _ => Err($crate::err::BlkidErr::PositiveReturnCode),
        }
    }
}
