use std::collections::{hash_map, HashMap};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{definition::osrs::FetchDefinition, Cache};

use super::{inv_def::InventoryDefinition, varbit_def::VarbitDefinition};

/// Loads all inventory definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct InventoryLoader(HashMap<u16, InventoryDefinition>);

impl_osrs_loader!(InventoryLoader, InventoryDefinition, index_id: 2, archive_id: 5);

/// Loads all varbit definitions from the current cache.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct VarbitLoader(HashMap<u16, VarbitDefinition>);

impl_osrs_loader!(VarbitLoader, VarbitDefinition, index_id: 2, archive_id: 14);
