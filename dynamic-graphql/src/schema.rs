use async_graphql::{dynamic::*, Value};

/// Example:
/// let add_field = define_add_field!("add00");
macro_rules! define_add_field {
    ($name:expr) => {
        Field::new($name, TypeRef::named_nn(TypeRef::INT), |ctx| {
            FieldFuture::new(async move {
                let a = ctx.args.try_get("a")?.i64()?;
                let b = ctx.args.try_get("b")?.i64()?;
                Ok(Some(Value::from(a + b)))
            })
        })
        .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
        .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
        .description("Returns the sum of a and b")
    };
}

pub fn create_schema() -> Schema {
    let query = Object::new("Query");

    let add_field = define_add_field!("add00");
    let query = query.field(add_field);
    let add_field = define_add_field!("add01");
    let query = query.field(add_field);
    let add_field = define_add_field!("add02");
    let query = query.field(add_field);
    let add_field = define_add_field!("add03");
    let query = query.field(add_field);
    let add_field = define_add_field!("add04");
    let query = query.field(add_field);

    let add_field = define_add_field!("add05");
    let query = query.field(add_field);
    let add_field = define_add_field!("add06");
    let query = query.field(add_field);
    let add_field = define_add_field!("add07");
    let query = query.field(add_field);
    let add_field = define_add_field!("add08");
    let query = query.field(add_field);
    let add_field = define_add_field!("add09");
    let query = query.field(add_field);

    let add_field = define_add_field!("add10");
    let query = query.field(add_field);
    let add_field = define_add_field!("add11");
    let query = query.field(add_field);
    let add_field = define_add_field!("add12");
    let query = query.field(add_field);
    let add_field = define_add_field!("add13");
    let query = query.field(add_field);
    let add_field = define_add_field!("add14");
    let query = query.field(add_field);

    let add_field = define_add_field!("add15");
    let query = query.field(add_field);
    let add_field = define_add_field!("add16");
    let query = query.field(add_field);
    let add_field = define_add_field!("add17");
    let query = query.field(add_field);
    let add_field = define_add_field!("add18");
    let query = query.field(add_field);
    let add_field = define_add_field!("add19");
    let query = query.field(add_field);

    let add_field = define_add_field!("add20");
    let query = query.field(add_field);
    let add_field = define_add_field!("add21");
    let query = query.field(add_field);
    let add_field = define_add_field!("add22");
    let query = query.field(add_field);
    let add_field = define_add_field!("add23");
    let query = query.field(add_field);
    let add_field = define_add_field!("add24");
    let query = query.field(add_field);

    let add_field = define_add_field!("add25");
    let query = query.field(add_field);
    let add_field = define_add_field!("add26");
    let query = query.field(add_field);
    let add_field = define_add_field!("add27");
    let query = query.field(add_field);
    let add_field = define_add_field!("add28");
    let query = query.field(add_field);
    let add_field = define_add_field!("add29");
    let query = query.field(add_field);

    let add_field = define_add_field!("add30");
    let query = query.field(add_field);
    let add_field = define_add_field!("add31");
    let query = query.field(add_field);
    let add_field = define_add_field!("add32");
    let query = query.field(add_field);
    let add_field = define_add_field!("add33");
    let query = query.field(add_field);
    let add_field = define_add_field!("add34");
    let query = query.field(add_field);

    let add_field = define_add_field!("add35");
    let query = query.field(add_field);
    let add_field = define_add_field!("add36");
    let query = query.field(add_field);
    let add_field = define_add_field!("add37");
    let query = query.field(add_field);
    let add_field = define_add_field!("add38");
    let query = query.field(add_field);
    let add_field = define_add_field!("add39");
    let query = query.field(add_field);

    let add_field = define_add_field!("add40");
    let query = query.field(add_field);
    let add_field = define_add_field!("add41");
    let query = query.field(add_field);
    let add_field = define_add_field!("add42");
    let query = query.field(add_field);
    let add_field = define_add_field!("add43");
    let query = query.field(add_field);
    let add_field = define_add_field!("add44");
    let query = query.field(add_field);

    let add_field = define_add_field!("add45");
    let query = query.field(add_field);
    let add_field = define_add_field!("add46");
    let query = query.field(add_field);
    let add_field = define_add_field!("add47");
    let query = query.field(add_field);
    let add_field = define_add_field!("add48");
    let query = query.field(add_field);
    let add_field = define_add_field!("add49");
    let query = query.field(add_field);

    let add_field = define_add_field!("add50");
    let query = query.field(add_field);
    let add_field = define_add_field!("add51");
    let query = query.field(add_field);
    let add_field = define_add_field!("add52");
    let query = query.field(add_field);
    let add_field = define_add_field!("add53");
    let query = query.field(add_field);
    let add_field = define_add_field!("add54");
    let query = query.field(add_field);

    let add_field = define_add_field!("add55");
    let query = query.field(add_field);
    let add_field = define_add_field!("add56");
    let query = query.field(add_field);
    let add_field = define_add_field!("add57");
    let query = query.field(add_field);
    let add_field = define_add_field!("add58");
    let query = query.field(add_field);
    let add_field = define_add_field!("add59");
    let query = query.field(add_field);

    let add_field = define_add_field!("add60");
    let query = query.field(add_field);
    let add_field = define_add_field!("add61");
    let query = query.field(add_field);
    let add_field = define_add_field!("add62");
    let query = query.field(add_field);
    let add_field = define_add_field!("add63");
    let query = query.field(add_field);
    let add_field = define_add_field!("add64");
    let query = query.field(add_field);

    let add_field = define_add_field!("add65");
    let query = query.field(add_field);
    let add_field = define_add_field!("add66");
    let query = query.field(add_field);
    let add_field = define_add_field!("add67");
    let query = query.field(add_field);
    let add_field = define_add_field!("add68");
    let query = query.field(add_field);
    let add_field = define_add_field!("add69");
    let query = query.field(add_field);

    let add_field = define_add_field!("add70");
    let query = query.field(add_field);
    let add_field = define_add_field!("add71");
    let query = query.field(add_field);
    let add_field = define_add_field!("add72");
    let query = query.field(add_field);
    let add_field = define_add_field!("add73");
    let query = query.field(add_field);
    let add_field = define_add_field!("add74");
    let query = query.field(add_field);

    let add_field = define_add_field!("add75");
    let query = query.field(add_field);
    let add_field = define_add_field!("add76");
    let query = query.field(add_field);
    let add_field = define_add_field!("add77");
    let query = query.field(add_field);
    let add_field = define_add_field!("add78");
    let query = query.field(add_field);
    let add_field = define_add_field!("add79");
    let query = query.field(add_field);

    let add_field = define_add_field!("add80");
    let query = query.field(add_field);
    let add_field = define_add_field!("add81");
    let query = query.field(add_field);
    let add_field = define_add_field!("add82");
    let query = query.field(add_field);
    let add_field = define_add_field!("add83");
    let query = query.field(add_field);
    let add_field = define_add_field!("add84");
    let query = query.field(add_field);

    let add_field = define_add_field!("add85");
    let query = query.field(add_field);
    let add_field = define_add_field!("add86");
    let query = query.field(add_field);
    let add_field = define_add_field!("add87");
    let query = query.field(add_field);
    let add_field = define_add_field!("add88");
    let query = query.field(add_field);
    let add_field = define_add_field!("add89");
    let query = query.field(add_field);

    let add_field = define_add_field!("add90");
    let query = query.field(add_field);
    let add_field = define_add_field!("add91");
    let query = query.field(add_field);
    let add_field = define_add_field!("add92");
    let query = query.field(add_field);
    let add_field = define_add_field!("add93");
    let query = query.field(add_field);
    let add_field = define_add_field!("add94");
    let query = query.field(add_field);

    let add_field = define_add_field!("add95");
    let query = query.field(add_field);
    let add_field = define_add_field!("add96");
    let query = query.field(add_field);
    let add_field = define_add_field!("add97");
    let query = query.field(add_field);
    let add_field = define_add_field!("add98");
    let query = query.field(add_field);
    let add_field = define_add_field!("add99");
    let query = query.field(add_field);

    Schema::build(query.type_name(), None, None)
        .register(query)
        .finish()
        .unwrap()
}
