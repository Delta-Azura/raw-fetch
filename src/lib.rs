pub mod getconf;
pub mod getdistant;
pub mod r#match;

pub use crate::getconf::getconf;
pub use crate::r#match::compare;
pub use crate::getdistant::getdistant;