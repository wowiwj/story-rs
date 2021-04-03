use crate::query::{Filter, FilterAble};

#[derive(Debug)]
pub struct Select {
    table: String,
    filter: Filter,
    fields: Option<Vec<String>>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl Select {
    pub fn new(table: &str) -> Self {
        Select {
            table: table.into(),
            filter: Filter::new(),
            fields: None,
            limit: None,
            offset: None,
        }
    }

    pub fn fields(&mut self, fields: Vec<&str>) -> &mut Self {
        self.fields = Some(fields.into_iter().map(|field| field.into()).collect());
        self
    }

    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.filter.filter(filter);
        self
    }

    pub fn take(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn skip(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }



    fn build_fields(&self) -> String {
        self.fields
            .as_ref()
            .map_or_else(|| "*".into(), |fields| fields.join(","))
    }


    pub fn build(&self) -> String {
        let mut raw = format!("select {} from {}", self.build_fields(), self.table);
        raw += self.filter.build_filters().as_str();
        raw.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::query::QueryBuilder;

    #[test]
    pub fn test_select() {
        let mut query = QueryBuilder::select("users");

        query.filter("name = $1")
            .filter("age > $2")
            .filter("title in $3")
            .fields(vec!["name", "age"]);
        query.take(1).skip(1);

        let sql = query.build();

        assert_eq!(sql, "select name,age from users where name = $1 and age > $2 and title in $3");
    }

}