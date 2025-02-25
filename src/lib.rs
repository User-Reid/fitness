mod weightlifting;
mod cardio;

pub use diet::ask_about_program;
pub use weightlifting::{Exercise as WeightliftingExercise, ask_about_program as ask_about_weightlifting};
pub use cardio::{CardioTool, Exercise as CardioExercise, ask_about_program as ask_about_cardio};

mod diet {
  pub const NUTRITIONIST: &str = "Norah Nutrition";

  pub fn ask_about_program() {
    println!("The nutritionist is {NUTRITIONIST}")
  }
}

#[derive(Debug)]
pub struct GymWorkout {
  pub cardio: CardioExercise,
  pub weightlifting: WeightliftingExercise
}

impl GymWorkout {
  pub fn new(cardio: CardioExercise, weightlifting: WeightliftingExercise) -> Self {
    ask_about_cardio();
    ask_about_weightlifting();
    ask_about_program();
    Self {
      cardio,
      weightlifting
    }
  }
}