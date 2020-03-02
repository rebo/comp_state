use crate::StateAccess;

pub struct DropType {
    pub activated: bool,
    pub on_drop: Box<dyn Fn() -> ()>,
}

impl DropType {
    pub fn new(on_drop: impl Fn() -> () + 'static) -> Self {
        Self {
            activated: true,
            on_drop: Box::new(on_drop),
        }
    }

    pub fn execute_if_activated(&self) {
        if self.activated {
            (self.on_drop)();
        }
    }

    pub fn activate(&mut self) {
        self.activated = true;
    }
    pub fn deactivate(&mut self) {
        self.activated = false;
    }
}

pub trait StateAccessDropType {
    fn activate(&self);
    fn deactivate(&self);
    fn execute_and_remove(self);
}

impl StateAccessDropType for StateAccess<DropType> {
    fn execute_and_remove(self) {
        self.update(|dt| {
            dt.execute_if_activated();
        });
        self.remove();
    }

    fn activate(&self) {
        self.update(|dt| dt.activate());
    }

    fn deactivate(&self) {
        self.update(|dt| dt.deactivate());
    }
}
