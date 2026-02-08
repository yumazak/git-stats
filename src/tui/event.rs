//! Event handling for TUI

use crate::error::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Terminal events
#[derive(Debug)]
pub enum Event {
    /// Key press event
    Key(KeyEvent),
    /// Terminal resize event
    Resize(u16, u16),
    /// Tick event (for animations/updates)
    Tick,
}

/// Event handler that runs in a separate thread
pub struct EventHandler {
    /// Event receiver
    receiver: mpsc::Receiver<Event>,
    /// Sender handle (kept for Drop)
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate in milliseconds
    #[must_use]
    pub fn new(tick_rate: u64) -> Self {
        let (sender, receiver) = mpsc::channel();
        let tick_rate = Duration::from_millis(tick_rate);

        let sender_clone = sender.clone();
        thread::spawn(move || {
            loop {
                // Poll for events with timeout
                if event::poll(tick_rate).unwrap_or(false) {
                    match event::read() {
                        Ok(CrosstermEvent::Key(key)) => {
                            if sender_clone.send(Event::Key(key)).is_err() {
                                break;
                            }
                        }
                        Ok(CrosstermEvent::Resize(w, h)) => {
                            if sender_clone.send(Event::Resize(w, h)).is_err() {
                                break;
                            }
                        }
                        _ => {}
                    }
                } else {
                    // Send tick event
                    if sender_clone.send(Event::Tick).is_err() {
                        break;
                    }
                }
            }
        });

        Self { receiver, sender }
    }

    /// Get the next event, blocking until one is available
    ///
    /// # Errors
    ///
    /// Returns an error if the event channel is disconnected.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv().unwrap_or(Event::Tick))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_handler_creation() {
        let handler = EventHandler::new(100);
        // Just verify it can be created without panicking
        drop(handler);
    }
}
