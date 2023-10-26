use std::{collections::HashMap, any::Any};

pub trait Record {
    fn get_primary_key(&self) -> String;
    fn set_primary_key(&self) -> String;
}

pub struct Table<T> {
    pub name: String,
    records: HashMap<String, T>,
}

impl<T: Record> Table<T> {
    /* 
    make each function on its own thread with records being an Arc<RwLock<>>
    will need a while locked loop in each funciton
     */

    pub fn new(name: String, records: Vec<T>) -> Table<T> {
        let mut map: HashMap<String, T> = HashMap::new();

        for record in records {
            map.insert(record.get_primary_key(), record);
        }

        Table { 
            name: name, 
            records: map 
        }

    }




    pub fn insert_record(&mut self, record: T) -> bool {

        match self.records.get(&record.get_primary_key()) {
            Some(_) => return false,
            None => {
                self.records.insert(record.get_primary_key(), record);
                return true;
            },
        }
        
        
    }





}