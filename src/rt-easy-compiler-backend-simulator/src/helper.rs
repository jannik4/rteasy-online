use crate::{Generate, Result};

impl<I, T> Generate<Option<I>> for Option<T>
where
    T: Generate<I>,
{
    fn generate(input: Option<I>) -> Result<Self> {
        match input {
            Some(input) => Ok(Some(Generate::generate(input)?)),
            None => Ok(None),
        }
    }
}

impl<I, T> Generate<Vec<I>> for Vec<T>
where
    T: Generate<I>,
{
    fn generate(input: Vec<I>) -> Result<Self> {
        input.into_iter().map(|input| T::generate(input)).collect()
    }
}
