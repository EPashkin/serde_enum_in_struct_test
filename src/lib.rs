extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Ident {
    Name(String),
    Pattern(String),
}

impl PartialEq for Ident {
    fn eq(&self, other: &Ident) -> bool {
        pub use self::Ident::*;
        match (self, other) {
            (&Name(ref s1), &Name(ref s2)) => s1 == s2,
            (&Pattern(ref r1), &Pattern(ref r2)) => r1.as_str() == r2.as_str(),
            _ => false,
        }
    }
}

impl Eq for Ident {}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Func {
    ident: Ident,
    typ: String,
    /*
    some other fields
    */
}

#[derive(Debug, Deserialize)]
pub struct FuncWithName {
    name: String,
    typ: String,
}

#[derive(Debug, Deserialize)]
pub struct FuncWithPattern {
    pattern: String,
    typ: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_func_by_name() {
        let f = serde_json::from_str::<Func>(r#"{"name":"func1", "typ":"str"}"#)
            .unwrap();

        assert_eq!(f.ident, Ident::Name("func1".to_owned()));
        assert_eq!(f.typ, "str");
    }

    #[test]
    fn deserialize_func_by_pattern() {
        let f = serde_json::from_str::<Func>(r#"{"pattern":".+", "typ":"str"}"#)
            .unwrap();

        assert_eq!(f.ident, Ident::Pattern(".+".to_owned()));
        assert_eq!(f.typ, "str");
    }

    #[test]
    fn deserialize_func_with_name() {
        let f = serde_json::from_str::<FuncWithName>(r#"{"name":"func1", "typ":"str"}"#)
            .unwrap();

        assert_eq!(f.name, "func1");
        assert_eq!(f.typ, "str");
    }

    #[test]
    fn deserialize_func_with_pattern() {
        let f = serde_json::from_str::<FuncWithPattern>(r#"{"pattern":".+", "typ":"str"}"#)
            .unwrap();

        assert_eq!(f.pattern, ".+");
        assert_eq!(f.typ, "str");
    }

    #[test]
    fn deserialize_ident_by_name() {
        let i = serde_json::from_str::<Ident>(r#"{"name":"func1"}"#)
            .unwrap();

        assert_eq!(i, Ident::Name("func1".to_owned()));
    }
    #[test]
    fn deserialize_ident_by_pattern() {
        let i = serde_json::from_str::<Ident>(r#"{"pattern":".+"}"#)
            .unwrap();

        assert_eq!(i, Ident::Pattern(".+".to_owned()));
    }
}
