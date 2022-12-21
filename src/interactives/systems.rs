use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};
use bevy_mod_raycast::Ray3d;
use bevy_rapier3d::prelude::*;
use iyes_loopless::state::NextState;

use crate::{
    animation::AnimationRegistry,
    camera::{
        BackToSpot,
        BackToState,
        CameraSpots,
        CurrentSpot,
        Ignores,
        NextSpot,
    },
    commands::CommandsExt,
    interactives::{
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
    MAIN_CAMERA,
};

type CameraQuery = (&'static Camera, &'static GlobalTransform);

#[derive(SystemParam)]
pub struct InteractiveQuery<'w, 's, T: Interactive + Component + 'static> {
    camera: Query<'w, 's, CameraQuery>,
    ignore: Query<'w, 's, &'static Ignores>,
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

    input: Res<Input<MouseButton>>,
    window: Res<Windows>,
    ctx: Res<RapierContext>,
    dragging: Res<DraggingItem>,
    mut inventory: ResMut<Inventory>,
    animation_server: Res<AnimationRegistry>,
    mut state: ResMut<WorldState>,
    at_spot: ResMut<CurrentSpot>,
    mut next_spot: ResMut<NextSpot>,

    mut query: InteractiveQuery<T>,
) {
    if input.just_released(MouseButton::Left) {
        let (camera, gtf) = query.camera.get_single().unwrap();

        if next_spot.is_none() && at_spot.is_some() {
            let cursor = window.primary().cursor_position().unwrap();

            if cursor.y > 100.0 {
                let ray = Ray3d::from_screenspace(cursor, camera, gtf).unwrap();

                let mut ignores = Vec::new();

                if let Ok(ignored) = query.ignore.get(at_spot.get().entity()) {
                    ignores.extend(commands.named_any(&ignored.names));
                }

                if let Some((entity, _)) = ctx.cast_ray(
                    ray.origin(),
                    ray.direction(),
                    64.0,
                    true,
                    QueryFilter::new().predicate(&|entity| !ignores.contains(&entity)),
                ) {
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
                                Action::Animation(animation) => {
                                    for mut player in &mut query.players {
                                        player.play(
                                            animation_server.get(animation.as_str()).unwrap(),
                                        );
                                    }
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
    }

    for mut actions in &mut query.interactives {
        actions.update(&mut commands, &mut state);
    }
}
