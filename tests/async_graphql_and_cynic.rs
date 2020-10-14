#[cfg(all(feature = "async-graphql", feature = "cynic"))]
mod async_graphql_and_cynic_tests {
    use std::io;
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    use async_executor::{Executor, Task};
    use futures_lite::future::{self, block_on};

    #[test]
    fn simple() -> io::Result<()> {
        let ex = Executor::new();
        let ex = Arc::new(ex);

        let ex_with_run_pending = ex.clone();
        thread::spawn(move || block_on(ex_with_run_pending.run(future::pending::<()>())));

        block_on(async move {
            let listen_addr_for_server = TcpListener::bind("0.0.0.0:0")
                .unwrap()
                .local_addr()
                .unwrap();
            let listen_addr_for_client = listen_addr_for_server.clone();

            let server: Task<io::Result<()>> = ex.clone().spawn(async move {
                use async_graphql::{
                    http::{playground_source, GraphQLPlaygroundConfig},
                    EmptyMutation, EmptySubscription, Object, Schema,
                };
                use graphql_int64_scalar::UInt64Scalar;
                use tide::{http::mime, Body, Response, StatusCode};

                struct Query;

                #[Object]
                impl Query {
                    async fn echo(&self, i: UInt64Scalar) -> UInt64Scalar {
                        i
                    }
                }

                let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

                let mut app = tide::new();
                let endpoint = async_graphql_tide::endpoint(schema);
                app.at("/graphql").post(endpoint.clone()).get(endpoint);
                app.at("/graphql_playground").get(|_| async {
                    let mut res = Response::new(StatusCode::Ok);
                    res.set_body(Body::from_string(playground_source(
                        GraphQLPlaygroundConfig::new("/graphql"),
                    )));
                    res.set_content_type(mime::HTML);
                    Ok(res)
                });

                println!("server listen {}", listen_addr_for_server);
                app.listen(listen_addr_for_server).await?;

                Ok(())
            });

            let client: Task<io::Result<()>> = ex.clone().spawn(async move {
                use cynic::{serde_json, QueryFragment};
                use futures_timer::Delay;
                use graphql_int64_scalar::UInt64Scalar as UInt64;
                use isahc::{http::Request, ResponseExt};

                mod query_dsl {
                    type Uint64Scalar = graphql_int64_scalar::UInt64Scalar;
                    cynic::query_dsl!("tests/schema.graphql");
                }
                #[derive(cynic::FragmentArguments, Clone)]
                pub struct EchoArguments {
                    pub i: UInt64,
                }
                #[derive(cynic::QueryFragment, Debug)]
                #[cynic(
                    schema_path = "tests/schema.graphql",
                    query_module = "query_dsl",
                    graphql_type = "Query",
                    argument_struct = "EchoArguments"
                )]
                pub struct EchoQuery {
                    #[arguments(i = args.i.clone())]
                    pub echo: UInt64,
                }

                //
                Delay::new(Duration::from_millis(300)).await;

                //
                let echo_arguments = EchoArguments {
                    i: UInt64(u64::MAX),
                };
                let echo_query = cynic::Operation::query(EchoQuery::fragment(&echo_arguments));

                let http_req_body = serde_json::to_string(&echo_query)?;
                println!("{:?}", http_req_body);
                let http_req = Request::post(format!("http://{}/graphql", listen_addr_for_client))
                    .body(http_req_body)
                    .unwrap();
                let mut http_res = isahc::send_async(http_req).await?;
                let http_res_body = http_res.text_async().await?;
                println!("{:?}", http_res_body);
                assert_eq!(http_res_body, r#"{"data":{"echo":"18446744073709551615"}}"#);
                let http_res_json_value = serde_json::from_str(&http_res_body)?;

                let gql_res = echo_query.decode_response(http_res_json_value).unwrap();

                match gql_res.data {
                    Some(data) => assert_eq!(data.echo, UInt64(u64::MAX)),
                    None => assert!(false),
                };

                Ok(())
            });

            client.await?;
            server.cancel().await;

            Ok(())
        })
    }
}
