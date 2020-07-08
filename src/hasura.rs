use graphql_client::{GraphQLQuery, Response};
use std::error::Error;

#[allow(non_camel_case_types)]
type uuid = String;

#[derive(Debug, GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/queries.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct GetUrl;

#[derive(Debug, GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/queries.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct CreateUrl;

#[derive(Debug, GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/queries.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct DeleteUrl;

pub fn fetch_url(short: &str) -> Result<get_url::ResponseData, Box<dyn Error>>{
    let req_body = GetUrl::build_query(get_url::Variables {
        limit: 1,
        short: short.to_string()
    });

    let client = reqwest::Client::new();
    let mut res = client
        .post("http://localhost:8080/v1/graphql")
        .json(&req_body)
        .send().unwrap();

    let body: Response<get_url::ResponseData> = res.json().unwrap();

    let data: get_url::ResponseData = body.data.expect("missing response data");

    Ok(data)
}
