use std::sync::Arc;

use crate::background_thread::BackgroundThread;
use crate::Event;
use crate::ImageInfo;
use super::WindowInner;

/// A event handler.
pub type EventHandler = Box<dyn FnMut(&mut EventHandlerContext) + Send>;

/// The context for a registered event handler.
pub struct EventHandlerContext<'a> {
	/// The vector to add spawned tasks too.
	background_tasks: &'a mut Vec<BackgroundThread<()>>,

	/// Flag to indicate if the event should be passed to other handlers.
	stop_propagation: bool,

	/// The event to be handled.
	event: &'a Event,

	/// The window that triggered the event.
	window: &'a WindowInner,
}

impl<'a> EventHandlerContext<'a> {
	pub(crate) fn new(
		background_tasks: &'a mut Vec<BackgroundThread<()>>,
		event: &'a Event,
		window: &'a WindowInner,
	) -> Self {
		Self {
			background_tasks,
			stop_propagation: false,
			event,
			window,
		}
	}

	/// Stop propagation of the event to other handlers.
	pub fn stop_propagation(&mut self) {
		self.stop_propagation = true;
	}

	/// Check if we should stop propagation of the event.
	pub(crate) fn should_stop_propagation(&self) -> bool {
		self.stop_propagation
	}

	/// Get the event.
	pub fn event(&self) -> &'a Event {
		self.event
	}

	/// Get the currently displayed image for the window.
	pub fn image(&self) -> Option<&'a (Arc<[u8]>, ImageInfo, String)> {
		self.window.image.as_ref()
	}

	/// Get the window that triggered the event.
	pub fn window(&self) -> &'a WindowInner {
		self.window
	}

	/// Spawn a background task.
	///
	/// The task will run in a new thread.
	/// The thread will be joined when [`crate::stop`] is called.
	/// If this is not desired, simply spawn a thread manually.
	pub fn spawn_task<F: FnOnce() + Send + 'static>(&mut self, task: F) {
		self.background_tasks.push(BackgroundThread::new(task));
	}
}
