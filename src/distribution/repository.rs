//! Repository types of the distribution spec.

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
    /// RepositoryList returns a catalog of repositories maintained on the registry.
    struct RepositoryList {
        /// The items of the RepositoryList.
        repositories: Vec<String>,
    }
);

#[cfg(test)]
#[cfg(feature = "builder")]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn repository_list_success() -> Result<()> {
        let list = RepositoryListBuilder::default()
            .repositories(vec![])
            .build()?;
        assert!(list.repositories().is_empty());
        Ok(())
    }

    #[test]
    fn repository_list_failure() {
        assert!(RepositoryListBuilder::default().build().is_err());
    }
}
