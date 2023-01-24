# Bevy_adventure 
[![][img_bevy]][bevy] [![][img_version]][crates] [![][img_doc]][doc] [![][img_license]][license] [![][img_tracking]][tracking] [![][img_downloads]][crates]

A framework for building 3d adventure games in Bevy.


<https://user-images.githubusercontent.com/29737477/212168485-6497dfdf-cfa6-406a-814b-8a6598e20152.mp4>


## Features

- `Interactive` trait is the backbone of the framework, allowing you to create powerful, dynamic objects in your world that can be interacted with and can affect other objects or global state.
- `Scene` trait exposes a `Plugin`-like interface for managing `GLTF` scenes and assigning components to entities (based on `bevy_scene_hook`)
- `WorldState` resource, a stringly-typed storage for tracking progression
- `Inventory` resource allows you to track held items and create recipes for combining them
- Automatic camera animation and state management, Component-based interface
- Support for multiple scenes (built on top of `iyes_loopless`)

## Examples

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

### Creating scenes

When implementing the Scene trait, the `scene` method should return a path to a file that can be loaded with the `AssetServer` to create a Bevy Scene.
In practice this is a GLTF scene file, but any other format can be used as long as it creates a Bevy Scene when loaded.

When imported with bevy_adventure, Cameras are automatically converted to CameraSpots, and PointLights are configured to cast shadows.

The Scene trait's `spawn` method is called for every Entity in the scene - you can use this to assign components and initialize behavior for objects in your scene.
The best way to do this is to match entity Names (from their names in the Scene), see the examples for an idea of how you should do this.

The only real requirement for your scene is that you create a special Camera with the name `Camera_Main`.
This is where your Camera will be positioned upon entering the Scene.
**The app will panic if a scene is loaded without the Main Camera**.

If you have issues matching objects by name, you might be matching the parent instead of the actual object you want to match.
Adding a print statement inside of your Scene's spawn method might help you figure out if the object is actually being found.

Note that when you are loading Scenes into Bevy, the 'Transform Scale' of each object is also loaded in.
This could cause colliders to be the wrong size - you should 'apply Scale' before exporting.

Object origin is also used loaded in - this could cause misaligned colliders if your object's origin doesn't match the center of your object.

Blender is a good choice to create GLTF scenes, but you must make sure you configure the export settings correctly:

- Under `Include`, check `Custom Properties`, `Cameras`, and `Punctual Lights`.
- Under `Transform`, uncheck `Y+ Up`

## TODO

- Audio example
- Save state example

## License

`bevy_adventure` is dual-licensed under MIT and Apache-2.0.

## Compatibility

NOTE: We do not track Bevy main.

|Bevy Version|Crate Version              |
|------------|---------------------------|
|`0.9`       |`0.1`, `0.2`, `0.3`, `0.4` |

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

[preview]: https://github.com/hankjordan/bevy_adventure/examples
