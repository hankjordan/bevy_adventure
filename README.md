# Bevy_adventure [![][img_crates]][crates] [![][img_doc]][doc]

A framework for building 3d adventure games in Bevy.

## Features

- `Interactive` trait is the backbone of the framework, allowing you to create powerful, dynamic objects in your world that can be interacted with and can affect other objects or global state.
- `Scene` trait exposes a `Plugin`-like interface for managing `GLTF` scenes and assigning components to entities (based on `bevy_scene_hook`)
- `WorldState` resource, a stringly-typed storage for tracking progression
- `Inventory` resource allows you to track held items and create recipes for combining them
- Automatic camera animation and state management, Component-based interface
- Support for multiple scenes (built on top of `iyes_loopless`)

## Notes

### Save States

`bevy_adventure` intentionally omits implementing save state functionality, as different games will have different requirements.

If you want to implement this, here is what you need to keep track of:
- `AtSpot` camera component, which determines your current spot in the scene
- `CurrentState<State>` resource, (from `iyes_loopless`) which determines what scene is currently loaded
- `Inventory` resource, which tracks what items the player is holding
- `WorldState` resource, the global game state storage

You will also have to enable the `serde` feature to allow serialization and deserialization of these resources.

### Using `WorldState`

When you are building interactives, you have the ability to store information in the component itself or the `WorldState`.

The component should only hold temporary state information - like which drawer is open on a dresser or if an entity has been spawned.

If you want to store any other kind of information, it should be done in the `WorldState` resource. This is so other interactives in your scene can access this state (for example, you'd flip a switch and the lights would go out) and so you are storing state information in a single place.

### Triggering multiple actions

The `Interactive` trait only allows you to return a single `Action` when an object is interacted with. This is done for ergonomics,

## TODO

- Audio
- Documentation
- Examples
- Gamepad support

[img_crates]: https://img.shields.io/crates/v/bevy_adventure.svg
[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

[crates]: https://crates.io/crates/bevy_adventure
[doc]: https://docs.rs/bevy_adventure/
