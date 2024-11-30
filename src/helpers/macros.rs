/// From https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer/57578431#57578431
/// This macro wraps an enum and adds a TryFrom implementation, which lets you take a u16
/// and get the resultant enum value for it.
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u16> for $name {
            type Error = crate::TokioLabjackError;

            fn try_from(v: u16) -> std::result::Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u16 => Ok($name::$vname),)*
                    _ => Err(crate::TokioLabjackError::UnknownStatusCode(v)),
                }
            }
        }
    }
}

pub(crate) use back_to_enum;
