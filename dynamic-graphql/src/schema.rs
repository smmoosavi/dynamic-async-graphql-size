use async_graphql::{dynamic::*, Value};

struct Box {
    value: i32,
}

fn define_add_field(name: &str, obj_name: &str, schema: SchemaBuilder) -> (Field, SchemaBuilder) {
    let value_field = Field::new("value", TypeRef::named_nn(TypeRef::INT), |ctx| {
        FieldFuture::new(async move {
            let obj = ctx.parent_value.downcast_ref::<Box>().unwrap();
            Ok(Some(Value::from(obj.value)))
        })
    });

    let obj_type = Object::new(obj_name).field(value_field);
    let obj_type_name = obj_type.type_name().to_string();
    let schema = schema.register(obj_type);
    let add_field = Field::new(name, TypeRef::named_nn(obj_type_name), |ctx| {
        FieldFuture::new(async move {
            let a = ctx.args.try_get("a")?.i64()?;
            let b = ctx.args.try_get("b")?.i64()?;
            Ok(Some(FieldValue::owned_any(Box {
                value: (a + b) as i32,
            })))
        })
    })
    .argument(InputValue::new("a", TypeRef::named_nn(TypeRef::INT)))
    .argument(InputValue::new("b", TypeRef::named_nn(TypeRef::INT)))
    .description("Returns the sum of a and b");
    (add_field, schema)
}

macro_rules! define_add_field {
    ($name:expr, $obj_name:expr, $struct_name:ident, $schema:ident) => {{
        define_add_field($name, $obj_name, $schema)
    }};
}

pub fn create_schema() -> Schema {
    let query = Object::new("Query");
    let schema = Schema::build(query.type_name(), None, None);

    let (add_field, schema) = define_add_field!("add00", "Obj00", Obj00, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add01", "Obj01", Obj01, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add02", "Obj02", Obj02, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add03", "Obj03", Obj03, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add04", "Obj04", Obj04, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add05", "Obj05", Obj05, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add06", "Obj06", Obj06, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add07", "Obj07", Obj07, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add08", "Obj08", Obj08, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add09", "Obj09", Obj09, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add10", "Obj10", Obj10, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add11", "Obj11", Obj11, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add12", "Obj12", Obj12, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add13", "Obj13", Obj13, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add14", "Obj14", Obj14, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add15", "Obj15", Obj15, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add16", "Obj16", Obj16, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add17", "Obj17", Obj17, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add18", "Obj18", Obj18, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add19", "Obj19", Obj19, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add20", "Obj20", Obj20, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add21", "Obj21", Obj21, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add22", "Obj22", Obj22, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add23", "Obj23", Obj23, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add24", "Obj24", Obj24, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add25", "Obj25", Obj25, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add26", "Obj26", Obj26, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add27", "Obj27", Obj27, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add28", "Obj28", Obj28, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add29", "Obj29", Obj29, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add30", "Obj30", Obj30, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add31", "Obj31", Obj31, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add32", "Obj32", Obj32, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add33", "Obj33", Obj33, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add34", "Obj34", Obj34, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add35", "Obj35", Obj35, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add36", "Obj36", Obj36, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add37", "Obj37", Obj37, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add38", "Obj38", Obj38, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add39", "Obj39", Obj39, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add40", "Obj40", Obj40, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add41", "Obj41", Obj41, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add42", "Obj42", Obj42, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add43", "Obj43", Obj43, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add44", "Obj44", Obj44, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add45", "Obj45", Obj45, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add46", "Obj46", Obj46, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add47", "Obj47", Obj47, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add48", "Obj48", Obj48, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add49", "Obj49", Obj49, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add50", "Obj50", Obj50, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add51", "Obj51", Obj51, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add52", "Obj52", Obj52, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add53", "Obj53", Obj53, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add54", "Obj54", Obj54, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add55", "Obj55", Obj55, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add56", "Obj56", Obj56, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add57", "Obj57", Obj57, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add58", "Obj58", Obj58, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add59", "Obj59", Obj59, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add60", "Obj60", Obj60, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add61", "Obj61", Obj61, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add62", "Obj62", Obj62, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add63", "Obj63", Obj63, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add64", "Obj64", Obj64, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add65", "Obj65", Obj65, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add66", "Obj66", Obj66, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add67", "Obj67", Obj67, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add68", "Obj68", Obj68, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add69", "Obj69", Obj69, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add70", "Obj70", Obj70, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add71", "Obj71", Obj71, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add72", "Obj72", Obj72, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add73", "Obj73", Obj73, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add74", "Obj74", Obj74, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add75", "Obj75", Obj75, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add76", "Obj76", Obj76, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add77", "Obj77", Obj77, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add78", "Obj78", Obj78, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add79", "Obj79", Obj79, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add80", "Obj80", Obj80, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add81", "Obj81", Obj81, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add82", "Obj82", Obj82, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add83", "Obj83", Obj83, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add84", "Obj84", Obj84, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add85", "Obj85", Obj85, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add86", "Obj86", Obj86, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add87", "Obj87", Obj87, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add88", "Obj88", Obj88, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add89", "Obj89", Obj89, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add90", "Obj90", Obj90, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add91", "Obj91", Obj91, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add92", "Obj92", Obj92, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add93", "Obj93", Obj93, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add94", "Obj94", Obj94, schema);
    let query = query.field(add_field);

    let (add_field, schema) = define_add_field!("add95", "Obj95", Obj95, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add96", "Obj96", Obj96, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add97", "Obj97", Obj97, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add98", "Obj98", Obj98, schema);
    let query = query.field(add_field);
    let (add_field, schema) = define_add_field!("add99", "Obj99", Obj99, schema);
    let query = query.field(add_field);

    let schema = schema.register(query);

    schema.finish().unwrap()
}
