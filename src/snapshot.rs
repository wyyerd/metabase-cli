use std::error::Error;

use crate::model::ChartType;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Snapshot {
    Dashboard(DashboardSnapshot),
    Question(QuestionSnapshot),
}

#[derive(Deserialize, Serialize)]
pub struct DashboardSnapshot {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub parameters: SerializedSnapshot,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<CollectionDependency>,

    pub cards: Vec<CardSnapshot>,
}

#[derive(Deserialize, Serialize)]
pub struct CardSnapshot {
    pub size_x: i32,
    pub size_y: i32,
    pub row: i32,
    pub col: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<QuestionDependency>,

    pub parameters: SerializedSnapshot,

    pub settings: SerializedSnapshot,
}

#[derive(Deserialize, Serialize)]
pub struct QuestionSnapshot {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub display: ChartType,

    pub query: SerializedSnapshot,

    pub settings: SerializedSnapshot,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<CollectionDependency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseDependency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableDependency>,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedSnapshot(serde_json::Value);

impl SerializedSnapshot {
    pub fn new(input: &str) -> Result<Self, Box<dyn Error>> {
        let value = serde_json::from_str(input)?;
        Ok(SerializedSnapshot(value))
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self.0).expect("Bug! Failed to stringify snapshot.")
    }
}

#[derive(Deserialize, Serialize)]
pub struct CollectionDependency {
    pub collection: String,
}

#[derive(Deserialize, Serialize)]
pub struct QuestionDependency {
    pub question: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DatabaseDependency {
    pub database: String,
}

#[derive(Deserialize, Serialize)]
pub struct TableDependency {
    pub table: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

impl TableDependency {
    pub fn schema_or(&self, default: &'static str) -> &str {
        self.schema.as_ref().map(|x| x.as_str()).unwrap_or(default)
    }
}

#[derive(Deserialize, Serialize)]
pub struct FieldDependency {
    pub field: String,
}

/*
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum QuerySnapshot {
    Query(BuilderQuerySnapshot),
    Native(NativeQuerySnapshot),
}

#[derive(Deserialize, Serialize)]
pub struct NativeQuerySnapshot {
}

#[derive(Deserialize, Serialize)]
pub struct BuilderQuerySnapshot {
    pub table: TableDependency,

    pub fields: Vec<FieldDependency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterSnapshot>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<OrderBySnapshot>>,
}

#[derive(Deserialize, Serialize)]
pub struct OrderBySnapshot {
    pub field: FieldDependency,
    pub order: Order,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Order {
    Asc,
    Desc
}

#[derive(Deserialize, Serialize)]
pub enum FilterSnapshot {
    #[serde(rename = "or")]
    Or(Vec<FilterSnapshot>),
    #[serde(rename = "and")]
    And(Vec<FilterSnapshot>),
    #[serde(rename = "=")]
    Equal(FieldDependency, serde_json::Value),
    #[serde(rename = "non-null")]
    NotNull(FieldDependency),
}
*/
