//! Game project.
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use direction::Direction;
use fyrox::core::algebra::Matrix4;
use fyrox::core::log::Log;
use fyrox::core::math::TriangleDefinition;
use fyrox::engine::GraphicsContext;
use fyrox::event::DeviceEvent::MouseMotion;
use fyrox::event::DeviceEvent::Button;
use fyrox::event::MouseButton;
use fyrox::event::WindowEvent::KeyboardInput;
use fyrox::gui::draw::Vertex;
use fyrox::gui::message::MessageData;
use fyrox::material::shader::{SamplerFallback, Shader, self, ShaderResource};
use fyrox::material::{Material, PropertyValue, SharedMaterial};
use fyrox::resource::texture::Texture;
use fyrox::scene::base::BaseBuilder;
use fyrox::scene::collider::{ColliderBuilder, ColliderShape, TrimeshShape, GeometrySource};
use fyrox::scene::graph::physics::RayCastOptions;
use fyrox::scene::light::directional::DirectionalLight;
use fyrox::scene::mesh::buffer::{VertexBuffer, TriangleBuffer};
use fyrox::scene::mesh::vertex::StaticVertex;
use fyrox::scene::mesh::{MeshBuilder, Mesh, surface};
use fyrox::scene::mesh::surface::{SurfaceData, SurfaceSharedData, SurfaceBuilder, Surface};
use fyrox::scene::node::NodeTrait;
use fyrox::scene::rigidbody::RigidBodyBuilder;
use fyrox::scene::transform::TransformBuilder;
use fyrox::window::CursorGrabMode;
use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
    },
    event::{ElementState, Event},
    gui::message::UiMessage,
    keyboard::{KeyCode, PhysicalKey},
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::{node::Node, Scene},
};

use chunk::SectorBuilder;

use crate::world::World;

mod block;

mod chunk;

mod algorithm;
mod direction;
mod world;
mod world_generator;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, _context: PluginRegistrationContext) {
        // Register your scripts here.
    }

    fn create_instance(&self, scene_path: Option<&str>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(scene_path, context))
    }
}

struct InputController {
    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,
    pitch: f32,
    yaw: f32,

    mouse_right_button_pressed: bool,

}

pub struct Game {
    input_controller: InputController,
    scene: Handle<Scene>,

    indicator: Handle<Node>,
    camera: Handle<Node>,
    test: Option<Node>,
}

impl Game {
    pub fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
  
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        
        let mut surfaces: Vec<Surface> = vec![];

        for direction in Direction::iterator()
        {
            let mut triangles:Vec<TriangleDefinition> = vec![];
            let mut verticles:Vec<StaticVertex> = vec![];
            for triangle in direction.triangles()
            {
                triangles.push(TriangleDefinition([
                    verticles.len() as u32 + triangle[0],
                    verticles.len() as u32 + triangle[1],
                    verticles.len() as u32 + triangle[2],
                ]));
            }
            let mut surface_data = SurfaceData::new(
                VertexBuffer::new(4, direction.verticles()).unwrap(),
                TriangleBuffer::new(triangles),
                false,
            );
            surface_data.calculate_tangents().unwrap();
            surface_data.transform_geometry(&Matrix4::new_nonuniform_scaling(&Vector3::new(
                2.0, 2.0, 2.0,
            )))
            .unwrap();

            surfaces.push(
                SurfaceBuilder::new(SurfaceSharedData::new(surface_data))
                    // .with_material(sector_builder.grass_shader.clone())
                    .build(),
            );
        }

       let test = MeshBuilder::new(
        BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .with_local_position(Vector3::new(
                        0 as f32 * (64_u32 / 4) as f32,
                        0 as f32 * (64_u32 / 4) as f32,
                        0 as f32 * (64_u32 / 4) as f32,
                    ))
                    .build(),
            ),
        )
        .with_surfaces(surfaces)
        .build_node() ;

        Self {
            scene: Handle::NONE,
            input_controller: InputController {
                move_forward: false,
                move_backward: false,
                move_left: false,
                move_right: false,
                move_up: false,
                move_down: false,
                pitch: 0.0,
                yaw: 0.0,

                mouse_right_button_pressed: false,               
                
            },
            camera: Handle::NONE,
            indicator: Handle::NONE,
            test: Some(test),
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // self.frame_counter += 1;

        let scene = &mut _context.scenes[self.scene];

        match self.test.take()
        {
            Some(data) => 
            {
                let handle = scene.graph.add_node(data);

                let collider = ColliderBuilder::new(BaseBuilder::new())
                            .with_shape(ColliderShape::Trimesh(TrimeshShape {
                                sources: vec![GeometrySource(handle)],
                            }))
                            .build(&mut scene.graph);

                RigidBodyBuilder::new(BaseBuilder::new().with_children(&[collider]))
                                .with_body_type(fyrox::scene::rigidbody::RigidBodyType::Static)
                                .build(&mut scene.graph);
            },
            None => {}
        }

        let graph = &mut scene.graph;

        let camera_position;
        let look_direction;
        {
            let camera = &mut graph[Handle::<Node>::new(1, 1)];

            let mut offset: Vector3<f32> = Vector3::default();
            if self.input_controller.move_forward {
                offset += camera.look_vector();
            }
            if self.input_controller.move_backward {
                offset -= camera.look_vector();
            }
            if self.input_controller.move_left {
                offset += camera.side_vector();
            }
            if self.input_controller.move_right {
                offset -= camera.side_vector();
            }
            if self.input_controller.move_up {
                offset.y += 1.0
            }
            if self.input_controller.move_down {
                offset.y -= 1.0
            }

            if let Some(offset) = offset.try_normalize(f32::EPSILON) {
                camera.local_transform_mut().offset(offset.scale(0.1));
            }

            camera
                .local_transform_mut()
                .set_rotation(UnitQuaternion::from_euler_angles(
                    self.input_controller.pitch,
                    self.input_controller.yaw,
                    0.0,
                ));

            camera_position = camera.global_position().clone();
            look_direction = camera.look_vector().clone();
        }
        

        let mut intersections = Vec::new();
        
        graph.physics.cast_ray(
                RayCastOptions {
                    ray_origin: camera_position.into(),
                    ray_direction: look_direction,
                    max_len: 10.0,
                    groups: Default::default(),
                    // Sort results of the ray casting so the closest intersection will be in the
                    // beginning of the list.
                    sort_results: true,
                },
                &mut intersections,
            );
        if self.input_controller.mouse_right_button_pressed
        {
            Log::info("======================================");
            for intersection in intersections
            {
                let (x,y,z) = (0.0, 0.12162548, 0.05452454);
                let position = Vector3::new(
                    intersection.position.x + x, 
                    intersection.position.y + y,
                    intersection.position.z + z,);
                graph[self.indicator].as_mesh_mut().local_transform_mut().set_position(position );
                Log::info( format!( "{} , {}",intersection.position.coords , position ));
                break;

            }
        }
            
        
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        match _event {
            Event::WindowEvent {
                event: KeyboardInput { event: input, .. },
                ..
            } => {
                if let PhysicalKey::Code(code) = input.physical_key {
                    match code {
                        KeyCode::KeyW => {
                            self.input_controller.move_forward =
                                input.state == ElementState::Pressed
                        }
                        KeyCode::KeyS => {
                            self.input_controller.move_backward =
                                input.state == ElementState::Pressed
                        }
                        KeyCode::KeyA => {
                            self.input_controller.move_left = input.state == ElementState::Pressed
                        }
                        KeyCode::KeyD => {
                            self.input_controller.move_right = input.state == ElementState::Pressed
                        }
                        KeyCode::KeyP => {
                            let mut scene = & mut _context.scenes[self.scene];
                            scene.graph.physics.draw(&mut scene.drawing_context);
                        }
                        KeyCode::ShiftLeft => {
                            self.input_controller.move_up = input.state == ElementState::Pressed
                        }
                        KeyCode::ControlLeft => {
                            self.input_controller.move_down = input.state == ElementState::Pressed
                        }

                        _ => (),
                    }
                }
            }
            Event::DeviceEvent {
                event: MouseMotion { delta, .. },
                ..
            } => {
                let mouse_sens: f32 = 0.2 * _context.dt;
                self.input_controller.yaw -= (delta.0 as f32) * mouse_sens;
                self.input_controller.pitch = (self.input_controller.pitch
                    + (delta.1 as f32) * mouse_sens)
                    .clamp(-90.0f32.to_radians(), 90.0f32.to_radians())
            }
            Event::DeviceEvent { event: Button { button, state }, .. } => 
            {
                if *button == 0_u32 // Left mouse button
                {
                    // Log::info( format!("button = {}", button) );
                    self.input_controller.mouse_right_button_pressed = ElementState::Pressed == *state;
                }
            }
            _ => (),
        }
    }

    fn on_ui_message(&mut self, _context: &mut PluginContext, _message: &UiMessage) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        _context: &mut PluginContext,
    ) {

        if let GraphicsContext::Initialized(ref graphics_context) = _context.graphics_context {
            graphics_context.window.set_cursor_grab(
                CursorGrabMode::Confined,
            );
        }


        self.scene = scene;
        self.camera = Handle::<Node>::new(1, 1);

        self.indicator = MeshBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .build(),
            ),
        )
        .with_surfaces(vec![SurfaceBuilder::new(SurfaceSharedData::new(
            SurfaceData::make_cube(Matrix4::new_nonuniform_scaling(&Vector3::new(
                0.25, 0.25, 0.25,
            ))),
        ))
            // .with_material(SharedMaterial::new(material))
            .build()])
        .build(&mut _context.scenes[scene].graph);  

        // self.world.lock().unwrap().start_generating_thread();
    }
}
