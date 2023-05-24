#[macro_use]
extern crate tracing;

use eframe::{App, Frame};
use egui::{CentralPanel, Context};
use egui_tracing::{DisplayInfo, EguiLog};
use tracing::Span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

fn main() {
    let (layer, mut widget) = egui_tracing::EguiLayer::new(10);
    widget.display_info = DisplayInfo {
        filter: "f".to_string(),
        level: "l".to_string(),
        time: "t".to_string(),
        span_data: "sd".to_string(),
        data: "d".to_string(),
        message: "m".to_string(),
    };

    let layer = layer.with_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("example=trace,warn")),
    );

    tracing_subscriber::registry().with(layer).init();

    warp_log();

    let _ = eframe::run_native(
        "example",
        Default::default(),
        Box::new(|_| Box::new(Example(widget))),
    );
}

#[instrument(name = "log_fn", fields(c))]
fn log(a: &str, b: i32) {
    let span = Span::current();
    info!(with = "start", "before");
    span.record("c", "field c");
    warn!(with = "end", "after");

    debug!("{}", "long ".repeat(100));

    error!("{}", "multi line\n".repeat(10));

    trace!("end");
}

#[instrument]
fn warp_log() {
    log("field a", 2);
}

pub struct Example(pub EguiLog);

impl App for Example {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.button("log").clicked() {
                warp_log();
                info!("clicked");
            }

            if let Some(level) = self.0.update() {
                println!("{:?}", level);
            }

            ui.add(&mut self.0);
        });
    }
}
