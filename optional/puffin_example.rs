//! `puffin_example`
//!
//! Simple demonstration of how to use the awesome `puffin` crate to do profiling of your application.
//! Most code is straight from the [`puffin` examples](https://github.com/emilk/egui/blob/master/examples/puffin_profiler/src/main.rs).

use std::ops::RangeBounds;

/// Makes a call to `puffin::set_scopes_on(true)` to begin `puffin` profiling, and starts the `puffin_http` server.
#[cfg(feature = "puffin")]
pub fn start_puffin_server() {
    puffin::set_scopes_on(true); // tell puffin to collect data

    match puffin_http::Server::new("0.0.0.0:8585") {
        Ok(puffin_server) => {
            eprintln!("Run:  cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585");

            // We can store the server if we want, but in this case we just want
            // it to keep running. Dropping it closes the server, so let's not drop it!
            #[allow(clippy::mem_forget)]
            std::mem::forget(puffin_server);
        }
        Err(err) => {
            eprintln!("Failed to start puffin server: {}", err);
        }
    };
}

/// No-op when the `profiling` feature is disabled.
#[cfg(not(feature = "puffin"))]
pub fn start_puffin_server() {}

#[derive(Debug, Default)]
struct AppContext;
#[derive(Debug, Default)]
struct FrameInfo {
    pub frame_number: usize,
}

/// Simple trait representing the update loop of a gui or game for example.
trait RequestsUpdate {
    fn update(&mut self, ctx: &AppContext, frame: &mut FrameInfo);
}

/// Simple, terrible, RNG function so as to not rely on any outside libraries for a simple
fn simple_rng(range: impl RangeBounds<u32>) -> u32 {
    let start = match range.start_bound() {
        std::ops::Bound::Included(i) => *i,
        std::ops::Bound::Excluded(i) => i.saturating_add(1),
        std::ops::Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        std::ops::Bound::Included(i) => *i,
        std::ops::Bound::Excluded(i) => i.saturating_sub(1),
        std::ops::Bound::Unbounded => u32::MAX,
    };

    let micros = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time since unix epoch")
        .subsec_micros();

    #[allow(clippy::cast_possible_truncation)]
    let span = end.saturating_sub(start);

    (micros % span) + start
}

#[derive(Default)]
pub struct MyApp;

impl RequestsUpdate for MyApp {
    fn update(&mut self, _ctx: &AppContext, _frame: &mut FrameInfo) {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("MyApp_update");
        // Or you can use
        // puffin::profile_function!();
        // but it is a bit slower (according to puffin's own docs)
        println!("MyApp::update");
        // Do other stuff, yadda yadda.

        if simple_rng(0..100) < 10 {
            #[cfg(feature = "puffin")]
            puffin::profile_scope!("MyApp_update_expensive_calculation");

            println!("MyApp::update() doing some occasional expensive work.");
            // Sleep to simulate expensive work.
            std::thread::sleep(std::time::Duration::from_millis(
                simple_rng(50u32..=250u32).into(),
            ));
        }
    }
}

fn execute_app(app: &mut dyn RequestsUpdate) {
    // Call to start_puffin_server should be here if not already called

    let mut frame = FrameInfo::default();
    let mut ctx = AppContext::default();

    #[cfg(feature = "puffin")]
    puffin::profile_scope!("execute_app");
    loop {
        #[cfg(feature = "puffin")]
        puffin::GlobalProfiler::lock().new_frame();

        app.update(&ctx, &mut frame);
        frame.frame_number += 1;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
