use notify::RecommendedWatcher;
use notify_debouncer_full::DebounceEventHandler;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::Debouncer;
use notify_debouncer_full::NoCache;
use notify_debouncer_full::new_debouncer;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};

/// Wraps a tokio mpsc sender channel, so we can implement the notify event
/// handler for it (we can't implement a trait we don't own on a struct or
/// type we also don't know. We need to own one of them, then we can implement
/// traits).
struct MySender<T>(Sender<T>);

impl DebounceEventHandler for MySender<DebounceEventResult> {
    fn handle_event(&mut self, event: DebounceEventResult) {
        // Handling the event is just sending it to the channel
        self.0
            .blocking_send(event)
            .expect("blocking send on tokio mpsc channel is ok");
    }
}

pub fn make_async_watcher(
    timeout: Duration,
) -> notify::Result<(
    Debouncer<RecommendedWatcher, NoCache>,
    Receiver<DebounceEventResult>,
)> {
    // number of buffered events. 10 is good? yes? no?
    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let tx = MySender(tx);
    let debouncer = new_debouncer(timeout, None, tx)?;
    Ok((debouncer, rx))
}
