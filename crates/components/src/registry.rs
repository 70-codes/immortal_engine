//! Component Registry
//!
//! The ComponentRegistry is a central store for all available component definitions.
//! It allows registering, querying, and instantiating components.

use std::collections::HashMap;
use std::sync::Arc;

use imortal_core::{ComponentCategory, EngineError, EngineResult};
use imortal_ir::Node;

use crate::definition::ComponentDefinition;
use crate::definitions::{auth, data, api, storage, logic};
use crate::traits::ComponentFactory;

/// Registry of all available component definitions
#[derive(Debug)]
pub struct ComponentRegistry {
    /// All registered components, keyed by their ID
    components: HashMap<String, Arc<ComponentDefinition>>,

    /// Components grouped by category for quick lookup
    by_category: HashMap<ComponentCategory, Vec<String>>,

    /// Whether the registry has been initialized with built-in components
    initialized: bool,
}

impl ComponentRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            by_category: HashMap::new(),
            initialized: false,
        }
    }

    /// Create a registry with all built-in components registered
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register_builtins();
        registry
    }

    /// Register all built-in component definitions
    pub fn register_builtins(&mut self) {
        if self.initialized {
            return;
        }

        // Auth components
        self.register(auth::login_component());
        self.register(auth::register_component());
        self.register(auth::logout_component());
        self.register(auth::session_component());

        // Data components
        self.register(data::entity_component());
        self.register(data::collection_component());
        self.register(data::query_component());

        // API components
        self.register(api::rest_endpoint_component());
        self.register(api::graphql_endpoint_component());
        self.register(api::websocket_component());

        // Storage components
        self.register(storage::database_component());
        self.register(storage::cache_component());
        self.register(storage::file_storage_component());

        // Logic components
        self.register(logic::validator_component());
        self.register(logic::transformer_component());
        self.register(logic::condition_component());

        self.initialized = true;
    }

    /// Register a component definition
    pub fn register(&mut self, definition: ComponentDefinition) {
        let id = definition.id.clone();
        let category = definition.category;

        // Add to main map
        self.components.insert(id.clone(), Arc::new(definition));

        // Add to category index
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id);
    }

    /// Unregister a component definition
    pub fn unregister(&mut self, id: &str) -> Option<Arc<ComponentDefinition>> {
        if let Some(def) = self.components.remove(id) {
            // Remove from category index
            if let Some(ids) = self.by_category.get_mut(&def.category) {
                ids.retain(|i| i != id);
            }
            Some(def)
        } else {
            None
        }
    }

    /// Get a component definition by ID
    pub fn get(&self, id: &str) -> Option<&ComponentDefinition> {
        self.components.get(id).map(|arc| arc.as_ref())
    }

    /// Get a component definition as Arc for shared ownership
    pub fn get_arc(&self, id: &str) -> Option<Arc<ComponentDefinition>> {
        self.components.get(id).cloned()
    }

    /// Check if a component is registered
    pub fn contains(&self, id: &str) -> bool {
        self.components.contains_key(id)
    }

    /// Get all component definitions
    pub fn all(&self) -> impl Iterator<Item = &ComponentDefinition> {
        self.components.values().map(|arc| arc.as_ref())
    }

    /// Get all component IDs
    pub fn all_ids(&self) -> impl Iterator<Item = &String> {
        self.components.keys()
    }

    /// Get the number of registered components
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Get all components in a specific category
    pub fn by_category(&self, category: ComponentCategory) -> Vec<&ComponentDefinition> {
        self.by_category
            .get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all components matching a search query (searches id, name, description)
    pub fn search(&self, query: &str) -> Vec<&ComponentDefinition> {
        let query_lower = query.to_lowercase();
        self.components
            .values()
            .filter(|def| {
                def.id.to_lowercase().contains(&query_lower)
                    || def.name.to_lowercase().contains(&query_lower)
                    || def.description.to_lowercase().contains(&query_lower)
            })
            .map(|arc| arc.as_ref())
            .collect()
    }

    /// Get all categories that have at least one component
    pub fn categories(&self) -> Vec<ComponentCategory> {
        self.by_category
            .iter()
            .filter(|(_, ids)| !ids.is_empty())
            .map(|(cat, _)| *cat)
            .collect()
    }

    /// Instantiate a component as a Node
    pub fn instantiate(&self, component_id: &str) -> EngineResult<Node> {
        let definition = self.get(component_id).ok_or_else(|| {
            EngineError::ComponentNotFound(component_id.to_string())
        })?;

        Ok(definition.create_node())
    }

    /// Instantiate a component with a custom name
    pub fn instantiate_with_name(&self, component_id: &str, name: &str) -> EngineResult<Node> {
        let mut node = self.instantiate(component_id)?;
        node.name = name.to_string();
        Ok(node)
    }

    /// Instantiate a component at a specific position
    pub fn instantiate_at(&self, component_id: &str, x: f32, y: f32) -> EngineResult<Node> {
        let mut node = self.instantiate(component_id)?;
        node.position.x = x;
        node.position.y = y;
        Ok(node)
    }

    /// Get component statistics
    pub fn stats(&self) -> RegistryStats {
        let mut stats = RegistryStats {
            total: self.components.len(),
            by_category: HashMap::new(),
        };

        for (category, ids) in &self.by_category {
            stats.by_category.insert(*category, ids.len());
        }

        stats
    }

    /// Clear all registered components
    pub fn clear(&mut self) {
        self.components.clear();
        self.by_category.clear();
        self.initialized = false;
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}

impl Clone for ComponentRegistry {
    fn clone(&self) -> Self {
        Self {
            components: self.components.clone(),
            by_category: self.by_category.clone(),
            initialized: self.initialized,
        }
    }
}

/// Statistics about the component registry
#[derive(Debug, Clone, Default)]
pub struct RegistryStats {
    /// Total number of components
    pub total: usize,
    /// Components per category
    pub by_category: HashMap<ComponentCategory, usize>,
}

impl RegistryStats {
    /// Get count for a specific category
    pub fn category_count(&self, category: ComponentCategory) -> usize {
        self.by_category.get(&category).copied().unwrap_or(0)
    }
}

/// Global registry accessor (for convenience)
pub mod global {
    use super::*;
    use std::sync::OnceLock;

    static REGISTRY: OnceLock<ComponentRegistry> = OnceLock::new();

    /// Get the global component registry
    pub fn registry() -> &'static ComponentRegistry {
        REGISTRY.get_or_init(ComponentRegistry::with_builtins)
    }

    /// Get a component definition from the global registry
    pub fn get_component(id: &str) -> Option<&'static ComponentDefinition> {
        registry().get(id)
    }

    /// Instantiate a component from the global registry
    pub fn instantiate(component_id: &str) -> EngineResult<Node> {
        registry().instantiate(component_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ComponentRegistry::new();
        assert!(registry.is_empty());

        let registry = ComponentRegistry::with_builtins();
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_register_component() {
        let mut registry = ComponentRegistry::new();
        let def = ComponentDefinition::new("test.component", "Test", ComponentCategory::Custom);

        registry.register(def);

        assert!(registry.contains("test.component"));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_unregister_component() {
        let mut registry = ComponentRegistry::new();
        let def = ComponentDefinition::new("test.component", "Test", ComponentCategory::Custom);

        registry.register(def);
        assert!(registry.contains("test.component"));

        let removed = registry.unregister("test.component");
        assert!(removed.is_some());
        assert!(!registry.contains("test.component"));
    }

    #[test]
    fn test_get_by_category() {
        let mut registry = ComponentRegistry::new();

        let auth1 = ComponentDefinition::new("auth.login", "Login", ComponentCategory::Auth);
        let auth2 = ComponentDefinition::new("auth.register", "Register", ComponentCategory::Auth);
        let data1 = ComponentDefinition::new("data.entity", "Entity", ComponentCategory::Data);

        registry.register(auth1);
        registry.register(auth2);
        registry.register(data1);

        let auth_components = registry.by_category(ComponentCategory::Auth);
        assert_eq!(auth_components.len(), 2);

        let data_components = registry.by_category(ComponentCategory::Data);
        assert_eq!(data_components.len(), 1);
    }

    #[test]
    fn test_search() {
        let mut registry = ComponentRegistry::new();

        let def1 = ComponentDefinition::new("auth.login", "Login Form", ComponentCategory::Auth)
            .with_description("User login component");
        let def2 = ComponentDefinition::new("auth.register", "Register", ComponentCategory::Auth)
            .with_description("User registration");

        registry.register(def1);
        registry.register(def2);

        let results = registry.search("login");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "auth.login");

        let results = registry.search("user");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_instantiate() {
        let registry = ComponentRegistry::with_builtins();

        let node = registry.instantiate("auth.login");
        assert!(node.is_ok());

        let node = node.unwrap();
        assert_eq!(node.component_type, "auth.login");
        assert_eq!(node.category, ComponentCategory::Auth);
    }

    #[test]
    fn test_instantiate_at() {
        let registry = ComponentRegistry::with_builtins();

        let node = registry.instantiate_at("auth.login", 100.0, 200.0).unwrap();
        assert_eq!(node.position.x, 100.0);
        assert_eq!(node.position.y, 200.0);
    }

    #[test]
    fn test_stats() {
        let registry = ComponentRegistry::with_builtins();
        let stats = registry.stats();

        assert!(stats.total > 0);
        assert!(stats.category_count(ComponentCategory::Auth) > 0);
    }

    #[test]
    fn test_global_registry() {
        let registry = global::registry();
        assert!(!registry.is_empty());

        let component = global::get_component("auth.login");
        assert!(component.is_some());
    }
}
