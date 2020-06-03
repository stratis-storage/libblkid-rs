macro_rules! str_ptr_to_owned {
    ($str_ptr:expr) => {
        unsafe { std::ffi::CStr::from_ptr($str_ptr) }
            .to_str()?
            .to_string()
    };
}

macro_rules! str_ptr_with_size_to_owned {
    ($str_ptr:expr, $size:expr) => {
        std::ffi::CStr::from_bytes_with_nul(unsafe {
            std::slice::from_raw_parts($str_ptr as *const u8, $size)
        })?
        .to_str()?
        .to_string()
    };
}

macro_rules! errno {
    ($ret_expr:expr) => {
        match $ret_expr {
            i if i == 0 => Ok(()),
            i if i < 0 => Err($crate::err::BlkidErr::LibErr),
            _ => Err($crate::err::BlkidErr::PositiveReturnCode),
        }
    };
}

macro_rules! errno_ptr {
    ($ret_expr:expr) => {{
        let ptr = $ret_expr;
        if ptr.is_null() {
            Err($crate::err::BlkidErr::LibErr)
        } else {
            Ok(ptr)
        }
    }};
}

macro_rules! option_ptr {
    ($ret_expr:expr) => {{
        let ptr = $ret_expr;
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }};
}

macro_rules! errno_with_ret {
    ($ret_expr:expr) => {
        match $ret_expr {
            i if i < 0 => Err($crate::err::BlkidErr::LibErr),
            i => Ok(i),
        }
    };
}

macro_rules! consts_enum_conv {
    ($(#[$enum_meta:meta])* $enum_ident:ident <=> $conv_type:ty, $($(#[$var_meta:meta])* $variant:ident => $const:expr),+) => {
        $(
            #[$enum_meta]
        )*
        #[derive(Hash, PartialEq, Eq)]
        pub enum $enum_ident {
            $(
                $(
                    #[$var_meta]
                )*
                $variant,
            )+
        }

        impl Into<$conv_type> for $enum_ident {
            fn into(self) -> $conv_type {
                match self {
                    $(
                        $enum_ident::$variant => $const,
                    )+
                }
            }
        }

        impl std::convert::TryFrom<$conv_type> for $enum_ident {
            type Error = $crate::err::BlkidErr;

            fn try_from(v: $conv_type) -> $crate::err::Result<Self> {
                match v {
                    $(
                        i if i == $const => Ok($enum_ident::$variant),
                    )+
                    _ => Err($crate::err::BlkidErr::InvalidConv),
                }
            }
        }
    }
}

macro_rules! flags {
    ($(#[$meta:meta])* $flag_set_name:ident <=> $converted_flag_type:ty, $enum_name:ty) => {
        $(
            #[$meta]
        )*
        pub struct $flag_set_name(std::collections::HashSet<$enum_name>);

        impl $flag_set_name {
            /// Create an empty flag set
            pub fn empty() -> Self {
                $flag_set_name(std::collections::HashSet::new())
            }

            /// Create a flag set initialized with the given vector
            pub fn new(vec: Vec<$enum_name>) -> Self {
                $flag_set_name(vec.into_iter().collect())
            }
        }

        impl std::convert::TryFrom<$converted_flag_type> for $flag_set_name {
            type Error = $crate::err::BlkidErr;

            fn try_from(v: $converted_flag_type) -> Result<Self, Self::Error> {
                const BITS_IN_A_BYTE: usize = 8;

                let mut vec = Vec::new();
                for i in 0..std::mem::size_of::<$converted_flag_type>() * BITS_IN_A_BYTE {
                    vec.push(<$enum_name>::try_from((1 << i) & v)?);
                }
                Ok(<$flag_set_name>::new(vec))
            }
        }

        impl Into<$converted_flag_type> for $flag_set_name {
            fn into(self) -> $converted_flag_type {
                self.0.into_iter().fold(0, |acc, next| {
                    let converted: $converted_flag_type = next.into();
                    acc | converted
                })
            }
        }
    }
}
