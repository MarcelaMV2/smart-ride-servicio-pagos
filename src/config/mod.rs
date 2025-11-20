pub mod settings;

pub use settings::Settings;

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}