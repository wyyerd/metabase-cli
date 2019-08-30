use std::error::Error;

use chrono::Utc;
// use diesel::Connection;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use regex::{Captures, Regex};

use crate::model::*;
use crate::schema::*;
use crate::snapshot::*;

pub fn import(database_url: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let conn = PgConnection::establish(&database_url)?;
    let cache = Cache::load(&conn)?;
    let snapshot_bytes = std::fs::read(filename)?;
    let snapshot: Snapshot = serde_json::from_slice(&snapshot_bytes)?;
    match snapshot {
        Snapshot::Dashboard(dashboard) => {
            let collection_id = if let Some(collection_dep) = &dashboard.collection {
                let collection = cache.search_collections(&collection_dep.collection)
                    .into_iter()
                    .filter(|x| !x.archived)
                    .next()
                    .ok_or(crate::error!("collection doesn't exist (collection: {})", collection_dep.collection))?;
                let already_exists: bool = 
                    diesel::select(
                        diesel::dsl::exists(
                            report_dashboard::table
                                .filter(report_dashboard::name.eq(&dashboard.name))
                                .filter(report_dashboard::collection_id.eq(collection.id))
                        )
                    )
                    .get_result(&conn)?;
                if already_exists {
                    return Err(crate::error!("a dashboard with the same name and collection already exists"));
                }
                Some(collection.id)
            } else {
                let already_exists: bool = 
                    diesel::select(
                        diesel::dsl::exists(
                            report_dashboard::table
                                .filter(report_dashboard::name.eq(&dashboard.name))
                                .filter(report_dashboard::collection_id.is_null())
                        )
                    )
                    .get_result(&conn)?;
                if already_exists {
                    return Err(crate::error!("a dashboard with the same name and collection already exists"));
                }
                None
            };


            let parameters = dashboard.parameters.to_string();
            let timestamp = Utc::now();
            conn.transaction::<_, Box<dyn Error>, _>(|| {
                let dashboard_id = 
                    diesel::insert_into(report_dashboard::table)
                        .values((
                            report_dashboard::name.eq(dashboard.name),
                            report_dashboard::description.eq(dashboard.description),
                            report_dashboard::creator_id.eq(1),
                            report_dashboard::created_at.eq(timestamp),
                            report_dashboard::updated_at.eq(timestamp),
                            report_dashboard::parameters.eq(parameters),
                            report_dashboard::show_in_getting_started.eq(false),
                            report_dashboard::enable_embedding.eq(false),
                            report_dashboard::archived.eq(false),
                            report_dashboard::collection_id.eq(collection_id),
                        ))
                        .returning(report_dashboard::id)
                        .get_result::<i32>(&conn)?;
                for card in dashboard.cards {
                    if let Some(question) = card.question {
                        let question = if let Some(collection) = &question.collection {
                            let collection = cache.search_collections(collection)
                                .into_iter()
                                .filter(|x| !x.archived)
                                .next()
                                .ok_or(crate::error!("collection doesn't exist (collection: {})", collection))?;
                            report_card::table
                                .filter(report_card::name.eq(&question.question))
                                .filter(report_card::collection_id.eq(collection.id))
                                .get_result::<Question>(&conn)
                                .optional()?
                                .ok_or(crate::error!("question doesn't exist (question: {})", question.question))?
                        } else {
                            report_card::table
                                .filter(report_card::name.eq(&question.question))
                                .filter(report_card::collection_id.is_null())
                                .get_result::<Question>(&conn)
                                .optional()?
                                .ok_or(crate::error!("question doesn't exist (question: {})", question.question))?
                        };
                        let table_id = question.table_id.ok_or(crate::error!("cannot restore card (unknown question table)"))?;
                        let table = cache.table(table_id)?;
                        let parameters = restore_parameters(&cache, &question, &table, card.parameters)?;
                        let settings = restore_settings(&cache, &table, card.settings)?;
                        diesel::insert_into(report_dashboardcard::table)
                            .values((
                                report_dashboardcard::dashboard_id.eq(dashboard_id),
                                report_dashboardcard::created_at.eq(timestamp),
                                report_dashboardcard::updated_at.eq(timestamp),
                                report_dashboardcard::sizeX.eq(card.size_x),
                                report_dashboardcard::sizeY.eq(card.size_y),
                                report_dashboardcard::row.eq(card.row),
                                report_dashboardcard::col.eq(card.col),
                                report_dashboardcard::card_id.eq(question.id),
                                report_dashboardcard::parameter_mappings.eq(parameters),
                                report_dashboardcard::visualization_settings.eq(settings),
                            ))
                            .execute(&conn)?;
                    }
                }
                Ok(())
            })?;
        }
        Snapshot::Question(question) => {
            let collection_dep = question.collection
                .ok_or(crate::error!("cannot restore question (missing collection dependency)"))?;
            let collection = cache.search_collections(&collection_dep.collection)
                .into_iter()
                .filter(|x| !x.archived)
                .next()
                .ok_or(crate::error!("collection doesn't exist (collection: {})", collection_dep.collection))?;
            let already_exists: bool = 
                diesel::select(
                    diesel::dsl::exists(
                        report_card::table
                            .filter(report_card::name.eq(&question.name))
                            .filter(report_card::collection_id.eq(collection.id))
                    )
                )
                .get_result(&conn)?;
            if already_exists {
                return Err(crate::error!("a question with the same name and collection already exists"));
            }

            let table_dep = question.table
                .ok_or(crate::error!("cannot restore question (missing table dependency)"))?;
            let table = discover_table(&cache, &table_dep, &question.database)
                .ok_or(crate::error!("table doesn't exist (tablename: {}.{})", table_dep.schema_or("public"), &table_dep.table))?;
            let query = restore_query(&cache, &table, question.query)?;
            let settings = restore_settings(&cache, &table, question.settings)?;
            let timestamp = Utc::now();
            diesel::insert_into(report_card::table)
                .values((
                    report_card::name.eq(question.name),
                    report_card::description.eq(question.description),
                    report_card::display.eq(question.display),
                    report_card::dataset_query.eq(query),
                    report_card::visualization_settings.eq(settings),
                    report_card::creator_id.eq(1),
                    report_card::created_at.eq(timestamp),
                    report_card::updated_at.eq(timestamp),
                    report_card::database_id.eq(table.db_id),
                    report_card::table_id.eq(table.id),
                    report_card::query_type.eq("query"),
                    report_card::archived.eq(false),
                    report_card::collection_id.eq(collection.id),
                    report_card::enable_embedding.eq(false),
                ))
                .execute(&conn)?;
        }
    }
    Ok(())
}

pub fn export(database_url: &str, source: &str, id: &str) -> Result<(), Box<dyn Error>> {
    let conn = PgConnection::establish(&database_url)?;
    let cache = Cache::load(&conn)?;
    match source {
        "question" => {
            let id: i32 = id.parse()?;
            let question = report_card::table.find(id).get_result::<Question>(&conn)?;
            let collection = question.collection_id.map(|id| cache.collection(id)).transpose()?;
            let database = question.database_id.map(|id| cache.database(id)).transpose()?;
            let table = question.table_id.map(|id| cache.table(id)).transpose()?;
            let snapshot = Snapshot::Question(QuestionSnapshot {
                name: question.name,
                description: question.description,
                display: question.display,
                query: snapshot_query(&cache, &question.dataset_query)?,
                settings: snapshot_settings(&cache, &question.visualization_settings)?,
                collection: collection.map(|col| CollectionDependency { collection: col.name.clone() }),
                database: database.map(|db| DatabaseDependency { database: db.name.clone() }),
                table: table.map(|tbl| TableDependency { table: tbl.name.clone(), schema: tbl.schema.clone() }),
            });
            println!("{}", serde_json::to_string(&snapshot)?);
        }
        "dashboard" => {
            let id: i32 = id.parse()?;
            let dashboard = report_dashboard::table.find(id).get_result::<Dashboard>(&conn)?;
            let collection = dashboard.collection_id.map(|id| cache.collection(id)).transpose()?;
            let cards = report_dashboardcard::table
                .filter(report_dashboardcard::dashboard_id.eq(id))
                .get_results::<DashboardCard>(&conn)?;
            let mut card_snapshots = Vec::new();
            for card in cards {
                let question = if let Some(question_id) = card.card_id {
                    let question = report_card::table.find(question_id).get_result::<Question>(&conn)?;
                    let collection = question.collection_id.map(|id| cache.collection(id)).transpose()?;
                    Some(QuestionDependency {
                        question: question.name,
                        collection: collection.map(|col| col.name.clone()),
                    })
                } else {
                    None
                };
                card_snapshots.push(CardSnapshot {
                    size_x: card.size_x,
                    size_y: card.size_y,
                    row: card.row,
                    col: card.col,
                    question,
                    parameters: snapshot_paremeters(&cache, &card.parameter_mappings)?,
                    settings: snapshot_settings(&cache, &card.visualization_settings)?,
                });
            }
            let snapshot = Snapshot::Dashboard(DashboardSnapshot {
                name: dashboard.name,
                description: dashboard.description,
                parameters: SerializedSnapshot::new(&dashboard.parameters)?,
                collection: collection.map(|col| CollectionDependency { collection: col.name.clone() }),
                cards: card_snapshots,
            });
            println!("{}", serde_json::to_string(&snapshot)?);
        }
        // "collection" => { ... }
        // "database" => { ... }
        // "table" => { ... }
        // "table-segments" => { ... }
        // "table-metrics" => { ... }
        // "segment" => { ... }
        // "metric" => { ... }
        _ => return Err(crate::error!("unknown datasource `{}`", source)),
    }
    Ok(())
}

fn discover_table<'a>(cache: &'a Cache, tbl: &TableDependency, db: &Option<DatabaseDependency>) -> Option<&'a DbTable> {
    let database = db.as_ref().and_then(|db| cache.search_databases(&db.database).into_iter().next());
    let tables = cache.search_tables(&tbl.table, &tbl.schema_or("public"));
    if let Some(database) = database {
        if let Some(table) = tables.iter().find(|x| x.db_id == database.id) {
            return Some(table);
        }
    }
    tables.iter().find(|x| x.active && x.visibility_type.is_none())
        .or_else(|| tables.iter().find(|x| x.active))
        .or_else(|| tables.iter().next())
        .map(|&x| x)
}

// FIXME: Use `regex::escape` on names
fn restore_query(cache: &Cache, table: &DbTable, snapshot: SerializedSnapshot) -> Result<String, Box<dyn Error>> {
    let mut restored = snapshot.to_string();
    let database_re = Regex::new(r#""snapshot:database""#).unwrap();
    restored = database_re.replace_all(&restored, |_: &Captures| table.db_id.to_string()).to_string();
    let table_re = Regex::new(&format!(r#""snapshot:table:{}.{}""#, table.schema_or("public"), table.name)).unwrap();
    restored = table_re.replace_all(&restored, |_: &Captures| table.id.to_string()).to_string();
    let field_re = Regex::new(r#""snapshot:field:([A-Za-z_]+)""#).unwrap();
    let field_names = field_re.captures_iter(&restored).map(|x| x[1].to_string()).collect::<Vec<_>>();
    for field_name in field_names {
        let re = Regex::new(&format!(r#""snapshot:field:{}""#, field_name)).unwrap();
        let field = cache.table_field(table.id, &field_name)?;
        restored = re.replace_all(&restored, |_: &Captures| format!(r#"["field-id",{}]"#, field.id)).to_string();
    }
    Ok(restored)
}

fn restore_parameters(cache: &Cache, question: &Question, table: &DbTable, snapshot: SerializedSnapshot) -> Result<String, Box<dyn Error>> {
    let mut restored = snapshot.to_string();
    let question_re = Regex::new(r#""snapshot:question""#).unwrap();
    restored = question_re.replace_all(&restored, |_: &Captures| question.id.to_string()).to_string();
    let field_re = Regex::new(r#""snapshot:field:([A-Za-z_]+)""#).unwrap();
    let field_names = field_re.captures_iter(&restored).map(|x| x[1].to_string()).collect::<Vec<_>>();
    for field_name in field_names {
        let re = Regex::new(&format!(r#""snapshot:field:{}""#, field_name)).unwrap();
        let field = cache.table_field(table.id, &field_name)?;
        restored = re.replace_all(&restored, |_: &Captures| format!(r#"["field-id",{}]"#, field.id)).to_string();
    }
    Ok(restored)
}

fn restore_settings(cache: &Cache, table: &DbTable, snapshot: SerializedSnapshot) -> Result<String, Box<dyn Error>> {
    let mut restored = snapshot.to_string();
    let field_re = Regex::new(r#""snapshot:field:([A-Za-z_]+)""#).unwrap();
    let field_names = field_re.captures_iter(&restored).map(|x| x[1].to_string()).collect::<Vec<_>>();
    for field_name in field_names {
        let re = Regex::new(&format!(r#""snapshot:field:{}""#, field_name)).unwrap();
        let field = cache.table_field(table.id, &field_name)?;
        restored = re.replace_all(&restored, |_: &Captures| format!(r#"["field-id",{}]"#, field.id)).to_string();
    }
    let field_ref_re = Regex::new(r#""snapshot:ref:field:([A-Za-z_]+)""#).unwrap();
    let field_refs = field_ref_re.captures_iter(&restored).map(|x| x[1].to_string()).collect::<Vec<_>>();
    for field_name in field_refs {
        let re = Regex::new(&format!(r#""snapshot:ref:field:{}""#, field_name)).unwrap();
        let field = cache.table_field(table.id, &field_name)?;
        restored = re.replace_all(&restored, |_: &Captures| format!(r#""[\"ref\", [\"field-id\",{}]]""#, field.id)).to_string();
    }
    Ok(restored)
}

fn snapshot_query(cache: &Cache, raw: &str) -> Result<SerializedSnapshot, Box<dyn Error>> {
    let mut snapshot = raw.to_string();
    let database_re = Regex::new(r#""database":(\d+)"#).unwrap();
    snapshot = database_re.replace_all(&snapshot, |_: &Captures| r#""database":"snapshot:database""#).to_string();
    let table_re = Regex::new(r#""source-table":(\d+)"#).unwrap();
    let table_id = table_re.captures_iter(raw).next().map(|x| x[1].parse::<i32>().unwrap());
    if let Some(table_id) = table_id {
        let re = Regex::new(&format!(r#""source-table":{}"#, table_id)).unwrap();
        let table = cache.table(table_id)?;
        snapshot = re.replace_all(&snapshot, |_: &Captures| format!(r#""source-table":"snapshot:table:{}.{}""#, table.schema_or("public"), table.name)).to_string();
    }
    let field_re = Regex::new(r#"\["field-id",(\d+)\]"#).unwrap();
    let field_ids = field_re.captures_iter(raw).map(|x| x[1].parse::<i32>().unwrap()).collect::<Vec<_>>();
    for field_id in field_ids {
        let re = Regex::new(&format!(r#"\["field-id",{}\]"#, field_id)).unwrap();
        let field = cache.field(field_id)?;
        snapshot = re.replace_all(&snapshot, |_: &Captures| format!(r#""snapshot:field:{}""#, field.name)).to_string();
    }
    SerializedSnapshot::new(&snapshot)
}

fn snapshot_paremeters(cache: &Cache, raw: &str) -> Result<SerializedSnapshot, Box<dyn Error>> {
    let mut snapshot = raw.to_string();
    let question_re = Regex::new(r#""card_id":(\d+)"#).unwrap();
    snapshot = question_re.replace_all(&snapshot, |_: &Captures| r#""card_id":"snapshot:question""#).to_string();
    let field_re = Regex::new(r#"\["field-id",(\d+)\]"#).unwrap();
    let field_ids = field_re.captures_iter(raw).map(|x| x[1].parse::<i32>().unwrap()).collect::<Vec<_>>();
    for field_id in field_ids {
        let re = Regex::new(&format!(r#"\["field-id",{}\]"#, field_id)).unwrap();
        let field = cache.field(field_id)?;
        snapshot = re.replace_all(&snapshot, |_: &Captures| format!(r#""snapshot:field:{}""#, field.name)).to_string();
    }
    SerializedSnapshot::new(&snapshot)
}

fn snapshot_settings(cache: &Cache, raw: &str) -> Result<SerializedSnapshot, Box<dyn Error>> {
    let mut snapshot = raw.to_string();
    let field_re = Regex::new(r#"\["field-id",(\d+)\]"#).unwrap();
    let field_ids = field_re.captures_iter(raw).map(|x| x[1].parse::<i32>().unwrap()).collect::<Vec<_>>();
    for field_id in field_ids {
        let re = Regex::new(&format!(r#"\["field-id",{}\]"#, field_id)).unwrap();
        let field = cache.field(field_id)?;
        snapshot = re.replace_all(&snapshot, |_: &Captures| format!(r#""snapshot:field:{}""#, field.name)).to_string();
    }
    let field_ref_re = Regex::new(r#""\[\\"ref\\",\[\\"field-id\\",(\d+)\]\]""#).unwrap();
    let field_ref_ids = field_ref_re.captures_iter(raw).map(|x| x[1].parse::<i32>().unwrap()).collect::<Vec<_>>();
    for field_id in field_ref_ids {
        let re = Regex::new(&format!(r#""\[\\"ref\\",\[\\"field-id\\",{}\]\]""#, field_id)).unwrap();
        let field = cache.field(field_id)?;
        snapshot = re.replace_all(&snapshot, |_: &Captures| format!(r#""snapshot:ref:field:{}""#, field.name)).to_string();
    }
    SerializedSnapshot::new(&snapshot)
}

/*
fn snapshot_query(conn: &PgConnection, raw: &serde_json::Value) -> Result<QuerySnapshot, Box<dyn Error>> {
    let table_id = raw["query"]["source-table"].as_i64().ok_or(crate::error!("expected `question.query.query.source0table` to be an integer"))?;
    let table = metabase_table::table.find(table_id as i32).get_result::<DbTable>(conn)?;
    let mut query = BuilderQuerySnapshot {
        table: TableDependency { table: table.name, schema: table.schema },
        fields: Vec::new(),
        filter: None,
        limit: None,
        order_by: None,
    };
    let fields = raw["query"]["fields"].as_array().ok_or(crate::error!("expected `question.query.query.fields` to be an array"))?;
    for field in fields {
        match field[0].as_str() {
            Some("field-id") => {
                let field_id = field[1].as_i64().ok_or(crate::error!("expected `field-id` to be an integer"))?;
                let field = metabase_field::table.find(field_id as i32).get_result::<DbField>(conn)?;
                query.fields.push(FieldDependency { field: field.name });
            }
            Some(key) => Err(crate::error!("unsupported query field type `{}`", key))?,
            None => Err(crate::error!("expected query field type to be a string"))?,
        }
    }
    if !raw["query"]["filter"].is_null() {
        let filter = snapshot_filter(conn, &raw["query"]["filter"])?;
        query.filter = Some(filter);
    }
    if !raw["query"]["order-by"].is_null() {
        let order_by = snapshot_order_by(conn, &raw["query"]["order-by"])?;
        query.order_by = Some(order_by);
    }
    query.limit = raw["query"]["limit"].as_i64();
    Ok(QuerySnapshot::Query(query))
}

fn snapshot_filter(conn: &PgConnection, raw: &serde_json::Value) -> Result<FilterSnapshot, Box<dyn Error>> {
    let mut s_expr = raw.as_array().ok_or(crate::error!("expected `filter` to be an array"))?.into_iter();
    match s_expr.next().and_then(|x| x.as_str()) {
        Some("and") => Ok(FilterSnapshot::And(s_expr.map(|x| snapshot_filter(conn, x)).collect::<Result<_, _>>()?)),
        Some("or") => Ok(FilterSnapshot::Or(s_expr.map(|x| snapshot_filter(conn, x)).collect::<Result<_, _>>()?)),
        Some("=") => {
            let field_data = s_expr.next().ok_or(crate::error!("missing field in `=` filter"))?;
            let field_id = field_data[1].as_i64().ok_or(crate::error!("expected `field-id` to be an integer"))?;
            let field = metabase_field::table.find(field_id as i32).get_result::<DbField>(conn)?;
            let value = s_expr.next().ok_or(crate::error!("missing value in `=` filter"))?;
            Ok(FilterSnapshot::Equal(FieldDependency { field: field.name }, value.clone()))
        }
        Some("not-null") => {
            let field_data = s_expr.next().ok_or(crate::error!("missing field in `non-null` filter"))?;
            let field_id = field_data[1].as_i64().ok_or(crate::error!("expected `field-id` to be an integer"))?;
            let field = metabase_field::table.find(field_id as i32).get_result::<DbField>(conn)?;
            Ok(FilterSnapshot::NotNull(FieldDependency { field: field.name }))
        }
        Some(key) => Err(crate::error!("unsupported query filter type `{}`", key))?,
        None => Err(crate::error!("expected query filter type to be a string"))?,
    }
}

fn snapshot_order_by(conn: &PgConnection, raw: &serde_json::Value) -> Result<Vec<OrderBySnapshot>, Box<dyn Error>> {
    let mut fields = Vec::new();
    let ordering = raw.as_array().ok_or(crate::error!("expected `order-by` to be an array"))?;
    for ord in ordering {
        let order = match ord[0].as_str() {
            Some("asc") => Order::Asc,
            Some("desc") => Order::Desc,
            _ => Err(crate::error!("expected `order-by` direction to be either \"asc\" or \"desc\""))?,
        };
        let field_id = ord[1][1].as_i64().ok_or(crate::error!("expected `field-id` to be an integer"))?;
        let field = metabase_field::table.find(field_id as i32).get_result::<DbField>(conn)?;
        fields.push(OrderBySnapshot { field: FieldDependency { field: field.name }, order });
    }
    Ok(fields)
}
*/
