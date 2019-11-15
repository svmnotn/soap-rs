use std::fmt::{Error as FmtError, Write};

pub trait DisplayAction {
    fn fmt(&self, fmt: &mut String, action: &str) -> Result<(), FmtError>;
}

impl DisplayAction for () {
    fn fmt(&self, _: &mut String, _: &str) -> Result<(), FmtError> {
        Ok(())
    }
}

impl<T: DisplayAction> DisplayAction for Option<T> {
    fn fmt(&self, fmt: &mut String, action: &str) -> Result<(), FmtError> {
        if let Some(ref v) = self {
            v.fmt(fmt, action)
        } else {
            Ok(())
        }
    }
}

macro_rules! impl_display {
    ($($t:ty),*) => {
        $(
            impl DisplayAction for $t {
                fn fmt(&self, fmt: &mut String, action: &str) -> Result<(), FmtError> {
                    write!(fmt, "<{act}>{data}</{act}>", act = action, data = self)
                }
            }
        )*
    };
}

impl_display! {
    char, &str, String,
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64
}
