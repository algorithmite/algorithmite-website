use crate::model::database::{
    finalized_schema::routes::dsl::*, insertable::NewRoute, queryable::Route,
};
use rocket_sync_db_pools::diesel::{insert_into, prelude::*, result::QueryResult, PgConnection};

//Create

pub fn create_user_action(
    conn: &PgConnection,
    input_parent: Option<i32>,
    input_url_slug: String,
) -> Option<QueryResult<Route>> {
    match read_route(conn, None, None, Some(input_url_slug.to_owned())) {
        Ok(_) => return None,
        _ => (),
    };
    let new_route: NewRoute = NewRoute {
        parent: input_parent,
        url_slug: &input_url_slug,
    };
    Some(insert_into(routes).values(&new_route).get_result(conn))
}

//Read

pub fn read_route(
    conn: &PgConnection,
    option_id: Option<i32>,
    option_parent: Option<Option<i32>>,
    option_url_slug: Option<String>,
) -> QueryResult<Route> {
    let mut query = routes.into_boxed();
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_parent {
        Some(query_parent) => query = query.filter(parent.eq(query_parent)),
        _ => (),
    }
    match option_url_slug {
        Some(query_url_slug) => query = query.filter(url_slug.eq(query_url_slug)),
        _ => (),
    }
    query.first(conn)
}

pub fn read_routes(
    conn: &PgConnection,
    option_id: Option<i32>,
    option_parent: Option<Option<i32>>,
    option_url_slug: Option<String>,
) -> QueryResult<Vec<Route>> {
    let mut query = routes.into_boxed();
    match option_id {
        Some(query_id) => query = query.filter(id.eq(query_id)),
        _ => (),
    }
    match option_parent {
        Some(query_parent) => query = query.filter(parent.eq(query_parent)),
        _ => (),
    }
    match option_url_slug {
        Some(query_url_slug) => query = query.filter(url_slug.eq(query_url_slug)),
        _ => (),
    }
    query.get_results(conn)
}

//Update

pub fn update_route(
    conn: &PgConnection,
    route_id: i32,
    option_parent: Option<Option<i32>>,
    option_url_slug: Option<String>,
) -> Option<QueryResult<usize>> {
    match read_route(conn, Some(route_id), None, None) {
        Ok(query_role) => {
            let mut final_parent: Option<i32> = query_role.parent;
            let mut final_url_slug: String = query_role.url_slug.clone();

            match option_parent {
                Some(query_parent) => final_parent = query_parent,
                _ => (),
            }
            match option_url_slug {
                Some(query_url_slug) => final_url_slug = query_url_slug,
                _ => (),
            }

            Some(
                diesel::update(&query_role)
                    .set((parent.eq(final_parent), url_slug.eq(final_url_slug)))
                    .execute(conn),
            )
        }
        _ => None,
    }
}

//Delete

pub fn delete_route(conn: &PgConnection, route_id: i32) -> QueryResult<Route> {
    match read_route(conn, Some(route_id), None, None) {
        Ok(query_route) => diesel::delete(&query_route).get_result(conn),
        Err(e) => Err(e),
    }
}
