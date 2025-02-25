use fitness::*;
mod weightlifting;
mod cardio;

fn main() {
    let x = fitness::GymWorkout::new(fitness::CardioExercise {day: String::from("Tuesday"), tool: CardioTool::Bike, minutes: 18}, fitness::WeightliftingExercise {name: String::from("Pelvic Pulverisers"), reps: 97});

    println!("{x:#?}");
}
