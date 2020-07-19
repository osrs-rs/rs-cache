mod loaders;
#[doc(hidden)]
pub mod item_def;
#[doc(hidden)]
pub mod npc_def;
#[doc(hidden)]
pub mod obj_def;

pub use loaders::*;
pub use item_def::ItemDefinition;
pub use npc_def::NpcDefinition;
pub use obj_def::ObjectDefinition;