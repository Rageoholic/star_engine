use bevy::prelude::*;

#[derive(Component, Debug)]
struct Person;
#[derive(Component, Debug)]
struct Name(String);

struct GreetTimer(Timer);

impl std::ops::Deref for GreetTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello_person(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}", name.0);
        }
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Ada".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Kylie".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Lexie".to_string()));
}

fn main() {
    App::new()
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(hello_person)
        .run();
}
