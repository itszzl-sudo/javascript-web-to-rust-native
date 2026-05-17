use crate::core::JsValue;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum EventType {
    Click,
    MouseEnter,
    MouseLeave,
    KeyDown,
    KeyUp,
    Load,
}

pub struct Event {
    event_type: EventType,
    _target: Option<Box<dyn EventTarget>>,
}

pub trait EventTarget {
    fn add_event_listener(&mut self, event_type: EventType, handler: Box<dyn Fn(&Event) -> JsValue>);
    fn remove_event_listener(&mut self, event_type: EventType, handler: Box<dyn Fn(&Event) -> JsValue>);
    fn dispatch_event(&self, event: Event);
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Event {
            event_type,
            _target: None,
        }
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn stop_propagation(&self) {
        // Implementation for stopping event propagation
    }

    pub fn prevent_default(&self) {
        // Implementation for preventing default browser action
    }
}
