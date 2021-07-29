// Note: Loads the contents of the module square_content from another file
//       with the same name as the module. Read more at
//       https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
mod square_content;
mod board_index;
mod board_content;
mod row;
mod game;

// Note: Re-exports the content of the square_content module to keep paths short.
//       Read more at https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
pub use crate::square_content::*;
pub use crate::board_index::*;
pub use crate::board_content::*;
pub use crate::row::*;
pub use crate::game::*;
