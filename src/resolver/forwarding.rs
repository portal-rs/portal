use super::ToResolver;

pub struct ForwardingResolver {}

impl ToResolver for ForwardingResolver {
    fn resolve(&self, message: &crate::types::dns::Message) -> super::ResolveResult {
        todo!()
    }

    fn resolve_raw(&self, name: String, class: u16, typ: u16) -> super::ResolveResult {
        todo!()
    }

    fn lookup(&self, name: String, class: u16, typ: u16) -> super::ResolveResult {
        todo!()
    }

    fn refresh(&self, name: String, class: u16, typ: u16) {
        todo!()
    }
}
