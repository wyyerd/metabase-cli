use std::collections::BTreeMap;
use std::error::Error;

use chrono::{DateTime, Utc};
use diesel::PgConnection;
use diesel::prelude::*;

use crate::schema;

pub struct Cache {
    collections: BTreeMap<i32, Collection>,
    databases: BTreeMap<i32, Database>,
    tables: BTreeMap<i32, DbTable>,
    fields: BTreeMap<i32, DbField>,
}

impl Cache {
    pub fn load(conn: &PgConnection) -> Result<Cache, Box<dyn Error>> {
        let collections = schema::collection::table
            .get_results::<Collection>(conn)?
            .into_iter()
            .map(|row| (row.id, row))
            .collect();
        let databases = schema::metabase_database::table
            .get_results::<Database>(conn)?
            .into_iter()
            .map(|row| (row.id, row))
            .collect();
        let tables = schema::metabase_table::table
            .get_results::<DbTable>(conn)?
            .into_iter()
            .map(|row| (row.id, row))
            .collect();
        let fields = schema::metabase_field::table
            .get_results::<DbField>(conn)?
            .into_iter()
            .map(|row| (row.id, row))
            .collect();
        Ok(Cache { collections, databases, tables, fields })
    }

    pub fn collection(&self, id: i32) -> Result<&Collection, Box<dyn Error>> {
        self.collections.get(&id).ok_or(crate::error!("could not find collection (id: {})", id))
    }
    pub fn database(&self, id: i32) -> Result<&Database, Box<dyn Error>> {
        self.databases.get(&id).ok_or(crate::error!("could not find database (id: {})", id))
    }
    pub fn table(&self, id: i32) -> Result<&DbTable, Box<dyn Error>> {
        self.tables.get(&id).ok_or(crate::error!("could not find table (id: {})", id))
    }
    pub fn field(&self, id: i32) -> Result<&DbField, Box<dyn Error>> {
        self.fields.get(&id).ok_or(crate::error!("could not find field (id: {})", id))
    }
    pub fn table_field(&self, table_id: i32, field: &str) -> Result<&DbField, Box<dyn Error>> {
        self.fields
            .values()
            .filter(|x| x.table_id == table_id && x.name == field)
            .next()
            .ok_or(crate::error!("could not find field on table (table: {}, field: {})", table_id, field))
    }
    pub fn search_collections(&self, collection: &str) -> Vec<&Collection> {
        self.collections.values().filter(|x| x.name == collection).collect()
    }
    pub fn search_databases(&self, database: &str) -> Vec<&Database> {
        self.databases.values().filter(|x| x.name == database).collect()
    }
    pub fn search_tables(&self, table: &str, schema: &str) -> Vec<&DbTable> {
        self.tables.values().filter(|x| x.name == table && x.schema.as_ref().map(|y| y.as_str()).unwrap_or("public") == schema).collect()
    }
}

/*
use diesel::{
    deserialize::FromSql,
    serialize::ToSql,
    sql_types::String,
};

macro_rules! impl_diesel_String {
    ($typename:ty) => {
        impl<DB> FromSql<String, DB> for $typename
            where DB: diesel::backend::Backend,
                  *const str: FromSql<String, DB>
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
                let ptr: *const str = FromSql::from_sql(bytes)?;
                let text: &str = unsafe { &*ptr }; // avoid alloc, `String` must not longer than `bytes`
                let value = text.parse::<$typename>()?;
                Ok(value)
            }
        }
        impl<DB> ToSql<String, DB> for $typename
            where DB: diesel::backend::Backend
        {
            fn to_sql<W>(&self, out: &mut diesel::serialize::Output<W, DB>) -> diesel::serialize::Result
                where W: ::std::io::Write
            {
                out.write_all(self.to_string().as_bytes())?;
                Ok(diesel::serialize::IsNull::No)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Display, EnumString, FromSqlRow, AsExpression)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    Scalar,
    Table,
    Pie,
    Bar,
}
impl_diesel_String!(ChartType);

#[derive(Debug, Deserialize, Serialize, Display, EnumString, FromSqlRow, AsExpression)]
#[serde(rename_all = "snake_case")]
pub enum QueryType {
    Query,
    Native,
}
impl_diesel_String!(QueryType);
*/

pub type ChartType = String;
pub type QueryType = String;

#[derive(Debug, Queryable)]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub archived: bool,
    pub location: String,
    pub personal_owner_id: Option<i32>,
    pub slug: String,
}

#[derive(Debug, Queryable)]
pub struct Dashboard {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
    pub creator_id: i32,
    pub parameters: String,
    pub points_of_interest: Option<String>,
    pub caveats: Option<String>,
    pub show_in_getting_started: bool,
    pub public_uuid: Option<String>,
    pub made_public_by_id: Option<i32>,
    pub enable_embedding: bool,
    pub embedding_params: Option<String>,
    pub archived: bool,
    pub position: Option<i32>,
    pub collection_id: Option<i32>,
    pub collection_position: Option<i16>,
}

#[derive(Debug, Queryable)]
pub struct DashboardCard {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub size_x: i32,
    pub size_y: i32,
    pub row: i32,
    pub col: i32,
    pub card_id: Option<i32>,
    pub dashboard_id: i32,
    pub parameter_mappings: String,
    pub visualization_settings: String,
}

#[derive(Debug, Queryable)]
pub struct Question {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
    pub display: ChartType,
    pub dataset_query: String,
    pub visualization_settings: String,
    pub creator_id: i32,
    pub database_id: Option<i32>,
    pub table_id: Option<i32>,
    pub query_type: Option<QueryType>,
    pub archived: bool,
    pub collection_id: Option<i32>,
    pub public_uuid: Option<String>,
    pub made_public_by_id: Option<i32>,
    pub enable_embedding: bool,
    pub embedding_params: Option<String>,
    pub cache_ttl: Option<i32>,
    pub result_metadata: Option<String>,
    pub read_permissions: Option<String>,
    pub collection_position: Option<i16>,
}

#[derive(Debug, Queryable)]
pub struct Database {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
    pub details: Option<String>,
    pub engine: String,
    pub is_sample: bool,
    pub is_full_sync: bool,
    pub points_of_interest: Option<String>,
    pub caveats: Option<String>,
    pub metadata_sync_schedule: String,
    pub cache_field_values_schedule: String,
    pub timezone: Option<String>,
    pub is_on_demand: bool,
    pub options: Option<String>,
}

#[derive(Debug, Queryable)]
pub struct DbTable {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub rows: Option<i64>,
    pub description: Option<String>,
    pub entity_name: Option<String>,
    pub entity_type: Option<String>,
    pub active: bool,
    pub db_id: i32,
    pub display_name: Option<String>,
    pub visibility_type: Option<String>,
    pub schema: Option<String>,
    pub points_of_interest: Option<String>,
    pub caveats: Option<String>,
    pub show_in_getting_started: bool,
    pub fields_hash: Option<String>,
}

impl DbTable {
    pub fn schema_or(&self, default: &'static str) -> &str {
        self.schema.as_ref().map(|x| x.as_str()).unwrap_or(default)
    }
}

#[derive(Debug, Queryable)]
pub struct DbField {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub base_type: String,
    pub special_type: Option<String>,
    pub active: bool,
    pub description: Option<String>,
    pub preview_display: bool,
    pub position: i32,
    pub table_id: i32,
    pub parent_id: Option<i32>,
    pub display_name: Option<String>,
    pub visibility_type: String,
    pub fk_target_field_id: Option<i32>,
    pub last_analyzed: Option<DateTime<Utc>>,
    pub points_of_interest: Option<String>,
    pub caveats: Option<String>,
    pub fingerprint: Option<String>,
    pub fingerprint_version: i32,
    pub database_type: String,
    pub has_field_values: Option<String>,
    pub settings: Option<String>,
}
