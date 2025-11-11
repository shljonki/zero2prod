// zero/src/routes/mod.rs
pub mod greeting;
pub mod health_check;
pub mod subscriptions;

// Re‑export everything that `subscriptions` makes public
pub use greeting::*;
pub use health_check::*;
pub use subscriptions::*;



// pub mod subscriptions; makes the module itself public, so other code can refer to it as crate::routes::subscriptions.

// pub use subscriptions::*; re‑exports all the public items inside that module (functions, structs, etc.) at the level
// of routes. After this line, code can reach those items directly through crate::routes::<item> instead of having to go
// through the extra subscriptions segment.
