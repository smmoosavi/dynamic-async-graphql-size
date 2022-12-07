use async_graphql::{dynamic::*, Value};

pub fn create_schema() -> Schema {
    let query = Object::new("Query");

    // ==============================
    // ------------------------------
    let add_field = Field::new("add00", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add01", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add02", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add03", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add04", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add05", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add06", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add07", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add08", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add09", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add10", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add11", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add12", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add13", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add14", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add15", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add16", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add17", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add18", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add19", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add20", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add21", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add22", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add23", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add24", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add25", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add26", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add27", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add28", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add29", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add30", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add31", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add32", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add33", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add34", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add35", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add36", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add37", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add38", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add39", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add40", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add41", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add42", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add43", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add44", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add45", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add46", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add47", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add48", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add49", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add50", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add51", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add52", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add53", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add54", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add55", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add56", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add57", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add58", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add59", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add60", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add61", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add62", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add63", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add64", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add65", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add66", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add67", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add68", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add69", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add70", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add71", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add72", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add73", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add74", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add75", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add76", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add77", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add78", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add79", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add80", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add81", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add82", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add83", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add84", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add85", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add86", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add87", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add88", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add89", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add90", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add91", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add92", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add93", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add94", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ==============================
    // ------------------------------
    let add_field = Field::new("add95", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add96", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add97", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add98", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    // ------------------------------
    let add_field = Field::new("add99", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(Value::from(a + b)))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    let query = query.field(add_field);

    Schema::build(query.type_name(), None, None)
        .register(query)
        .finish()
        .unwrap()
}
