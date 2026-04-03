#[cfg(test)]
#[path = "../Tests/SharedTests.rs"]
mod shared_tests;

// Modules moved from game's collapsed plugin folders.
#[path = "Plugins/BevyInspector/BevyInspectorComponent.rs"]
pub mod bevy_inspector_component;
#[path = "Plugins/BevyInspector/BevyInspectorPlugin.rs"]
pub mod bevy_inspector_plugin;
#[path = "Plugins/BevyInspector/BevyInspectorSystem.rs"]
pub mod bevy_inspector_system;
#[path = "Plugins/HotReload/ContextPlugin.rs"]
pub mod context_plugin;
#[path = "Plugins/HotReload/ContextResource.rs"]
pub mod context_resource;
#[path = "Plugins/HotReload/ContextSystem.rs"]
pub mod context_system;
#[path = "Plugins/CustomWindow/CustomWindowComponent.rs"]
pub mod custom_window_component;
#[path = "Plugins/CustomWindow/CustomWindowPlugin.rs"]
pub mod custom_window_plugin;
#[path = "Plugins/CustomWindow/CustomWindowResource.rs"]
pub mod custom_window_resource;
#[path = "Plugins/CustomWindow/CustomWindowSystem.rs"]
pub mod custom_window_system;
