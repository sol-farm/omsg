//! omsg stands for optimized-msg and is a crate of helper functions to enable maximal 
//! efficiency of compute units consumption from message logging in solana programs.
//! while working on tulip one of the biggest issues we had with compute unit consumption
//! was logging messages which made use of string formatting. this is because string formatting
//! requires heap allocations. when using stackbased formatting as provided by arrform, rough
//! estimates seem to indicate that each log msg which includes string formatting using `omsg`
//! should save around ~200 compute units.

pub mod arrform;
pub use arrform::ArrForm;

#[macro_export]
macro_rules! sum {
    // this delcares an exrpession i think :shrug:
    // todo(): explain this more
    ($($args:expr),*) => {{
        let result = 0;
        $(
            // combine the size of each value 
            let result = result + std::mem::size_of_val(&$args);
        )*
        // return the size of all arguments
        result
    }}
}

/// an optimized form of the `msg!` macro, which attempts to utilizes stack based formatting
/// of strings instead of heap based formatting where possible, attempting to optimize the stack
/// that is used. in the even of a message requiring larger than 768 stack bytes, regular msg formatting is used
#[macro_export]
macro_rules! omsg {
    ($($args:tt)+) => {
        let input_sizes = sum!($($args)*);
        match input_sizes {
            s if s <= 768 && s > 512 => msg!("{}", arrform!(768, $($args)*).as_str()),
            s if s <= 512 && s > 256 => msg!("{}", arrform!(512, $($args)*).as_str()),
            s if s <= 256 && s > 128 => msg!("{}", arrform!(256, $($args)*).as_str()),
            s if s <= 128 && s > 64 => msg!("{}", arrform!(128, $($args)*).as_str()),
            s if s <= 64 && s > 32 => msg!("{}", arrform!(64, $($args)*).as_str()),
            s if s <= 32 && s > 0 => msg!("{}", arrform!(32, $($args)*).as_str()),
            _ => msg!("{}", format!($($args)*)),
        }
    };
}

/// similar to `omsg!` except it adds tracing information (file and line number). if the combined file and line number
/// results in a byte size > 128, this will cause a run time error
#[macro_export]
macro_rules! omsg_trace {
    ($($args:tt)+) => {
        let file_name = std::path::Path::new(file!()).file_name().unwrap().to_string_lossy();
        let file_info = arrform!(128, "{}:{}", file_name, line!());
        let input_sizes = sum!($($args)*);
        match input_sizes  {
            s if s <= 768 && s > 512 => msg!("[{}] {}", file_info.as_str(), arrform!(768, $($args)*).as_str()),
            s if s <= 512 && s > 256 => msg!("[{}] {}", file_info.as_str(), arrform!(512, $($args)*).as_str()),
            s if s <= 256 && s > 128 => msg!("[{}] {}", file_info.as_str(), arrform!(256, $($args)*).as_str()),
            s if s <= 128 && s > 64 => msg!("[{}] {}", file_info.as_str(),  arrform!(128, $($args)*).as_str()),
            s if s <= 64 && s > 32 => msg!("[{}] {}",  file_info.as_str(), arrform!(64, $($args)*).as_str()),
            s if s <= 32 && s > 0 => msg!("[{}] {}", file_info.as_str(), arrform!(32, $($args)*).as_str()),
            _ => msg!("[{}] {}", file_info.as_str(),  format!($($args)*)),
        }
    };
}


#[cfg(test)]
mod test {
    use super::*;
    use solana_program::msg;
    #[test]
    fn test_omsg() {
        omsg!("abc too {}", "yooo");
        omsg_trace!("abc too {}", "yoooo");
    }
    #[test]
    fn test_size_ofs() {
        println!("{}", sum!("y", "o", "bbbbbb"));
    }
}