use crate::core::JsValue;
use std::cell::Cell;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum EventType {
    Click,
    DblClick,
    MouseDown,
    MouseUp,
    MouseMove,
    MouseEnter,
    MouseLeave,
    MouseOut,
    MouseOver,
    ContextMenu,
    Wheel,
    KeyDown,
    KeyUp,
    KeyPress,
    Focus,
    Blur,
    FocusIn,
    FocusOut,
    Change,
    Input,
    Submit,
    Reset,
    Select,
    Scroll,
    Resize,
    Load,
    Unload,
    Error,
    Drag,
    DragStart,
    DragEnd,
    DragEnter,
    DragLeave,
    DragOver,
    Drop,
    TouchStart,
    TouchMove,
    TouchEnd,
    TouchCancel,
    Copy,
    Cut,
    Paste,
    Play,
    Pause,
    Ended,
    VolumeChange,
    Seeked,
    TimeUpdate,
    Custom(String),
}

impl EventType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "click" => EventType::Click,
            "dblclick" => EventType::DblClick,
            "mousedown" => EventType::MouseDown,
            "mouseup" => EventType::MouseUp,
            "mousemove" => EventType::MouseMove,
            "mouseenter" => EventType::MouseEnter,
            "mouseleave" => EventType::MouseLeave,
            "mouseout" => EventType::MouseOut,
            "mouseover" => EventType::MouseOver,
            "contextmenu" => EventType::ContextMenu,
            "wheel" => EventType::Wheel,
            "keydown" => EventType::KeyDown,
            "keyup" => EventType::KeyUp,
            "keypress" => EventType::KeyPress,
            "focus" => EventType::Focus,
            "blur" => EventType::Blur,
            "focusin" => EventType::FocusIn,
            "focusout" => EventType::FocusOut,
            "change" => EventType::Change,
            "input" => EventType::Input,
            "submit" => EventType::Submit,
            "reset" => EventType::Reset,
            "select" => EventType::Select,
            "scroll" => EventType::Scroll,
            "resize" => EventType::Resize,
            "load" => EventType::Load,
            "unload" => EventType::Unload,
            "error" => EventType::Error,
            "drag" => EventType::Drag,
            "dragstart" => EventType::DragStart,
            "dragend" => EventType::DragEnd,
            "dragenter" => EventType::DragEnter,
            "dragleave" => EventType::DragLeave,
            "dragover" => EventType::DragOver,
            "drop" => EventType::Drop,
            "touchstart" => EventType::TouchStart,
            "touchmove" => EventType::TouchMove,
            "touchend" => EventType::TouchEnd,
            "touchcancel" => EventType::TouchCancel,
            "copy" => EventType::Copy,
            "cut" => EventType::Cut,
            "paste" => EventType::Paste,
            "play" => EventType::Play,
            "pause" => EventType::Pause,
            "ended" => EventType::Ended,
            "volumechange" => EventType::VolumeChange,
            "seeked" => EventType::Seeked,
            "timeupdate" => EventType::TimeUpdate,
            other => EventType::Custom(other.to_string()),
        }
    }
    
    pub fn as_str(&self) -> &str {
        match self {
            EventType::Click => "click",
            EventType::DblClick => "dblclick",
            EventType::MouseDown => "mousedown",
            EventType::MouseUp => "mouseup",
            EventType::MouseMove => "mousemove",
            EventType::MouseEnter => "mouseenter",
            EventType::MouseLeave => "mouseleave",
            EventType::MouseOut => "mouseout",
            EventType::MouseOver => "mouseover",
            EventType::ContextMenu => "contextmenu",
            EventType::Wheel => "wheel",
            EventType::KeyDown => "keydown",
            EventType::KeyUp => "keyup",
            EventType::KeyPress => "keypress",
            EventType::Focus => "focus",
            EventType::Blur => "blur",
            EventType::FocusIn => "focusin",
            EventType::FocusOut => "focusout",
            EventType::Change => "change",
            EventType::Input => "input",
            EventType::Submit => "submit",
            EventType::Reset => "reset",
            EventType::Select => "select",
            EventType::Scroll => "scroll",
            EventType::Resize => "resize",
            EventType::Load => "load",
            EventType::Unload => "unload",
            EventType::Error => "error",
            EventType::Drag => "drag",
            EventType::DragStart => "dragstart",
            EventType::DragEnd => "dragend",
            EventType::DragEnter => "dragenter",
            EventType::DragLeave => "dragleave",
            EventType::DragOver => "dragover",
            EventType::Drop => "drop",
            EventType::TouchStart => "touchstart",
            EventType::TouchMove => "touchmove",
            EventType::TouchEnd => "touchend",
            EventType::TouchCancel => "touchcancel",
            EventType::Copy => "copy",
            EventType::Cut => "cut",
            EventType::Paste => "paste",
            EventType::Play => "play",
            EventType::Pause => "pause",
            EventType::Ended => "ended",
            EventType::VolumeChange => "volumechange",
            EventType::Seeked => "seeked",
            EventType::TimeUpdate => "timeupdate",
            EventType::Custom(s) => s.as_str(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ModifierState {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Clone, Debug)]
pub enum EventData {
    None,
    Mouse(MouseEventData),
    Keyboard(KeyboardEventData),
    Touch(TouchEventData),
    Drag(DragEventData),
    Focus(FocusEventData),
    Form(FormEventData),
    Custom(std::collections::HashMap<String, JsValue>),
}

#[derive(Clone, Debug)]
pub struct MouseEventData {
    pub client_x: f64,
    pub client_y: f64,
    pub page_x: f64,
    pub page_y: f64,
    pub screen_x: f64,
    pub screen_y: f64,
    pub button: i32,
    pub buttons: u32,
    pub modifiers: ModifierState,
}

impl Default for MouseEventData {
    fn default() -> Self {
        Self {
            client_x: 0.0,
            client_y: 0.0,
            page_x: 0.0,
            page_y: 0.0,
            screen_x: 0.0,
            screen_y: 0.0,
            button: 0,
            buttons: 0,
            modifiers: ModifierState::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeyboardEventData {
    pub key: String,
    pub code: String,
    pub key_code: u32,
    pub char_code: u32,
    pub which: u32,
    pub repeat: bool,
    pub modifiers: ModifierState,
}

impl Default for KeyboardEventData {
    fn default() -> Self {
        Self {
            key: String::new(),
            code: String::new(),
            key_code: 0,
            char_code: 0,
            which: 0,
            repeat: false,
            modifiers: ModifierState::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TouchEventData {
    pub touches: Vec<TouchPoint>,
    pub target_touches: Vec<TouchPoint>,
    pub changed_touches: Vec<TouchPoint>,
    pub modifiers: ModifierState,
}

impl Default for TouchEventData {
    fn default() -> Self {
        Self {
            touches: Vec::new(),
            target_touches: Vec::new(),
            changed_touches: Vec::new(),
            modifiers: ModifierState::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TouchPoint {
    pub identifier: i32,
    pub client_x: f64,
    pub client_y: f64,
    pub page_x: f64,
    pub page_y: f64,
    pub screen_x: f64,
    pub screen_y: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DragEventData {
    pub data_transfer: Option<JsValue>,
    pub mouse_data: MouseEventData,
}

#[derive(Clone, Debug, Default)]
pub struct FocusEventData {
    pub related_target: Option<JsValue>,
}

#[derive(Clone, Debug, Default)]
pub struct FormEventData {
    pub value: Option<JsValue>,
    pub checked: Option<bool>,
}

pub struct Event {
    event_type: EventType,
    data: EventData,
    target: Option<JsValue>,
    current_target: Option<JsValue>,
    time_stamp: f64,
    default_prevented: Cell<bool>,
    propagation_stopped: Cell<bool>,
    immediate_stopped: Cell<bool>,
    event_phase: Cell<u16>,
    bubbles: bool,
    cancelable: bool,
}

pub trait EventTarget {
    fn add_event_listener(&mut self, event_type: EventType, handler: Box<dyn Fn(&Event) -> JsValue>);
    fn remove_event_listener(&mut self, event_type: EventType, handler_id: usize);
    fn dispatch_event(&self, event: Event);
    fn has_event_listener(&self, event_type: &EventType) -> bool;
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Event {
            event_type,
            data: EventData::None,
            target: None,
            current_target: None,
            time_stamp: 0.0,
            default_prevented: Cell::new(false),
            propagation_stopped: Cell::new(false),
            immediate_stopped: Cell::new(false),
            event_phase: Cell::new(0),
            bubbles: false,
            cancelable: true,
        }
    }
    
    pub fn with_data(event_type: EventType, data: EventData) -> Self {
        Event {
            event_type,
            data,
            target: None,
            current_target: None,
            time_stamp: 0.0,
            default_prevented: Cell::new(false),
            propagation_stopped: Cell::new(false),
            immediate_stopped: Cell::new(false),
            event_phase: Cell::new(0),
            bubbles: false,
            cancelable: true,
        }
    }
    
    pub fn with_bubbles(event_type: EventType, bubbles: bool) -> Self {
        let mut event = Self::new(event_type);
        event.bubbles = bubbles;
        event
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }
    
    pub fn data(&self) -> &EventData {
        &self.data
    }
    
    pub fn target(&self) -> Option<&JsValue> {
        self.target.as_ref()
    }
    
    pub fn current_target(&self) -> Option<&JsValue> {
        self.current_target.as_ref()
    }
    
    pub fn time_stamp(&self) -> f64 {
        self.time_stamp
    }
    
    pub fn bubbles(&self) -> bool {
        self.bubbles
    }
    
    pub fn cancelable(&self) -> bool {
        self.cancelable
    }
    
    pub fn default_prevented(&self) -> bool {
        self.default_prevented.get()
    }
    
    pub fn propagation_stopped(&self) -> bool {
        self.propagation_stopped.get()
    }
    
    pub fn event_phase(&self) -> u16 {
        self.event_phase.get()
    }

    pub fn stop_propagation(&self) {
        self.propagation_stopped.set(true);
    }
    
    pub fn stop_immediate_propagation(&self) {
        self.propagation_stopped.set(true);
        self.immediate_stopped.set(true);
    }

    pub fn prevent_default(&self) {
        if self.cancelable {
            self.default_prevented.set(true);
        }
    }
    
    pub fn init_event(&mut self, bubbles: bool, cancelable: bool) {
        self.bubbles = bubbles;
        self.cancelable = cancelable;
    }
    
    pub fn is_mouse_event(&self) -> bool {
        matches!(self.data, EventData::Mouse(_))
    }
    
    pub fn is_keyboard_event(&self) -> bool {
        matches!(self.data, EventData::Keyboard(_))
    }
    
    pub fn is_touch_event(&self) -> bool {
        matches!(self.data, EventData::Touch(_))
    }
    
    pub fn mouse_data(&self) -> Option<&MouseEventData> {
        match &self.data {
            EventData::Mouse(data) => Some(data),
            _ => None,
        }
    }
    
    pub fn keyboard_data(&self) -> Option<&KeyboardEventData> {
        match &self.data {
            EventData::Keyboard(data) => Some(data),
            _ => None,
        }
    }
    
    pub fn touch_data(&self) -> Option<&TouchEventData> {
        match &self.data {
            EventData::Touch(data) => Some(data),
            _ => None,
        }
    }
}

pub struct MouseEvent {
    event: Event,
}

impl MouseEvent {
    pub fn new(event_type: EventType, data: MouseEventData) -> Self {
        Self {
            event: Event::with_data(event_type, EventData::Mouse(data)),
        }
    }
    
    pub fn client_x(&self) -> f64 {
        self.event.mouse_data().map(|d| d.client_x).unwrap_or(0.0)
    }
    
    pub fn client_y(&self) -> f64 {
        self.event.mouse_data().map(|d| d.client_y).unwrap_or(0.0)
    }
    
    pub fn page_x(&self) -> f64 {
        self.event.mouse_data().map(|d| d.page_x).unwrap_or(0.0)
    }
    
    pub fn page_y(&self) -> f64 {
        self.event.mouse_data().map(|d| d.page_y).unwrap_or(0.0)
    }
    
    pub fn screen_x(&self) -> f64 {
        self.event.mouse_data().map(|d| d.screen_x).unwrap_or(0.0)
    }
    
    pub fn screen_y(&self) -> f64 {
        self.event.mouse_data().map(|d| d.screen_y).unwrap_or(0.0)
    }
    
    pub fn button(&self) -> i32 {
        self.event.mouse_data().map(|d| d.button).unwrap_or(0)
    }
    
    pub fn buttons(&self) -> u32 {
        self.event.mouse_data().map(|d| d.buttons).unwrap_or(0)
    }
    
    pub fn ctrl_key(&self) -> bool {
        self.event.mouse_data().map(|d| d.modifiers.ctrl).unwrap_or(false)
    }
    
    pub fn shift_key(&self) -> bool {
        self.event.mouse_data().map(|d| d.modifiers.shift).unwrap_or(false)
    }
    
    pub fn alt_key(&self) -> bool {
        self.event.mouse_data().map(|d| d.modifiers.alt).unwrap_or(false)
    }
    
    pub fn meta_key(&self) -> bool {
        self.event.mouse_data().map(|d| d.modifiers.meta).unwrap_or(false)
    }
    
    pub fn as_event(&self) -> &Event {
        &self.event
    }
    
    pub fn into_event(self) -> Event {
        self.event
    }
}

pub struct KeyboardEvent {
    event: Event,
}

impl KeyboardEvent {
    pub fn new(event_type: EventType, data: KeyboardEventData) -> Self {
        Self {
            event: Event::with_data(event_type, EventData::Keyboard(data)),
        }
    }
    
    pub fn key(&self) -> &str {
        self.event.keyboard_data().map(|d| d.key.as_str()).unwrap_or("")
    }
    
    pub fn code(&self) -> &str {
        self.event.keyboard_data().map(|d| d.code.as_str()).unwrap_or("")
    }
    
    pub fn key_code(&self) -> u32 {
        self.event.keyboard_data().map(|d| d.key_code).unwrap_or(0)
    }
    
    pub fn char_code(&self) -> u32 {
        self.event.keyboard_data().map(|d| d.char_code).unwrap_or(0)
    }
    
    pub fn which(&self) -> u32 {
        self.event.keyboard_data().map(|d| d.which).unwrap_or(0)
    }
    
    pub fn repeat(&self) -> bool {
        self.event.keyboard_data().map(|d| d.repeat).unwrap_or(false)
    }
    
    pub fn ctrl_key(&self) -> bool {
        self.event.keyboard_data().map(|d| d.modifiers.ctrl).unwrap_or(false)
    }
    
    pub fn shift_key(&self) -> bool {
        self.event.keyboard_data().map(|d| d.modifiers.shift).unwrap_or(false)
    }
    
    pub fn alt_key(&self) -> bool {
        self.event.keyboard_data().map(|d| d.modifiers.alt).unwrap_or(false)
    }
    
    pub fn meta_key(&self) -> bool {
        self.event.keyboard_data().map(|d| d.modifiers.meta).unwrap_or(false)
    }
    
    pub fn as_event(&self) -> &Event {
        &self.event
    }
    
    pub fn into_event(self) -> Event {
        self.event
    }
}
