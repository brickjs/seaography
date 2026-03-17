use crate::{BuilderContext, Connection, SeaResult};
use async_graphql::dynamic::{
    InputObject, InputValue, ObjectAccessor, ResolverContext, TypeRef, ValueAccessor,
};
use sea_orm::{EntityTrait, Select};

/// used to hold information about page pagination
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageArgsInput {
    pub page: u64,
    pub size: u64,
    pub search: String,
    pub sorts: Vec<Vec<String>>,
    pub filters: Vec<String>,
}

pub type PageArgsQueryFn<E>
where
    E: EntityTrait,
= Box<dyn Fn(ResolverContext) -> Result<Connection<E>, sea_orm::DbErr> + Sync + Send>;

/// The configuration structure for PageArgsInputBuilder
pub struct PageArgsInputConfig {
    /// name of the object
    pub type_name: String,
    /// name for 'page' field
    pub page: String,
    /// name for 'size' field
    pub size: String,
    /// name for 'search' field
    pub search: String,
    /// name for 'sorts' field
    pub sorts: String,
    /// name for 'filters' field
    pub filters: String,
}

impl std::default::Default for PageArgsInputConfig {
    fn default() -> Self {
        PageArgsInputConfig {
            type_name: "PageArgsInput".into(),
            page: "page".into(),
            size: "size".into(),
            search: "search".into(),
            sorts: "sorts".into(),
            filters: "filters".into(),
        }
    }
}

/// This builder produces the page pagination options input object
pub struct PageArgsInputBuilder {
    pub context: &'static BuilderContext,
}

impl PageArgsInputBuilder {
    /// used to get type name
    pub fn type_name(&self) -> String {
        self.context.page_args_input.type_name.clone()
    }

    /// used to get page pagination options object
    pub fn input_object(&self) -> InputObject {
        InputObject::new(&self.context.page_args_input.type_name)
            .field(InputValue::new(
                &self.context.page_args_input.size,
                TypeRef::named_nn(TypeRef::INT),
            ))
            .field(InputValue::new(
                &self.context.page_args_input.page,
                TypeRef::named_nn(TypeRef::INT),
            ))
            .field(InputValue::new(
                &self.context.page_args_input.search,
                TypeRef::named(TypeRef::STRING),
            ))
            .field(InputValue::new(
                &self.context.page_args_input.sorts,
                TypeRef::List(Box::new(TypeRef::List(Box::new(TypeRef::named(
                    TypeRef::STRING,
                ))))),
            ))
            .field(InputValue::new(
                &self.context.page_args_input.filters,
                TypeRef::List(Box::new(TypeRef::named(TypeRef::STRING))),
            ))
    }

    /// used to parse query input to page pagination options struct
    pub fn parse_object_not_used(&self, object: &ObjectAccessor) -> SeaResult<PageArgsInput> {
        let page = object
            .get(&self.context.page_args_input.page)
            .map_or(Ok(0), |v| v.u64())
            .unwrap_or(0);
        let size = object.try_get(&self.context.page_args_input.size)?.u64()?;
        let search = object
            .try_get(&self.context.page_args_input.search)?
            .string()?
            .to_string();
        let sorts: Vec<Vec<String>> = object
            .try_get(&self.context.page_args_input.sorts)?
            .list()?
            .iter()
            //.map(|x| x.string().unwrap().to_string())
            .map(|x| {
                x.list()
                    .unwrap()
                    .iter()
                    .map(|x| x.string().unwrap().to_string())
                    .collect()
            })
            .collect();

        let filters: Vec<String> = object
            .try_get(&self.context.page_args_input.sorts)?
            .list()?
            .iter()
            .map(|x| x.string().unwrap().to_string())
            .collect();

        Ok(PageArgsInput {
            page,
            size,
            search,
            sorts,
            filters,
        })
    }

    /// used to parse query input to page_args information structure
    pub fn parse_object(&self, value: Option<ValueAccessor<'_>>) -> SeaResult<PageArgsInput> {
        if value.is_none() {
            return Ok(PageArgsInput {
                page: 0,
                size: 25,
                search: "".to_string(),
                sorts: vec![],
                filters: vec![],
            });
        }

        let binding = value.expect("Checked not null");
        let object = binding.object()?;

        let page = object
            .get(&self.context.page_args_input.page)
            .map_or(Ok(0), |v| v.u64())?;

        let size = object
            .get(&self.context.page_args_input.size)
            .map_or(Ok(0), |v| v.u64())?;

        let search = object
            .get(&self.context.page_args_input.search)
            .map_or(Ok(""), |v| v.string())?;

        Ok(PageArgsInput {
            page,
            size,
            search: search.to_string(),
            sorts: vec![],
            filters: vec![],
        })
    }
}
