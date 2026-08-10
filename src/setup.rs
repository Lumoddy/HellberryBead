
#[cfg(feature = "arduino-uno")]
/// Calls the given macro with a description of the pwm timers on the device in
/// the form:
/// ```
/// # macro_rules! x
/// # {
/// #     (
/// $( $index:literal , )*
/// #     )
/// #     => { };
/// # }
/// ```
macro_rules! with_pwm_timers
{
    ( $macro:ident ) =>
    {
        $macro! { 0, 1, 2, }
    };
}

#[cfg(feature = "arduino-uno")]
/// Calls the given macro with a description of the pins on the device in the
/// form:
/// ```
/// # macro_rules! x
/// # {
/// #     (
/// $(
///     $name:ident : $port:ident =
///     {
///         digital_in: $digital_in:literal , // `true` or `false`.
///         digital_out: $digital_out:literal , // `true` or `false`.
///         analog_in: $analog_in:literal , // `true` or `false`.
///         analog_out: $analog_out:literal , // `true` or `false`.
///         $(analog_timer: $analog_timer_index:literal , )? // integer literal as the index of the timer if `analog_out` is `true`.
///     } ,
/// )*
/// #     )
/// #     => { };
/// # }
/// ```
macro_rules! with_pins
{
    ( $macro:ident ) =>
    {
        $macro!
        {
            a0: PC0 = { display: "A0", digital_in: true, digital_out: true, analog_in: true, analog_out: false, },
            a1: PC1 = { display: "A1", digital_in: true, digital_out: true, analog_in: true, analog_out: false, },
            a2: PC2 = { display: "A2", digital_in: true, digital_out: true, analog_in: true, analog_out: false, },
            a3: PC3 = { display: "A3", digital_in: true, digital_out: true, analog_in: true, analog_out: false, },
            a4: PC4 = { display: "A4", digital_in: true, digital_out: true, analog_in: true, analog_out: false, },
            a5: PC5 = { display: "A5", digital_in: true, digital_out: true, analog_in: true, analog_out: false, },

            // d0: PD0 = { display: "0", digital_in: true, digital_out: true, analog_in: false, analog_out: false },
            // d1: PD1 = { display: "1", digital_in: true, digital_out: true, analog_in: false, analog_out: false },
            d2: PD2 = { display: "2", digital_in: true, digital_out: true, analog_in: false, analog_out: false, },
            d3: PD3 = { display: "3", digital_in: true, digital_out: true, analog_in: false, analog_out: true, analog_timer: 2, },
            d4: PD4 = { display: "4", digital_in: true, digital_out: true, analog_in: false, analog_out: false, },
            d5: PD5 = { display: "5", digital_in: true, digital_out: true, analog_in: false, analog_out: true, analog_timer: 0, },
            d6: PD6 = { display: "6", digital_in: true, digital_out: true, analog_in: false, analog_out: true, analog_timer: 0, },
            d7: PD7 = { display: "7", digital_in: true, digital_out: true, analog_in: false, analog_out: false, },

            d8: PB0 = { display: "8", digital_in: true, digital_out: true, analog_in: false, analog_out: false, },
            d9: PB1 = { display: "9", digital_in: true, digital_out: true, analog_in: false, analog_out: true, analog_timer: 1, },
            d10: PB2 = { display: "10", digital_in: true, digital_out: true, analog_in: false, analog_out: true, analog_timer: 1, },
            d11: PB3 = { display: "11", digital_in: true, digital_out: true, analog_in: false, analog_out: true, analog_timer: 2, },
            d12: PB4 = { display: "12", digital_in: true, digital_out: true, analog_in: false, analog_out: false, },
            d13: PB5 = { display: "13", digital_in: true, digital_out: true, analog_in: false, analog_out: false, },
        }
    };
}

#[cfg(feature = "arduino-uno")]
pub const BOARD_NAME: &str = "arduino-uno";

#[cfg(not(any(feature = "arduino-uno")))]
compile_error!("unknown board type");

pub(crate) use with_pwm_timers;
pub(crate) use with_pins;