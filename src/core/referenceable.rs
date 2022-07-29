use super::ReferenceTo;

pub trait Referenceable: Clone {
    fn get_reference(&self) -> Option<ReferenceTo<Self>>
    where
        Self: Sized;

    fn get_ref_type() -> &'static str;
}
