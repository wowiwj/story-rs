
use std::collections::HashMap;

#[derive(Debug)]
pub struct Insert {
    table: String,
    values: HashMap<String, String>,
}

impl Insert {
    pub fn new(table: &str) -> Insert {
        Self {
            table: table.into(),
            values: HashMap::new(),
        }
    }


    pub fn set(&mut self, field: &str, value: &str) -> &mut Self {
        self.values.insert(field.into(), value.into());
        self
    }

    pub fn build(&self) -> String {
        let mut raw = format!("insert into {}", self.table);
        let columns: Vec<String> = self.values.keys().map(|k|k.into()).collect();
        raw += format!(" ({})",&columns.join(",")).as_str();
        let values: Vec<String> = self.values
            .iter().map(|(k,_)| self.values.get(k).unwrap().into() )
            .collect();
        raw += format!(" values({})",&values.join(",")).as_str();
        raw.into()
    }
}


#[cfg(test)]
mod test {
    
    use crate::query::QueryBuilder;


    #[test]
    pub fn test_insert() {
        let mut insert = QueryBuilder::insert("users");
        insert.set("name", "$1");
        insert.set("age", "$2");
        let query = insert.build();
        let assert1 = "insert into users (age,name) values($2,$1)" == query;
        let assert2 = "insert into users (name,age) values($1,$2)" == query;
        assert!(assert1 || assert2);
    }
}