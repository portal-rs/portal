use crate::types::{
    dns::Name,
    rr::{Class, Type},
};

pub trait ToQuery {
    fn to_query(self) -> Query;
}

pub struct Query {
    pub name: Name,
    pub ty: Type,
    pub class: Class,
}

impl ToQuery for Query {
    fn to_query(self) -> Query {
        self
    }
}

impl ToQuery for (Name, Type, Class) {
    fn to_query(self) -> Query {
        let (name, ty, class) = self;
        Query::new(name, ty, class)
    }
}

impl Query {
    pub fn new(name: Name, ty: Type, class: Class) -> Self {
        Self { name, ty, class }
    }
}
