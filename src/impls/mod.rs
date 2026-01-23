use crate::TryExtendOne;

#[cfg(feature = "unsafe")]
mod r#unsafe;

#[cfg(feature = "arrayvec")]
mod arrayvec;

mod maps;
mod sets;

#[cfg(feature = "alloc")]
mod result;

#[cfg(feature = "tuple")]
mod tuples;

fn try_extend_basic<T, C, I>(map: &mut C, iter: &mut I) -> Result<(), C::Error>
where
    I: Iterator<Item = T>,
    C: TryExtendOne<Item = T>,
{
    iter.try_for_each(|item| map.try_extend_one(item))
}
