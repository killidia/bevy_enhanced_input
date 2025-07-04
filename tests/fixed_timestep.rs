use bevy::{input::InputPlugin, prelude::*, time::TimeUpdateStrategy};
use bevy_enhanced_input::prelude::*;
use test_log::test;

#[test]
fn once_in_two_frames() -> Result<()> {
    let time_step = Time::<Fixed>::default().timestep() / 2;

    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(time_step))
        .add_input_context::<Test>()
        .add_observer(bind)
        .finish();

    let entity = app.world_mut().spawn(Actions::<Test>::default()).id();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(TestAction::KEY);

    for frame in 0..2 {
        app.update();

        let actions = app.world().get::<Actions<Test>>(entity).unwrap();
        assert!(
            actions.events::<TestAction>()?.is_empty(),
            "shouldn't fire on frame {frame}"
        );
    }

    for frame in 2..4 {
        app.update();

        let actions = app.world().get::<Actions<Test>>(entity).unwrap();
        assert_eq!(
            actions.events::<TestAction>()?,
            ActionEvents::STARTED | ActionEvents::FIRED,
            "should maintain start-firing on frame {frame}"
        );
    }

    Ok(())
}

#[test]
fn twice_in_one_frame() -> Result<()> {
    let time_step = Time::<Fixed>::default().timestep() * 2;

    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin))
        .insert_resource(TimeUpdateStrategy::ManualDuration(time_step))
        .add_input_context::<Test>()
        .add_observer(bind)
        .finish();

    let entity = app.world_mut().spawn(Actions::<Test>::default()).id();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(TestAction::KEY);

    app.update();

    let actions = app.world().get::<Actions<Test>>(entity).unwrap();
    assert!(
        actions.events::<TestAction>()?.is_empty(),
        "`FixedMain` should never run on the first frame"
    );

    app.update();

    let actions = app.world().get::<Actions<Test>>(entity).unwrap();
    assert_eq!(
        actions.events::<TestAction>()?,
        ActionEvents::FIRED,
        "should run twice, so it shouldn't be started on the second run"
    );

    Ok(())
}

fn bind(trigger: Trigger<Bind<Test>>, mut actions: Query<&mut Actions<Test>>) {
    let mut actions = actions.get_mut(trigger.target()).unwrap();
    actions.bind::<TestAction>().to(TestAction::KEY);
}

#[derive(InputContext)]
#[input_context(schedule = FixedPreUpdate)]
struct Test;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct TestAction;

impl TestAction {
    const KEY: KeyCode = KeyCode::KeyA;
}
