mod model;
mod player_camera;
use player_camera as camera;


use std::time::Instant;
use std::sync::Arc;
use winit::{event::*,event_loop::EventLoop, keyboard::PhysicalKey, window::Window};
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;

// lets start by just rendering a triangle and then once we do that we can 
// add the part where we move around and shit, and then we can add moving around the 
// triangle and then we can add just a block of a solid color, then we can
// add mult blocks for the chunks.

// need chunk loading and gen as we move around 
// need a voxel  based raycast for placing and breaking voxels (this will be v1)

const NUM_INSTANCES_PER_ROW: u32 = 10;

pub struct State {
  window: Arc<Window>,
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  render_pipeline: wgpu::RenderPipeline,
  obj_model: model::Model,
  camera: camera::Camera,
  projection: camera::Projection,
  camera_controller: camera::CameraController,
  camera_uniform: camera::CameraUniform,
  camera_buffer: wgpu::Buffer,
  camera_bind_group: wgpu::BindGroup,
  instances: Vec<Instance>,
  #[allow(dead_code)]
  instance_buffer: wgpu::Buffer,
  depth_texture: texture::Texture,
  is_surface_configured: bool,
  light_uniform: LightUniform,
  light_buffer: wgpu::Buffer,
  light_bind_group: wgpu::BindGroup,
  light_render_pipeline: wgpu::RenderPipeline,
  #[allow(dead_code)]
  debug_material: model::material,
  mouse_pressed: bool
}

pub struct App {
  state: Option<State>,
  // wtf is a proxy and why do I need it 
  // used for tick updates
  last_time: Instant
}

impl App {
  pub fn new() -> Self {
    Self {
      state: None,
      last_time: Instant::now()
    }
  }
}

impl ApplicationHandler<State> for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
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
    };

    match event {
      DeviceEvent::MouseMotion {delta: (dx, dy)} => {
        if state.mouse_pressed {
          state.camera_controller.handle_mouse(dx,dy);
        }

      }
      _ => {}
    }
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: WindowEvent,
    ) {
    // early exit if no state
    let state = match &mut self.state {
      Some(canvas) => canvas,
      None => return,
    };

    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::Resized(size) => state.resize(size.width, size.height),
      // like rendering a new frame. need to find the tick time
      WindowEvent::RedrawRequested => {
        let dt = self.last_time.elapsed();
        self.last_time = Instant::now();
        // tick our state
        state.update(dt);
        // attempt to render a frame
        match state.render() {
          Ok(_) => {}
          Err(e) => {
            log::error!("{e}");
            event_loop.exit();
          }
        }
      }
      WindowEvent::MouseInput {
        state: btn_state,
        button,
        ..
      } => state.handle_mouse_button(button, btn_state.is_pressed()),
      WindowEvent::MouseWheel {delta, ..} => {
        state.handle_mouse_scroll(&delta);
      }
      WindowEvent::KeyboardInput {
        event:
          KeyEvent {
            physical_key: PhysicalKey::Code(code),
            state: key_state,
            ..
          },
        ..
      } => state.handle_key(event_loop,code, key_state.is_pressed()),
      _ => {}
    }
  }
}

pub fn run() -> anyhow::Result<()> {
  // we should use a better logger, they set up some outdated logger ting
  let event_loop = EventLoop::with_user_event().build()?;
  let mut app = App::new();
  event_loop.run_app(&mut app)?;
  Ok(())
}
