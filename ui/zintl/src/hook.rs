#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HookId(u32);

impl HookId {
    pub const DEFAULT: Self = HookId(0);

    pub fn new(id: u32) -> Self {
        HookId(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

pub trait Hook {
    type Message;

    fn init(&mut self, id: HookId);
    fn get_id(&self) -> HookId;
}

pub struct HookContext {
    triggered: Vec<HookId>,
}

impl HookContext {
    pub fn new() -> Self {
        HookContext {
            triggered: Vec::new(),
        }
    }

    pub fn trigger(&mut self, id: HookId) {
        self.triggered.push(id);
    }
}

pub struct HookManager<M> {
    hooks: Vec<Box<dyn Hook<Message = M>>>,
}
