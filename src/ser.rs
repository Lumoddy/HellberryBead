
pub trait Serializer
{
    type Error;

    fn u8(&mut self, value: u8) -> Result<(), Self::Error>;

    fn u16(&mut self, value: u16) -> Result<(), Self::Error>;

    fn len(&mut self, value: u16) -> Result<(), Self::Error> { self.u16(value) }

    fn enum_tag(&mut self, value: u8) -> Result<(), Self::Error> { self.u8(value) }

    fn value<T>(&mut self, value: T) -> Result<(), Self::Error>
    where
        T: Serialize { value.serialize(self) }

    fn str(&mut self, iter: &str) -> Result<(), Self::Error>
    {
        let iter = iter.bytes();
        self.u16(iter.len() as _);
        for x in iter { self.u8(x)? }
        Ok(())
    }

    fn array<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
        I::Item: Serialize
    {
        let iter = iter.into_iter();
        self.len(iter.len() as _);
        for x in iter { x.serialize(self)? }
        Ok(())
    }
}

pub trait Serialize
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized;
}

impl Serialize for &u8
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.u8(*self) }
}

impl Serialize for u8
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &u16
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.u16(*self) }
}

impl Serialize for u16
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(&self) }
}

impl Serialize for &str
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.str(self) }
}

impl<'a, T: ?Sized> Serialize for &&'a T
where
    &'a T: Serialize,
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.value(*self) }
}

impl<'a, T> Serialize for &'a [T]
where
    &'a T: Serialize,
{
    fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
    where
        S: Serializer + ?Sized { ser.array(self) }
}

macro_rules! tuple_impl
{
    ( $( ( $( $field:tt : $field_ty:ident ),*$(,)? ) ),*$(,)? ) =>
    {
        $(
            impl< $( $field_ty , )* > Serialize for ( $( $field_ty , )* )
            where
                $( $field_ty : Serialize, )*
            {
                #[allow(unused_variables)]
                fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
                where
                    S: Serializer + ?Sized
                {
                    $( ser.value(self. $field )?; )*
                    Ok(())
                }
            }

            impl<'a, $( $field_ty , )* > Serialize for &'a ( $( $field_ty , )* )
            where
                $( &'a $field_ty : Serialize, )*
            {
                #[allow(unused_variables)]
                fn serialize<S>(self, ser: &mut S) -> Result<(), S::Error>
                where
                    S: Serializer + ?Sized
                {
                    $( ser.value(&self. $field )?; )*
                    Ok(())
                }
            }
        )*
    };
}

tuple_impl!
{
    (),
    (0: T0),
    (0: T0, 1: T1),
    (0: T0, 1: T1, 2: T2),
    (0: T0, 1: T1, 2: T2, 3: T3),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7, 8: T8),
}