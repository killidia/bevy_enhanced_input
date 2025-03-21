use core::any;

use bevy::{input::InputPlugin, prelude::*};
use bevy_enhanced_input::prelude::*;

#[test]
fn explicit() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin))
        .add_input_context::<DummyContext>()
        .add_observer(binding);

    let entity = app.world_mut().spawn(DummyContext).id();

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();
    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Explicit>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(Explicit::KEY);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();
    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Explicit>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Fired);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(Explicit::KEY);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();
    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Explicit>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);
}

#[test]
fn implicit() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin))
        .add_input_context::<DummyContext>()
        .add_observer(binding);

    let entity = app.world_mut().spawn(DummyContext).id();

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Implicit>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(ReleaseAction::KEY);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Ongoing);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Implicit>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::Ongoing);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(ReleaseAction::KEY);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::Fired);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Implicit>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::Fired);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Implicit>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);
}

#[test]
fn blocker() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin))
        .add_input_context::<DummyContext>()
        .add_observer(binding);

    let entity = app.world_mut().spawn(DummyContext).id();

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Blocker>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
    keys.press(ReleaseAction::KEY);
    keys.press(Blocker::KEY);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Ongoing);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Blocker>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Fired);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(ReleaseAction::KEY);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::Fired);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Blocker>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::None);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<Blocker>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Fired);
}

#[test]
fn events_blocker() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin))
        .add_input_context::<DummyContext>()
        .add_observer(binding);

    let entity = app.world_mut().spawn(DummyContext).id();

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<EventsBlocker>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
    keys.press(ReleaseAction::KEY);
    keys.press(EventsBlocker::KEY);

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Ongoing);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<EventsBlocker>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Fired);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(ReleaseAction::KEY);
    let observers = panic_on_action_events::<EventsBlocker>(app.world_mut());

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::Fired);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<EventsBlocker>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Fired);

    for entity in observers {
        app.world_mut().despawn(entity);
    }

    app.update();

    let registry = app.world().resource::<InputContextRegistry>();

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<ReleaseAction>();
    assert_eq!(action.value(), false.into());
    assert_eq!(action.state(), ActionState::None);

    let ctx = registry.context::<DummyContext>(entity);
    let action = ctx.action::<EventsBlocker>();
    assert_eq!(action.value(), true.into());
    assert_eq!(action.state(), ActionState::Fired);
}

fn binding(mut trigger: Trigger<Binding<DummyContext>>) {
    trigger
        .bind::<ReleaseAction>()
        .to(ReleaseAction::KEY)
        .with_conditions(Release::default());
    trigger
        .bind::<Explicit>()
        .with_conditions(Press::default())
        .to(Explicit::KEY);
    trigger
        .bind::<Implicit>()
        .with_conditions(Chord::<ReleaseAction>::default());
    trigger
        .bind::<Blocker>()
        .to(Blocker::KEY)
        .with_conditions(BlockBy::<ReleaseAction>::default());
    trigger
        .bind::<EventsBlocker>()
        .to(EventsBlocker::KEY)
        .with_conditions(BlockBy::<ReleaseAction>::events_only());
}

#[derive(Debug, Component)]
struct DummyContext;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct ReleaseAction;

impl ReleaseAction {
    const KEY: KeyCode = KeyCode::KeyA;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Explicit;

impl Explicit {
    const KEY: KeyCode = KeyCode::KeyB;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Implicit;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Blocker;

impl Blocker {
    const KEY: KeyCode = KeyCode::KeyD;
}

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct EventsBlocker;

impl EventsBlocker {
    const KEY: KeyCode = KeyCode::KeyE;
}

fn panic_on_action_events<A: InputAction>(world: &mut World) -> [Entity; 5] {
    [
        world.add_observer(panic_on_event::<Started<A>>).id(),
        world.add_observer(panic_on_event::<Ongoing<A>>).id(),
        world.add_observer(panic_on_event::<Fired<A>>).id(),
        world.add_observer(panic_on_event::<Completed<A>>).id(),
        world.add_observer(panic_on_event::<Canceled<A>>).id(),
    ]
}

fn panic_on_event<E: Event>(_trigger: Trigger<E>) {
    panic!(
        "event for action `{}` shouldn't trigger",
        any::type_name::<E>()
    );
}
