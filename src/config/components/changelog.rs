use crate::config::{ApplyLayer, ApplyValExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Config for tweaking the changelog page generation
#[derive(Debug, Clone)]
pub struct ChangelogConfig {
    /// Whether to attempt to read from the local changelog file
    pub read_changelog_file: bool,
}

/// The config for generating a separate changelog page
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ChangelogLayer {
    /// Whether we factor in the local `CHANGELOG.md` file, attempt to parse
    /// it, and try and match version headings to release versions that we
    /// detect.
    pub read_changelog_file: Option<bool>,
}

impl Default for ChangelogConfig {
    fn default() -> Self {
        ChangelogConfig {
            read_changelog_file: true,
        }
    }
}

impl ApplyLayer for ChangelogConfig {
    type Layer = ChangelogLayer;
    fn apply_layer(&mut self, layer: Self::Layer) {
        // This is intentionally written slightly cumbersome to make you update this
        let ChangelogLayer {
            read_changelog_file,
        } = layer;
        self.read_changelog_file.apply_val(read_changelog_file);
    }
}
