mod schema;

#[tokio::main]
async fn main() {
    let query = "{ add00(a: 10, b: 20) { value } }";
    let schema = schema::create_schema();

    // execute query 1000 times and measure the time
    let start = std::time::Instant::now();
    for _ in 0..100_000 {
        let _res = schema.execute(query).await;
    }
    let end = std::time::Instant::now();
    eprintln!("Time elapsed: {:?}", end - start);

    let res = schema.execute(query).await;
    println!("{}", &schema.sdl());

    eprintln!("query:  {:?}", query);
    eprintln!("result: {:?}", res.data.into_json().unwrap().to_string());
}
