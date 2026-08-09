
#[cfg(feature = "arduino-uno")]
macro_rules! with_pins
{
    ( $macro:ident ) =>
    {
        
    };
}

#[cfg(not(any(feature = "arduino-uno")))]
compile_error!("unknown board type");