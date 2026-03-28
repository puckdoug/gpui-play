use gpui::EventEmitter;

/// Events emitted by CounterState when thresholds are crossed.
#[derive(Clone, Debug)]
pub enum CounterEvent {
    ThresholdReached(i32),
}

/// Shared counter state demonstrating Model/Entity, EventEmitter, observe, subscribe.
pub struct CounterState {
    count: i32,
    threshold: i32,
}

impl EventEmitter<CounterEvent> for CounterState {}

impl CounterState {
    pub fn new(threshold: i32) -> Self {
        Self { count: 0, threshold }
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn threshold(&self) -> i32 {
        self.threshold
    }

    /// Increment the counter. Emits ThresholdReached if crossing threshold.
    pub fn increment(&mut self, cx: &mut gpui::Context<Self>) {
        let was_below = self.count < self.threshold;
        self.count += 1;
        if was_below && self.count >= self.threshold {
            cx.emit(CounterEvent::ThresholdReached(self.count));
        }
        cx.notify();
    }

    /// Decrement the counter.
    pub fn decrement(&mut self, cx: &mut gpui::Context<Self>) {
        self.count -= 1;
        cx.notify();
    }

    /// Set count directly (for testing).
    pub fn set_count(&mut self, value: i32, cx: &mut gpui::Context<Self>) {
        self.count = value;
        cx.notify();
    }
}
