use exercises::{
    basic_types_exercises::run_basic_types_exercises,
    ownership_and_borrowing_exercises::run_ownership_and_borrowing_exercises,
    variables_exercises::run_variables_exercises,
};

mod exercises;

fn main() {
    run_variables_exercises();
    run_basic_types_exercises();
    run_ownership_and_borrowing_exercises();
}
