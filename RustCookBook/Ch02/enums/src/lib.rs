// Going further with Advanced Rust
// Creating meaningful numbers with enums
use std::fmt::format;
use std::fs::write;
use std::io;

pub enum ApplicationError{
    Code {full : usize, short : u16},
    Message(String),
    IOWrapper(io::Error),
    Unknown,
}

impl ApplicationError {

    pub fn print_kid(&self, mut to:&mut impl io::Write) ->
    io::Result<()>{
        let kind  = match self {
            ApplicationError::Code {full: _, short: _} => "Code",
            ApplicationError::IOWrapper(_) => "IOWrapper",
            ApplicationError::Message(_) => "Message",
            ApplicationError::Unknown => "Unknown",
        };
        write!(&mut to, "{}", kind)?;
        Ok(())
    }

}

pub fn do_work(choice: i32) -> Result<(), ApplicationError>{
    if choice < -100{
        Err(ApplicationError::IOWrapper(io::Error::from(io::ErrorKind::Other)))
    } else if choice == 42{
        Err(ApplicationError::Code { full: choice as usize, short:(choice % u16::MAX as i32) as u16})
    } else if choice > 42{
        Err(ApplicationError::Message(
            format!("{} lead to terrible error", choice)
        ))
    } else {
       Err(ApplicationError::Unknown)
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::format;
    use super::{ApplicationError, do_work};
    use std::io;

    #[test]
    fn test_do_work() {
        let choice = 10;
        if let Err(error) = do_work(choice){
            match error {
                ApplicationError::Code { full: code, short:_} => assert_eq!(choice as usize, code),
                ApplicationError::Unknown |
                ApplicationError::IOWrapper(_) => assert!(choice < 43),
                ApplicationError::Message(msg) => assert_eq!(format!("{} lead to a terrible error",choice), msg)

            }
        }
    }

    #[test]
    fn test_application_error_get_kid(){
        let mut target = vec![];
        let _= ApplicationError::Code { full: 100, short: 100}.print_kid(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Code".to_string());

        let mut target = vec![];
        let _= ApplicationError::Message("0".to_string()).print_kid(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Message".to_string());

        let mut target = vec![];
        let _= ApplicationError::Unknown.print_kid(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "Unknown".to_string());

        let mut target = vec![];
        let error = io::Error::from(io::ErrorKind::WriteZero);
        let _= ApplicationError::IOWrapper(error).print_kid(&mut target);
        assert_eq!(String::from_utf8(target).unwrap(), "IOWrapper".to_string());
    }
}


