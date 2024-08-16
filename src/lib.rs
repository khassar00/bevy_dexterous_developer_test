use bevy::prelude::*;
use bevy_dexterous_developer::{reloadable_main, reloadable_scope, ReloadableElementsSetup};
use bevy_dexterous_developer::ReloadableAppContents;
use bevy_dexterous_developer::ReloadableApp;
reloadable_main!( bevy_main(initial_plugins) {
    App::new()
        .add_plugins(initial_plugins.initialize::<DefaultPlugins>())
        .setup_reloadable_elements::<reloadable>()
        .run();
});
reloadable_scope!(reloadable(app) {
    app.add_systems(Update,update);
});
fn update() {
    println!("Swapped Update System!");
}
