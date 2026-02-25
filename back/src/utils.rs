pub fn inspect_err<F, T, E>(f: F) -> impl FnMut(&Result<T, E>)
where
    F: Fn(&E),
{
    move |r| match r {
        Ok(_) => {}
        Err(e) => f(e),
    }
}
