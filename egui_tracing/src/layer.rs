use crate::widget::EguiLog;
use chrono::Local;
use crossbeam_channel::{Receiver, Sender};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Record};
use tracing::{Event, Id, Level, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::{LookupSpan, Scope};
use tracing_subscriber::Layer;

pub struct EguiLayer {
    span_data: RwLock<HashMap<Id, HashMap<&'static str, String>>>,
    sender: Sender<LogRecord>,
    receiver: Receiver<LogRecord>,
}

#[derive(Debug)]
pub(crate) struct LogRecord {
    pub level: Level,
    pub time: String,
    pub message: String,
    pub data: HashMap<&'static str, String>,
    pub span_data: Vec<(&'static str, HashMap<&'static str, String>)>,
}

impl EguiLayer {
    pub fn new(log_max_size: usize) -> (Self, EguiLog) {
        let (sender, receiver) = crossbeam_channel::bounded(log_max_size);
        let layer = Self {
            span_data: Default::default(),
            sender,
            receiver: receiver.clone(),
        };
        let ui = EguiLog::new(log_max_size, receiver);
        (layer, ui)
    }
}

impl<S: Subscriber + for<'a> LookupSpan<'a>> Layer<S> for EguiLayer {
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, _ctx: Context<'_, S>) {
        let mut map = self.span_data.write();
        let entry = map.entry(id.clone()).or_default();
        attrs.record(&mut EguiVisit(entry));
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, _ctx: Context<'_, S>) {
        let mut map = self.span_data.write();
        let entry = map.entry(span.clone()).or_default();
        values.record(&mut EguiVisit(entry));
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut data = Default::default();
        let mut visit = EguiVisit(&mut data);
        event.record(&mut visit);

        let metadata = event.metadata();
        let level = *metadata.level();

        let span_data = ctx
            .event_scope(event)
            .map(|scope| {
                let r = self.span_data.read();
                Scope::from_root(scope)
                    .filter_map(|span| r.get(&span.id()).cloned().map(|m| (span.name(), m)))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let message = data
            .remove("message")
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let record = LogRecord {
            level,
            time: Local::now().format("%F %T.%3f").to_string(),
            message,
            data,
            span_data,
        };

        if self.receiver.is_full() {
            let _ = self.receiver.recv();
        }

        let _ = self.sender.send(record);
    }

    fn on_close(&self, id: Id, _ctx: Context<'_, S>) {
        self.span_data.write().remove(&id);
    }
}

struct EguiVisit<'a>(pub &'a mut HashMap<&'static str, String>);

impl EguiVisit<'_> {
    #[inline]
    pub fn add(&mut self, field: &Field, value: impl ToString) {
        self.0.insert(field.name(), value.to_string());
    }
}

macro_rules! impl_visit {
    (fn $fn_name: ident (&mut self, $field: ident : &Field, $value: ident : $ty: ty)) => {
        fn $fn_name(&mut self, $field: &Field, $value: $ty) {
            self.add($field, $value);
        }
    };
}

impl Visit for EguiVisit<'_> {
    impl_visit!(fn record_f64(&mut self, field: &Field, value: f64));

    impl_visit!(fn record_i64(&mut self, field: &Field, value: i64));

    impl_visit!(fn record_u64(&mut self, field: &Field, value: u64));

    impl_visit!(fn record_i128(&mut self, field: &Field, value: i128));

    impl_visit!(fn record_u128(&mut self, field: &Field, value: u128));

    impl_visit!(fn record_bool(&mut self, field: &Field, value: bool));

    impl_visit!(fn record_str(&mut self, field: &Field, value: &str));

    impl_visit!(fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)));

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.add(field, format!("{value:#?}"));
    }
}
