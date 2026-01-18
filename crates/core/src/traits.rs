//! Core traits used across Immortal Engine
//!
//! These traits define common behaviors for components, nodes, and other engine entities.

use crate::error::EngineResult;
use uuid::Uuid;

/// Trait for entities that have a unique identifier
pub trait Identifiable {
    /// Returns the unique identifier for this entity
    fn id(&self) -> Uuid;
}

/// Trait for entities that have a name
pub trait Named {
    /// Returns the name of this entity
    fn name(&self) -> &str;

    /// Returns an optional description
    fn description(&self) -> Option<&str> {
        None
    }
}

/// Trait for entities that can be validated
pub trait Validatable {
    /// Validates the entity and returns any validation errors
    fn validate(&self) -> EngineResult<()>;

    /// Returns whether the entity is currently valid
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

/// Trait for entities that can be cloned with a new ID
pub trait CloneWithNewId: Clone {
    /// Creates a clone of this entity with a new unique identifier
    fn clone_with_new_id(&self) -> Self;
}

/// Trait for entities that have a visual position on the canvas
pub trait Positioned {
    /// Returns the x coordinate
    fn x(&self) -> f32;

    /// Returns the y coordinate
    fn y(&self) -> f32;

    /// Returns the position as a tuple
    fn position(&self) -> (f32, f32) {
        (self.x(), self.y())
    }

    /// Sets the position
    fn set_position(&mut self, x: f32, y: f32);

    /// Moves the entity by a delta
    fn translate(&mut self, dx: f32, dy: f32) {
        let (x, y) = self.position();
        self.set_position(x + dx, y + dy);
    }
}

/// Trait for entities that have dimensions
pub trait Sized {
    /// Returns the width
    fn width(&self) -> f32;

    /// Returns the height
    fn height(&self) -> f32;

    /// Returns the size as a tuple
    fn size(&self) -> (f32, f32) {
        (self.width(), self.height())
    }

    /// Sets the size
    fn set_size(&mut self, width: f32, height: f32);
}

/// Trait for entities that occupy a rectangular area (positioned + sized)
pub trait Bounded: Positioned + Sized {
    /// Returns the bounding rectangle as (x, y, width, height)
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x(), self.y(), self.width(), self.height())
    }

    /// Checks if a point is within the bounds
    fn contains_point(&self, px: f32, py: f32) -> bool {
        let (x, y, w, h) = self.bounds();
        px >= x && px <= x + w && py >= y && py <= y + h
    }

    /// Checks if this bounds intersects with another
    fn intersects(&self, other: &impl Bounded) -> bool {
        let (x1, y1, w1, h1) = self.bounds();
        let (x2, y2, w2, h2) = other.bounds();

        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }

    /// Returns the center point
    fn center(&self) -> (f32, f32) {
        let (x, y, w, h) = self.bounds();
        (x + w / 2.0, y + h / 2.0)
    }
}

/// Blanket implementation for anything that is both Positioned and Sized
impl<T: Positioned + Sized> Bounded for T {}

/// Trait for entities that can be serialized to/from JSON
pub trait JsonSerializable: serde::Serialize + serde::de::DeserializeOwned {
    /// Serializes to a JSON string
    fn to_json(&self) -> EngineResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| crate::error::EngineError::Serialization(e.to_string()))
    }

    /// Deserializes from a JSON string
    fn from_json(json: &str) -> EngineResult<Self> {
        serde_json::from_str(json).map_err(|e| crate::error::EngineError::Deserialization(e.to_string()))
    }
}

/// Blanket implementation for anything that implements Serialize + DeserializeOwned
impl<T: serde::Serialize + serde::de::DeserializeOwned> JsonSerializable for T {}

/// Trait for entities that can generate code
pub trait CodeGenerator {
    /// The type of output this generator produces
    type Output;

    /// Generates code and returns the output
    fn generate(&self) -> EngineResult<Self::Output>;
}

/// Trait for entities that can accept visitor pattern
pub trait Visitable<V> {
    /// Accept a visitor
    fn accept(&self, visitor: &mut V);
}

/// Trait for entities that support undo/redo operations
pub trait Undoable {
    /// The type representing a snapshot of state
    type Snapshot: Clone;

    /// Creates a snapshot of the current state
    fn snapshot(&self) -> Self::Snapshot;

    /// Restores state from a snapshot
    fn restore(&mut self, snapshot: Self::Snapshot);
}

/// Trait for entities that can be selected in the UI
pub trait Selectable {
    /// Returns whether this entity is currently selected
    fn is_selected(&self) -> bool;

    /// Sets the selection state
    fn set_selected(&mut self, selected: bool);

    /// Toggles the selection state
    fn toggle_selected(&mut self) {
        self.set_selected(!self.is_selected());
    }
}
