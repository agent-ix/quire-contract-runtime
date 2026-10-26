//! Default-stack child executable for the deep type consumer control.

mod deep_value_type;

fn main() {
    deep_value_type::exercise_deep_type();
    println!("{}", deep_value_type::CHILD_COMPLETED);
}
