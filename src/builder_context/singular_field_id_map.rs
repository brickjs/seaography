use crate::{BuilderContext, TypesMapHelper};
use async_graphql::dynamic::ResolverContext;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{EntityTrait, Iterable, PrimaryKeyToColumn, Select};

pub struct SingularFieldIdMap {
    pub context: &'static BuilderContext,
}

pub trait SingularFieldIdMapTrait {
    fn to_select<T>(&self, context: &'static BuilderContext, ctx: &ResolverContext) -> Select<T>
    where
        T: EntityTrait,
    {
        let mut stmt = T::find();
        let mapper = TypesMapHelper { context };
        let column = T::PrimaryKey::iter()
            .map(|variant| variant.into_column())
            .collect::<Vec<T::Column>>()[0];

        let id = &ctx.args.try_get("id").unwrap();
        let v = mapper
            .async_graphql_value_to_sea_orm_value::<T>(&column, id)
            .unwrap();
        println!("VITO to_singular_field id {:?}", v);

        stmt = stmt.filter(column.eq(v));
        stmt
    }
}
