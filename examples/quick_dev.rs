#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/index.html").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username":"demo1",
            "pwd":"welcome"
        }),
    );

    // This hello will fail -> NO_AUTH
    hc.do_get("/hello").await?.print().await?;

    // Execute login request
    req_login.await?.print().await?;

    // This hello will work because we did the login
    hc.do_get("/hello").await?.print().await?;

    // logoff -> the next hello won't work
    let req_logoff = hc.do_post(
        "/api/logoff",
        json!({
            "logoff": true
        })
    );
    // execute logoff request
    req_logoff.await?.print().await?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}