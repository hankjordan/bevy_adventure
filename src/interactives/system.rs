use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};
use iyes_loopless::state::NextState;

use crate::{
    audio::AudioServer,
    camera::{
        BackToSpot,
        BackToState,
        CameraSpots,
        CurrentSpot,
        NextSpot,
    },
    commands::CommandsExt,
    interactives::{
        hovering::Hovering,
        Action,
        Interactive,
        ItemRef,
    },
    inventory::{
        DraggingItem,
        Inventory,
    },
    state::WorldState,
    textdisplay::{
        Message,
        TextDisplay,
    },
    AnimationServer,
    Cursor,
    MAIN_CAMERA,
};

#[derive(SystemParam)]
pub struct InteractiveQuery<'w, 's, T: Interactive + Component + 'static> {
    back_spot: Query<'w, 's, &'static BackToSpot>,
    back_state: Query<'w, 's, &'static BackToState<T::State>>,
    players: Query<'w, 's, &'static mut AnimationPlayer>,
    interactives: Query<'w, 's, &'static mut T>,
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::needless_pass_by_value)]
pub fn interactive<T: Interactive + Component>(
    mut commands: CommandsExt,
    mut display: TextDisplay,
    spots: CameraSpots,
    animation_server: AnimationServer,
    audio_server: AudioServer,

    input: Res<Input<MouseButton>>,
    cursor: Res<Cursor>,
    dragging: Res<DraggingItem>,
    mut inventory: ResMut<Inventory>,
    mut state: ResMut<WorldState>,
    at_spot: ResMut<CurrentSpot>,
    mut next_spot: ResMut<NextSpot>,

    hovering: Res<Hovering>,

    mut query: InteractiveQuery<T>,
) {
    if input.just_released(MouseButton::Left) && next_spot.is_none() && at_spot.is_some() {
        if cursor.position().y > 100.0 {
            if let Some(entity) = hovering.entity {
                if let Ok(mut interactive) = query.interactives.get_mut(entity) {
                    if at_spot.get().entity() != entity {
                        if let Some(spot) = spots.for_interactive(entity) {
                            next_spot.set(spot.name());
                            next_spot.set_entity(entity);
                            return;
                        }
                    }

                    let actions;

                    if let Some(dragged) = &dragging.src {
                        let mut item = ItemRef::new(dragged);

                        actions = interactive.use_item(&mut state, &mut item);

                        if item.consumed() {
                            inventory.items.remove(dragged);
                        }
                    } else {
                        actions = interactive.interact(&mut state);
                    }

                    for action in actions {
                        match action {
                            Action::AddItem(name) => {
                                display.show(Message::ItemPickup(name.clone()));
                                inventory.items.insert(name);
                            }
                            Action::Animation(name) => {
                                for mut player in &mut query.players {
                                    if let Some(animation) = animation_server.get(&name) {
                                        player.play(animation);
                                    }
                                }
                            }
                            Action::Audio(name) => {
                                audio_server.play(&name);
                            }
                            Action::Message(text) => display.show(text),
                            Action::Transition(state) => {
                                commands.insert_resource(NextState(state));
                            }
                            Action::Move(name) => next_spot.set(&name),
                        }
                    }
                }
            }
        } else if !dragging.is_dragging() {
            if let Ok(back) = query.back_spot.get(at_spot.get().entity()) {
                next_spot.set(&back.name);
            } else if let Ok(back) = query.back_state.get(at_spot.get().entity()) {
                commands.insert_resource(NextState(back.state.clone()));
            } else {
                next_spot.set(MAIN_CAMERA);
            }
        }
    }

    for mut actions in &mut query.interactives {
        actions.update(&mut commands, &mut state);
    }
}
