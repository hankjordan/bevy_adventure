# Bevy_adventure 
[![][img_bevy]][bevy] [![][img_version]][crates] [![][img_doc]][doc] [![][img_license]][license] [![][img_tracking]][tracking] [![][img_downloads]][crates]

A framework for building 3d adventure games in Bevy.

## Features

- `Interactive` trait is the backbone of the framework, allowing you to create powerful, dynamic objects in your world that can be interacted with and can affect other objects or global state.
- `Scene` trait exposes a `Plugin`-like interface for managing `GLTF` scenes and assigning components to entities (based on `bevy_scene_hook`)
- `WorldState` resource, a stringly-typed storage for tracking progression
- `Inventory` resource allows you to track held items and create recipes for combining them
- Automatic camera animation and state management, Component-based interface
- Support for multiple scenes (built on top of `iyes_loopless`)

## Examples

![Preview](https://raw.githubusercontent.com/hankjordan/bevy_adventure/master/examples/preview.mp4)

There is an elaborate example available at `examples/main.rs`.
Three scenes are included, with multiple objects that can be interacted with or picked up.

## Notes

### Save States

`bevy_adventure` intentionally omits implementing save state functionality, as different games will have different requirements.

If you want to implement this, here is what you need to keep track of:
- `CurrentSpot` resource, which determines your current camera spot in the scene.
When saving, save the name of the `CameraSpot` from this resource.
When loading, create a new `NextSpot` instance from the loaded name, and insert it with `Commands`.
- `CurrentState<State>` resource, (from `iyes_loopless`) which determines what scene is currently loaded
- `Inventory` resource, which tracks what items the player is holding
- `WorldState` resource, the global game state storage

You will also have to enable the `serde` feature to allow serialization and deserialization of these resources.

### Using `WorldState`

When you are building interactives, you have the ability to store information in the component itself or the `WorldState`.

The component should only hold temporary state information - like which drawer is open on a dresser or if an entity has been spawned.

If you want to store any other kind of information, it should be done in the `WorldState` resource. This is so other interactives in your scene can access this state (for example, you'd flip a switch and the lights would go out) and so you are storing state information in a single place.

## TODO

- Audio
- Save state example
- Gamepad support

## License

bevy_adventure is dual-licensed under MIT and Apache-2.0.

## Compatibility

NOTE: We do not track Bevy main.

|Bevy Version|Crate Version      |
|------------|-------------------|
|`0.9`       |`0.1`              |

[img_bevy]: https://img.shields.io/badge/Bevy-0.9-blue
[img_version]: https://img.shields.io/crates/v/bevy_adventure.svg
[img_doc]: https://docs.rs/bevy_adventure/badge.svg
[img_license]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[img_downloads]:https://img.shields.io/crates/d/bevy_adventure.svg
[img_tracking]: https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue

[bevy]: https://crates.io/crates/bevy/0.9.1
[crates]: https://crates.io/crates/bevy_adventure
[doc]: https://docs.rs/bevy_adventure/
[license]: https://github.com/hankjordan/bevy_adventure#license
[tracking]: https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking