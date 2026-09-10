pub mod prompt;
pub mod persona;

pub use prompt::{
    CommentaryGenerator, CommentaryScript, NEUROCLIP_COMMENTARY_SYSTEM_PROMPT,
};
pub use persona::CommentaryPersona;
