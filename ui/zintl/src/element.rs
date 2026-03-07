use crate::hook::HookId;
use crate::view::{Context, View};

pub struct ElementContext<R> {
    phantom: std::marker::PhantomData<R>,
}

impl<R> ElementContext<R> {
    pub fn new() -> Self {
        ElementContext {
            phantom: std::marker::PhantomData,
        }
    }

    pub fn view_cx(&mut self) -> Context<R> {
        todo!()
    }
}

pub trait IntoElement {
    type Output;

    fn into_element(&self, cx: &mut ElementContext<Self::Output>) -> Element<Self::Output>;
}

pub enum Element<R> {
    Binded {
        inner: Box<Element<R>>,
        dependencies: Vec<HookId>,
        builder: Box<dyn View<Output = R>>,
    },
    BindedList {
        items: Vec<Element<R>>,
        dependencies: Vec<HookId>,
        builder: Box<dyn View<Output = R>>,
    },
    Normal {
        inner: R,
    },
    NormalList {
        items: Vec<Element<R>>,
    },
}
