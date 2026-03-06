use crate::hook::HookId;
use crate::view::Context;

pub trait IntoElement {
    type Output;

    fn into_element(&self, cx: &mut Context<Self::Output>) -> Element<Self::Output>;
}

pub struct Element<R> {
    pub inner: R,
    pub dependencies: Vec<HookId>,
}
