// In this file: An unfinished `Multimap` type that ended up not getting used.
// Maybe some day.

/*
use std::collections::HashMap;

/// A `Multimap` is a map that can contain more than one value for any given key.
/// It is conceptually, and implemented as, a `HashMap<K, Vec<V>>`.
#[derive(Default)]
pub struct Multimap<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K, V> Multimap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Make a `Multimap` from an iterator of (key, value) pairs.
    pub fn from_key_value_pairs(it: impl Iterator<Item = (K, V)>) -> Self
    where
        K: Eq + std::hash::Hash,
    {
        let mut map = Self::new();
        it.for_each(|(k, v)| map.inner.entry(k).or_default().push(v));
        map
    }
}

impl<K, V> std::fmt::Display for Multimap<K, V>
where
    K: std::fmt::Display,
    V: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, vs) in self.inner.iter() {
            for v in vs {
                writeln!(f, "{k}\t{v}")?;
            }
        }
        Ok(())
    }
}

pub async fn analyze_libhfst(
    lang: &str,
    input: &str,
) -> Result<Multimap<String, String>, ()> {
    // want to be able to name the fields, both in code, but also in the json-encoded
    // multimap
    Ok(Multimap::new())
}
*/
