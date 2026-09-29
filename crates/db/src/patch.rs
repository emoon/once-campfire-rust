/// An update to a nullable attribute: leave it alone, set it, or set it to `NULL`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Patch<T> {
    #[default]
    Keep,
    Set(T),
    Clear,
}

impl<T> Patch<T> {
    /// Applies the patch to `attribute`; true when that changed it.
    pub fn apply(self, attribute: &mut Option<T>) -> bool
    where
        T: PartialEq,
    {
        let value = match self {
            Patch::Keep => return false,
            Patch::Set(value) => Some(value),
            Patch::Clear => None,
        };
        if *attribute == value {
            return false;
        }
        *attribute = value;
        true
    }

    /// The value it sets, or `None` when it keeps or clears the attribute.
    pub fn into_value(self) -> Option<T> {
        match self {
            Patch::Set(value) => Some(value),
            Patch::Keep | Patch::Clear => None,
        }
    }
}
