use quaint::Value;
use sqlx::mysql::MySqlArguments;
use sqlx::{ Arguments};

pub struct QueryXArgument {
    arguments: MySqlArguments
}

impl QueryXArgument {
    pub fn new() -> Self {
        Self {
            arguments: MySqlArguments::default()
        }
    }

    pub fn arguments(mut self, params: Vec<Value>) -> MySqlArguments {
        params.into_iter().for_each(|param| {
            self.convert_add(param);
        });
        self.arguments
    }

    fn convert_add(&mut self, value: Value) {
        if value.is_text() || value.is_bytes() {
            self.arguments.add(value.as_str());
            return;
        }
        if value.is_integer() {
            self.arguments.add(value.as_i64());
            return;
        }
        if value.is_bool() {
            self.arguments.add(value.as_bool());
            return;
        }
    }
}