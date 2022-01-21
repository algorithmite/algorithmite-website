use crate::model::database::{
    finalized_schema::routes::dsl::*, insertable::NewRoute, queryable::Route,
};
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

/*
#[derive(Clone, Debug, Insertable)]
#[table_name = "routes"]
pub struct NewRoute<'a> {
    pub parent: Option<i32>,
    pub url_slug: &'a str,
}
#[derive(Clone, Debug, Eq, Identifiable, PartialOrd, Queryable)]
pub struct Route {
    pub id: i32,
    pub parent: Option<i32>,
    pub url_slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

pub fn query_id(conn: &PgConnection, query_id: i32) -> QueryResult<Route> {
    routes.filter(id.eq(query_id)).first(conn)
}

pub fn query_url_slug(conn: &PgConnection, query_url_slug: String) -> QueryResult<Route> {
    routes.filter(url_slug.eq(query_url_slug)).first(conn)
}

pub fn query_all_parent(conn: &PgConnection, query_parent_id: i32) -> QueryResult<Vec<Route>> {
    routes.filter(parent.eq(query_parent_id)).get_results(conn)
}

pub fn create_user_action(
    conn: &PgConnection,
    input_parent: Option<i32>,
    input_url_slug: String,
) -> RouteCreationStatus {
    match query_url_slug(conn, input_url_slug.to_owned()) {
        Ok(_) => return RouteCreationStatus::RouteNameExists,
        _ => (),
    };
    let new_route: NewRoute = NewRoute {
        parent: input_parent,
        url_slug: &input_url_slug,
    };
    RouteCreationStatus::Success(insert_into(routes).values(&new_route).get_result(conn))
}
