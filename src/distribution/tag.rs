//! Tag types of the distribution spec.

use serde::{Deserialize, Serialize};

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(
            pattern = "owned",
            setter(into, strip_option),
            build_fn(error = "crate::error::OciSpecError")
        ),
        getset(get = "pub")
    )]
    /// A list of tags for a given repository.
    struct TagList {
        /// The namespace of the repository.
        name: String,

        /// Each tags on the repository.
        tags: Vec<String>,
    }
);

#[cfg(test)]
#[cfg(feature = "builder")]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn tag_list_success() -> Result<()> {
        let list = TagListBuilder::default()
            .name("name")
            .tags(vec![])
            .build()?;
        assert!(list.tags().is_empty());
        assert_eq!(list.name(), "name");
        Ok(())
    }

    #[test]
    fn tag_list_failure() {
        assert!(TagListBuilder::default().build().is_err());
    }
}
