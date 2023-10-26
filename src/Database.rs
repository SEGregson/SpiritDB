use std::{collections::HashMap, any::Any};

use crate::table::Table;

struct Database {
    tables: HashMap<String, Table<Box<dyn Any>>>
}