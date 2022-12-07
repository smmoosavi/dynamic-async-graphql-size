mod schema;

#[tokio::main]
async fn main() {
    let query = "{ add00(a: 10, b: 20) { value } }";
    let schema = schema::create_schema();
    let res = schema.execute(query).await;
    println!("{}", &schema.sdl());

    eprintln!("query:  {:?}", query);
    eprintln!("result: {:?}", res.data.into_json().unwrap().to_string());
}
