use async_graphql::{dynamic::*, Value};

pub fn create_schema() -> Schema {
    let add_field = Field::new("add", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
        .description("Returns the sum of a and b");
    let query = Object::new("Query").field(add_field);

    Schema::build(query.type_name(), None, None)
        .register(query)
        .finish()
        .unwrap()
}
