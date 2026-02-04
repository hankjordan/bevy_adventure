use bevy::prelude::*;
// use bevy::state::state::FreelyMutableState;
// use bevy_rapier3d::prelude::*;

fn test_event_writer(mut ew: EventWriter<()>) {}
fn test_message_writer(mut mw: MessageWriter<()>) {}

fn test_rapier(
    // ctx: ReadDefaultRapierContext,
) {}

fn main() {}

#[derive(Event)]
struct TestEvent;

#[derive(bevy::ecs::system::SystemParam)]
struct TestParam<'w, 's> {
    q: Query<'w, 's, Entity>,
}
