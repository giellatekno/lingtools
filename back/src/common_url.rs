#[derive(serde::Deserialize)]
pub struct LangAndInputParams {
    pub lang: String,
    pub input: String,
}

/// Common query param: `format`. Accepts "json", "pretty-json", or "text". Defaults
/// to "text" if not given.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize)]
pub enum Format {
    #[serde(alias = "json")]
    Json,
    #[serde(alias = "pretty-json")]
    PrettyJson,
    #[serde(alias = "text")]
    Text,
}

#[derive(serde::Deserialize)]
pub struct FormatQueryParam {
    format: Option<Format>,
}

impl FormatQueryParam {
    pub fn unwrap_or(&self, value: Format) -> Format {
        self.format.unwrap_or(value)
    }

    pub fn unwrap_or_json(&self) -> Format {
        self.unwrap_or(Format::Json)
    }
}

/// Common query param: `libhfst`. Accepts
#[derive(serde::Deserialize)]
pub struct LibhfstQueryParam {
    #[serde(default, deserialize_with = "deserialize_boolqueryparam")]
    libhfst: Option<BoolQueryParam>,
}

impl LibhfstQueryParam {
    /// Unwrap the value. If `libhfst` was not present in the query params, returns
    /// [`false`]. If `libhfst` was present, it is considered `true` if
    /// [`crate::util::trueish_query_param`] considers it `true`, and `false` otherwise.
    pub fn unwrap_or_false(&self) -> bool {
        match self.libhfst {
            Some(ref value) => value.0,
            None => false,
        }
    }
}

/// A field to use in a query param struct. If the named param is not given in the query
/// params, it evaluates to `false`. If it is given, the result is equal to what
/// `trueish_query_param()` considers to be the value.
pub struct BoolQueryParam(pub bool);

pub fn deserialize_boolqueryparam<'de, D>(
    deserializer: D,
) -> Result<Option<BoolQueryParam>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    match std::borrow::Cow::<'de, str>::deserialize(deserializer) {
        Ok(v) => match trueish_query_param(&v) {
            true => Ok(Some(BoolQueryParam(true))),
            false => Ok(Some(BoolQueryParam(false))),
        },
        Err(_) => {
            // XXX: Can this ever be reached?
            Ok(None)
        }
    }
}

pub fn trueish_query_param(val: &str) -> bool {
    match val {
        "1" => true,
        "y" | "Y" => true,
        "yes" | "Yes" | "YES" => true,
        "yea" | "Yea" | "YEA" => true,
        "yeah" | "Yeah" | "YEAH" => true,
        "t" | "T" => true,
        "true" | "True" | "TRUE" => true,
        _ => false,
    }
}
