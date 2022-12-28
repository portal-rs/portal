use crate::types::{
    dns::{Message, Name},
    rr::{Class, Type},
};

pub trait ToQuery: Send {
    fn to_query(self) -> Query;
}

#[derive(Clone)]
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

impl ToQuery for (&Name, &Type, &Class) {
    fn to_query(self) -> Query {
        let (name, ty, class) = self;
        Query::new(name.clone(), *ty, *class)
    }
}

impl ToQuery for &Message {
    fn to_query(self) -> Query {
        let question = self.question().unwrap();

        Query {
            name: question.name.clone(),
            ty: question.ty,
            class: question.class,
        }
    }
}

impl Query {
    pub fn new(name: Name, ty: Type, class: Class) -> Self {
        Self { name, ty, class }
    }
}
