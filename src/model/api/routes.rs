use crate::model::database::{
    finalized_schema::routes::dsl::*, insertable::NewRoute, queryable::Route,
};
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

/*
#[derive(Clone, Debug, Insertable)]
#[table_name = "routes"]
pub struct NewRoute<'a> {
    pub parent: i32,
    pub url_slug: &'a str,
}
#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct Route {
    pub id: i32,
    pub parent: Option<i32>,
    pub url_slug: Option<String>,
}
*/

pub enum RouteCreationStatus {
    Success(QueryResult<Route>),
    RouteNameExists,
}

pub enum RouteUpdateStatus<T> {
    Success(T),
    Failure(T),
    ValueOverlap,
    DoesNotExist,
}
