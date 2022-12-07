mod schema;

#[tokio::main]
async fn main() {
    let query = "{ add(a: 10, b: 20) }";
    let schema = schema::create_schema();
    let res = schema.execute(query).await;
    println!("{}", &schema.sdl());

    eprintln!("query:  {:?}", query);
    eprintln!("result: {:?}", res.data.into_json().unwrap().to_string());
}
