use crate::models::shared::description::{Description, TagContext};

pub trait ResolveTags {
    fn resolve_tags(&mut self, ctx: &TagContext);
}

impl ResolveTags for Description {
    fn resolve_tags(&mut self, ctx: &TagContext) {
        self.0 = self.resolve(ctx);
    }
}

impl<T: ResolveTags> ResolveTags for Option<T> {
    fn resolve_tags(&mut self, ctx: &TagContext) {
        if let Some(v) = self {
            v.resolve_tags(ctx);
        }
    }
}
impl<T: ResolveTags> ResolveTags for Vec<T> {
    fn resolve_tags(&mut self, ctx: &TagContext) {
        for v in self.iter_mut() {
            v.resolve_tags(ctx);
        }
    }
}
