mod model;
mod texture;
mod player_camera;
use player_camera as camera;


use cgmath;
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


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &camera::Camera, projection: &camera::Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into()
    }
}

struct Instance {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[allow(dead_code)]
struct InstanceRaw {
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3],
}

impl model::Vertex for InstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We don't have to do this in code though.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding: u32,
    color: [f32; 3],
    _padding2: u32,
}


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
  camera_uniform: CameraUniform,
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
  debug_material: model::Material,
  mouse_pressed: bool
}

impl State {
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
