// TODO implement macro for default of a value;
// TODO can also try to call associated functions directly from instances?
fn default_of_helper<T>(_: &T) -> T
where
    T: Default,
{
    T::default()
}

#[macro_export]
macro_rules! default_of {
    ($a:expr) => {
        default_of_helper(&$a)
    };
}
