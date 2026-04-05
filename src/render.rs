use std::time::Instant;
use winit::{event::*,event_loop::EventLoop, keyboard::PhysicalKey, window::Window};
// lets start by just rendering a triangle and then once we do that we can 
// add the part where we move around and shit, and then we can add moving around the 
// triangle and then we can add just a block of a solid color, then we can
// add mult blocks for the chunks.

// need chunk loading and gen as we move around 
// need a voxel  based raycast for placing and breaking voxels (this will be v1)

const NUM_INSTANCES_PER_ROW: u32 = 10;

pub struct State {
  // TODO 
}

pub struct App {
  state: Option<State>,
  // wtf is a proxy and why do I need it 
  // used for tick updates
  last_time: Instant
}

impl App {
  pub fn new(event_loop: EventLoop<State>) -> Self {
    Self {
      state: None,
      last_time: Instant::now()
    }
  }
}

impl ApplicationHandler<State> for App {
  fn resumed(&mut self, event_loop: ActiveEventLoop) {
    let mut window_attributes = Window::default_attributes();
    let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
    // for some reason they made the creation of state async... 
    // thus for now we will also make the creation of state async
    // must poll on result
    self.state = Some(pollster::block_on(State::new(window)).unwrap());
  }
  #[allow(unused_mut)]
  fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
    self.state = Some(event);
  }

  fn device_event(
    &mut self,
    _event_loop: &ActiveEventLoop,
    _device_id: DeviceId,
    event: DeviceEvent,
  ) {
    // if the state is not set for app, then we should exit
    let state = if let Some(state) = &mut self.state {
      state
    } else {
      return;
    }
  }
}
