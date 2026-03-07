use crate::element::{Element, ElementContext, IntoElement};

pub struct Context<R> {
    phantom: std::marker::PhantomData<R>,
}

impl<R> Context<R> {
    pub(crate) fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }

    pub fn child(&mut self) -> Self {
        Self::new()
    }
}

pub trait View {
    type Output;

    fn init(&mut self, _cx: &mut Context<Self::Output>) {}
    fn render(&self, cx: &mut Context<Self::Output>) -> Element<Self::Output>;
    fn deinit(&mut self, _cx: &mut Context<Self::Output>) {}
}

impl<R, T: View<Output = R> + Copy> IntoElement for T {
    type Output = R;

    fn into_element(&self, cx: &mut ElementContext<R>) -> Element<R> {
        let mut view_cx = cx.view_cx();
        let inner = self.render(&mut view_cx);

        Element::Binded {
            inner,
            dependencies: view_cx.get_dependencies(),
            builder: Box::new(Self),
        }
    }
}
