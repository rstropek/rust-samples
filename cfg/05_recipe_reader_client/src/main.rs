use recipe_reader::{yaml, RecipeMarkdownFactory};
//                  ^^^^
//                  Here we use the yaml module from the recipe_reader crate.
//                  Here is the new error message from Rust 1.72:

// error[E0432]: unresolved import `recipe_reader::yaml`
//   --> src/main.rs:1:21
//    |
// 1  | use recipe_reader::{yaml, RecipeMarkdownFactory};
//    |                     ^^^^ no `yaml` in the root
//    |
// note: found an item that was configured out             <<< THIS IS NEW IN 1.72
//   --> /home/rainer/github/rust-samples/cfg/04_recipe_reader/src/lib.rs:39:9
//    |
// 39 | pub mod yaml                                       <<< THIS IS NEW IN 1.72
//    |         ^^^^                                       <<< THIS IS NEW IN 1.72
//    = note: the item is gated behind the `yaml` feature  <<< THIS IS NEW IN 1.72

fn main() {
    let reader = yaml::YamlRecipeReader();
    let factory = RecipeMarkdownFactory();
    let markdown = factory.file_to_markdown("caesar_salad.yaml", reader).unwrap();
    println!("{markdown}");
}
