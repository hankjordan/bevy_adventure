use bevy::{
    ecs::{
        schedule::StateData,
        system::SystemParam,
    },
    prelude::*,
};
use iyes_loopless::state::NextState;

use crate::{
    camera::{
        BackToSpot,
        BackToState,
        CameraSpots,
        CurrentSpot,
        NextSpot,
        SkipAnimation,
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
    Cursor,
    MAIN_CAMERA,
};

#[derive(Resource, Default)]
pub struct Interaction {
    state: State,
}

#[derive(Default)]
pub enum State {
    #[default]
    Ready,
    Prepared,
    Interact,
    Complete,
}

impl Interaction {
    fn ready(&mut self) -> bool {
        if let State::Ready = self.state {
            self.state = State::Prepared;
            true
        } else {
            false
        }
    }

    fn begin(&mut self) -> bool {
        if let State::Prepared = self.state {
            self.state = State::Interact;
            true
        } else {
            false
        }
    }

    fn ok(&self) -> bool {
        matches!(self.state, State::Interact)
    }

    fn done(&mut self) {
        self.state = State::Complete;
    }
}

pub fn reset_interaction(mut commands: Commands) {
    commands.insert_resource(Interaction::default());
}

#[derive(Resource)]
pub struct LookingAt(pub Entity);

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::collapsible_if)]
pub fn prepare_interaction<T: StateData>(
    mut commands: Commands,
    spots: CameraSpots,

    input: Res<Input<MouseButton>>,
    cursor: Res<Cursor>,

    mut interaction: ResMut<Interaction>,

    dragging: Res<DraggingItem>,
    at_spot: ResMut<CurrentSpot>,

    back_spot: Query<&BackToSpot>,
    back_state: Query<&BackToState<T>>,
) {
    if interaction.ready() {
        if input.just_released(MouseButton::Left) {
            if cursor.position().y > 100.0 {
                interaction.begin();
            } else if !dragging.is_dragging() {
                interaction.done();

                let mut back = None;

                if let Ok(spot) = back_spot.get(at_spot.get().entity()) {
                    back = Some(spot);
                } else if let Some(looking_at) = spots.for_spot(at_spot.get()) {
                    if let Ok(spot) = back_spot.get(looking_at) {
                        back = Some(spot);
                    }
                }

                if let Some(spot) = back {
                    commands.insert_resource(NextSpot(spot.name.clone()));
                } else {
                    if let Ok(back) = back_state.get(at_spot.get().entity()) {
                        commands.insert_resource(NextState(back.state.clone()));
                    }

                    commands.insert_resource(NextSpot(MAIN_CAMERA.to_owned()));
                }

                commands.remove_resource::<LookingAt>();
            }
        }
    }
}

#[derive(SystemParam)]
pub struct Interactives<'w, 's, T: Interactive + Component + 'static> {
    interaction: ResMut<'w, Interaction>,
    hovering: Res<'w, Hovering>,
    query: Query<'w, 's, &'static mut T>,
}

impl<'w, 's, T: Interactive + Component + 'static> Interactives<'w, 's, T> {
    fn get(&mut self) -> Option<(Entity, Mut<T>)> {
        if self.interaction.ok() {
            if let Some(entity) = self.hovering.entity {
                if let Ok(interactive) = self.query.get_mut(entity) {
                    self.interaction.done();
                    return Some((entity, interactive));
                }
            }
        }

        None
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::needless_pass_by_value)]
pub fn interactive<T: Interactive + Component>(
    mut commands: CommandsExt,
    mut display: TextDisplay,
    spots: CameraSpots,

    dragging: Res<DraggingItem>,
    mut inventory: ResMut<Inventory>,
    mut state: ResMut<WorldState>,
    at_spot: ResMut<CurrentSpot>,

    mut interactives: Interactives<T>,
) {
    if let Some((entity, mut interactive)) = interactives.get() {
        let mut focused = true;

        if let Some(spot) = spots.for_interactive(entity) {
            if at_spot.get().entity() != spot.entity() {
                commands.insert_resource(NextSpot(spot.name().to_owned()));
                commands.insert_resource(LookingAt(entity));
                focused = false;
            }
        }

        if focused {
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
                        commands.play_animation(&name);
                    }
                    Action::Audio(name) => {
                        commands.play_audio(&name);
                    }
                    Action::Message(text) => display.show(text),
                    Action::Transition(state) => {
                        commands.insert_resource(NextState(state));
                    }
                    Action::Move(name) => commands.insert_resource(NextSpot(name)),
                    Action::Jump(name) => {
                        commands.insert_resource(NextSpot(name));
                        commands.insert_resource(SkipAnimation);
                    }
                }
            }
        }
    }

    for mut actions in &mut interactives.query {
        actions.update(&mut commands, &mut state);
    }
}
