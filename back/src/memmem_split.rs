/// Split the string [`s`] by [`delim`], and return an iterator that yields
/// [`std::ops::Range<usize>`] that slices the individual pieces of [`s`].
///
/// For exampe:
/// ```
/// let s = "v1+V+IV+Ind+Prs+Sg2";
/// let mut it = memmem_split("+", s);
/// assert_eq!(&s[it.next().unwrap()], "v1");
/// assert_eq!(&s[it.next().unwrap()], "V");
/// assert_eq!(&s[it.next().unwrap()], "IV");
/// assert_eq!(&s[it.next().unwrap()], "Ind");
/// assert_eq!(&s[it.next().unwrap()], "Prs");
/// assert_eq!(&s[it.next().unwrap()], "Sg2");
/// assert_eq!(it.next(), None);
/// ```
pub fn memmem_split<'a>(
    delim: &'a str,
    s: &'a str,
) -> impl Iterator<Item = std::ops::Range<usize>> {
    let finder = memchr::memmem::Finder::new(delim).into_owned();
    let mut it = finder.find_iter(s.as_bytes()).into_owned();
    let mut prev = 0;
    let mut done = false;

    std::iter::from_fn(move || match it.next() {
        Some(i) => {
            let res = prev..i;
            prev = i + delim.len();
            Some(res)
        }
        None => {
            if done {
                None
            } else {
                done = true;
                Some(prev..s.len())
            }
        }
    })
}
