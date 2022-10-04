//! # Encodings for Common Constraint Types to CNF
//! 
//! CNF encodings for cardinality and pseudo-boolean constraints.

pub mod card;
pub mod pb;

/// Possible bound types for cardinality constraints
pub enum BoundType {
    /// Can only enforce lower bounds
    LB,
    /// Can only enforce upper bounds
    UB,
    /// Can enforce both lower and upper bounds
    EQ,
}