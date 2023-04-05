use libc_print::{libc_println};

macro_rules! enum_str {
    (pub enum $name:ident {
        $($varient:ident = $val:expr),*,
    }) => {
        pub enum $name {
            $($varient = $val),*
        }
        impl $name {
            pub fn into_string(&self) -> &'static str {
                match self {
                    $($name::$varient => stringify!($varient)),*
                }
            }
            
        }
    };
}

pub struct StatusType;
struct StatusColors<'a> {
    color_green: &'a str,
    color_yellow: &'a str,
    color_cyan: &'a str,
    color_blue: &'a str,
    color_magenta: &'a str,
    color_reset: &'a str
}

pub enum StatusEnum {
    IsInfo,
    IsWarning,
    IsError,
    IsAOk,
    IsDebug
}
enum_str! {
    pub enum StatusStringTypes {
        Info = 0x00,
        Warning = 0x01,
        Error = 0x02,
        Success = 0x03,
        Debug = 0x04,
        Unknown = 0x05,
    }
}
impl StatusType {
    
    fn init_colors() -> StatusColors<'static>{
        StatusColors{
            color_blue: "\x1b[34m",
            color_green: "\x1b[32m",
            color_yellow: "\x1b[33m",
            color_cyan: "\x1b[36m",
            color_magenta: "\x1b[35m",
            color_reset: "\x1b[0m"
        }
    }

    fn format_log(token:&str, color:&str, _type_:&str) -> String {
        let full_log = "[".to_owned() + color + _type_ + "\x1b[0m" + "] " + token;
        full_log
    }
    
}

pub fn log_(msg: &str, status: &'static str) -> () {
    let stat_cols = StatusType::init_colors();

    match status {
        "Warning" => {libc_println!("{}", StatusType::format_log(msg, stat_cols.color_yellow, status))},
        "Error" => {libc_println!("{}", StatusType::format_log(msg, stat_cols.color_magenta, status))},
        "Info" => {libc_println!("{}", StatusType::format_log(msg, stat_cols.color_cyan, status))},
        "Success" => {libc_println!("{}", StatusType::format_log(msg, stat_cols.color_green, status))},
        "Debug" => {libc_println!("{}", StatusType::format_log(msg, stat_cols.color_blue, status))}
        _=> {libc_println!("{}", StatusType::format_log(msg, stat_cols.color_reset, status))}
    }
    
}