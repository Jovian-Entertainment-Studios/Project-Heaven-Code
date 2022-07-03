use egui::{FontDefinitions, FontFamily};
use glam::{DVec2, Mat4, Quat, Vec3, Vec3A};
use histogram::Histogram;
use instant::Instant;
use rend3::util::typedefs::FastHashMap;
use serde::Deserialize;
use std::borrow::Cow;
use std::{collections::HashMap, hash::BuildHasher, sync::Arc};
use winit::event::{ElementState, KeyboardInput, MouseButton};

mod physics;

mod controls;
use controls::ship_cam;
use controls::space_cam;
use controls::ShipCam;
use controls::SpaceCam;

mod platform;

mod mesh_importer;
use mesh_importer::load_gltf;

fn button_pressed<Hash: BuildHasher>(map: &HashMap<u32, bool, Hash>, key: u32) -> bool {
    map.get(&key).map_or(false, |b| *b)
}

#[derive(Deserialize)]
struct StarData {
    ra: f64,
    dec: f64,
    plx: f64,
    gmag: f64,
}

struct RenderingData {
    _station_handle: rend3::types::ObjectHandle,
    _station_material_handle: rend3::types::MaterialHandle,

    _object_handle: std::vec::Vec<rend3::types::ObjectHandle>,
    _player_handle: rend3::types::ObjectHandle,
    _material_handle: std::vec::Vec<rend3::types::MaterialHandle>,
    player_material_handle: rend3::types::MaterialHandle,
    _directional_handle: rend3::types::DirectionalLightHandle,

    egui_routine: rend3_egui::EguiRenderRoutine,
    platform: egui_winit_platform::Platform,
    start_time: instant::Instant,
    color: [f32; 4],

    walk_speed: f32,
    run_speed: f32,

    camera_pitch: f32,
    camera_yaw: f32,
    camera_roll: f32,

    camera_pitch_keyboard: f32,
    camera_yaw_keyboard: f32,

    ship_yaw: f32,
    ship_pitch: f32,
    ship_roll: f32,

    rotation: Quat,

    camera_rotation: Quat,

    camera_relative_rotation: Quat,

    ship_location: Vec3A,
    ship_rotation: Quat,

    side: Vec3A,
    up: Vec3A,
    forward: Vec3A,

    ship_side: Vec3A,
    ship_up: Vec3A,
    ship_forward: Vec3A,

    acceleration_max: f32,
    acceleration: f32,
    velocity_vec: Vec3A,

    camera_location: Vec3A,
    timestamp_last_frame: Instant,
    timestamp_last_second: Instant,
    frame_times: Histogram,

    camtype: bool,

    camera_fixture: u32,

    view: Mat4,
}

const SAMPLE_COUNT: rend3::types::SampleCount = rend3::types::SampleCount::Four;

#[derive(Default)]
pub struct Rendering {
    menu_toggle: bool,
    project_heaven_logo: egui::TextureId,

    grabber: Option<rend3_framework::Grabber>,
    scancode_status: FastHashMap<u32, bool>,

    data: Option<RenderingData>,
}

impl rend3_framework::App for Rendering {
    const HANDEDNESS: rend3::types::Handedness = rend3::types::Handedness::Left;

    fn sample_count(&self) -> rend3::types::SampleCount {
        SAMPLE_COUNT
    }

    fn setup(
        &mut self,
        window: &winit::window::Window,
        renderer: &Arc<rend3::Renderer>,
        _routines: &Arc<rend3_framework::DefaultRoutines>,
        surface_format: rend3::types::TextureFormat,
    ) {
        self.grabber = Some(rend3_framework::Grabber::new(window));

        let window_size = window.inner_size();

        // Create the egui render routine
        let mut egui_routine = rend3_egui::EguiRenderRoutine::new(
            renderer,
            surface_format,
            rend3::types::SampleCount::One,
            window_size.width,
            window_size.height,
            window.scale_factor() as f32,
        );
        // Create mesh and calculate smooth normals based on vertices.
        //
        // We do not need to keep these handles alive once we make the object
        let (sphere_mesh, _material) = load_gltf(
            renderer,
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/data/3d/Sphere_low.glb"),
        );

        let (player_mesh, _material) = load_gltf(
            renderer,
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/data/3d/Heaven1_2.glb"),
        );

        let (station_mesh, _station_material_handle) = load_gltf(
            renderer,
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/data/3d/Stardrifter.glb"),
        );

        let mut star_data: std::vec::Vec<StarData> = vec![];
        match spv_rs::input_data::parse_csv("src/data/stars/edr3_10gmag.csv", true, b',', b'\n') {
            Ok(vec) => star_data = vec,
            Err(ex) => {
                println!("ERROR -> {}", ex);
            }
        };

        // Add PBR material with all defaults except a single color.
        let player_material = rend3_routine::pbr::PbrMaterial {
            albedo: rend3_routine::pbr::AlbedoComponent::Value(glam::Vec4::new(1., 1., 1., 1.)),
            transparency: rend3_routine::pbr::Transparency::Opaque,
            ..rend3_routine::pbr::PbrMaterial::default()
        };

        let player_material_handle = renderer.add_material(player_material);

        let mut material_vec = Vec::new();

        let player = rend3::types::Object {
            mesh_kind: rend3::types::ObjectMeshKind::Static(player_mesh),
            material: player_material_handle.clone(),
            transform: glam::Mat4::from_scale_rotation_translation(
                glam::Vec3::new(1., 1., -1.),
                rend3::types::glam::Quat::IDENTITY,
                glam::Vec3::new(0.0, 0.0, 0.0),
            ),
        };

        let station = rend3::types::Object {
            mesh_kind: rend3::types::ObjectMeshKind::Static(station_mesh),
            material: _station_material_handle.clone(),
            transform: glam::Mat4::from_scale_rotation_translation(
                glam::Vec3::new(2., 2., -2.),
                rend3::types::glam::Quat::IDENTITY,
                glam::Vec3::new(0.0, 0.0, 0.0),
            ),
        };

        let mut object_vec = Vec::new();
        let _player_handle = renderer.add_object(player);

        let _station_handle = renderer.add_object(station);

        for i in star_data {
            if i.gmag < 7. {
                let star_material = rend3_routine::pbr::PbrMaterial {
                    albedo: rend3_routine::pbr::AlbedoComponent::Value(glam::Vec4::new(
                        1.0, 1.0, 1.0, 1.0,
                    )),
                    emissive: rend3_routine::pbr::MaterialComponent::Value(glam::Vec3::new(
                        1.0, 1.0, 1.0,
                    )),
                    ..rend3_routine::pbr::PbrMaterial::default()
                };

                let _material_handle = renderer.add_material(star_material);

                // Combine the mesh and the material with a location to give an object.
                let object = rend3::types::Object {
                    mesh_kind: rend3::types::ObjectMeshKind::Static(sphere_mesh.clone()),
                    material: _material_handle.clone(),
                    transform: glam::Mat4::from_scale_rotation_translation(
                        glam::Vec3::new(696000000000000.0, 696000000000000.0, -696000000000000.0),
                        rend3::types::glam::Quat::IDENTITY,
                        spv_rs::position::position_f32(i.plx as f32, i.ra as f32, i.dec as f32),
                    ),
                };
                // We need to keep the object alive.
                object_vec.push(renderer.add_object(object));
                material_vec.push(_material_handle.clone());
            } else {
            }
        }

        // Create a single directional light
        //
        // We need to keep the directional light handle alive.
        let _directional_handle = renderer.add_directional_light(rend3::types::DirectionalLight {
            color: glam::Vec3::new(0.95, 0.9, 0.6),
            intensity: 5.,
            // Direction will be normalized
            direction: glam::Vec3::new(-1.0, -4.0, 2.0),
            distance: 4000.0,
        });

        let mut style: egui::Style = Default::default();

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(0, 0, 0);

        style.visuals.faint_bg_color = egui::Color32::from_rgb(0, 0, 0);

        style.visuals.code_bg_color = egui::Color32::from_rgb(0, 0, 0);

        style.visuals.hyperlink_color = egui::Color32::from_rgb(255, 0, 0);

        style.visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));

        style.visuals.window_corner_radius = 0.1;

        style.visuals.button_frame = true;

        style.visuals.collapsing_header_frame = true;

        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(0, 0, 0);

        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(0, 0, 0);

        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0, 0, 0);

        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 0, 0);

        style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(0, 0, 0);

        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(0, 0, 0),
        };

        style.visuals.widgets.active.fg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(255, 0, 0),
        };

        style.visuals.widgets.hovered.fg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(255, 0, 0),
        };

        style.visuals.widgets.inactive.fg_stroke = egui::Stroke {
            width: 0.1,
            color: egui::Color32::from_rgb(50, 50, 50),
        };

        style.visuals.widgets.open.fg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(0, 0, 255),
        };

        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(50, 50, 50),
        };

        style.visuals.widgets.active.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(255, 0, 0),
        };

        style.visuals.widgets.hovered.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(255, 0, 0),
        };

        style.visuals.widgets.inactive.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(50, 50, 50),
        };

        style.visuals.widgets.open.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(0, 0, 255),
        };

        style.visuals.widgets.noninteractive.corner_radius = 0.1;

        style.visuals.widgets.active.corner_radius = 0.1;

        style.visuals.widgets.hovered.corner_radius = 0.1;

        style.visuals.widgets.inactive.corner_radius = 0.1;

        style.visuals.widgets.open.corner_radius = 0.1;

        let font_dejavusansmono = include_bytes!("data/fonts/Section9-4lXp.ttf");
        let mut font = FontDefinitions::default();

        font.font_data.insert(
            "DejaVu Sans Mono".to_string(),
            egui::FontData {
                font: Cow::from(&font_dejavusansmono[..]),
                index: 0,
            },
        );
        font.fonts_for_family
            .insert(FontFamily::Monospace, vec!["DejaVu Sans Mono".to_string()]);

        font.fonts_for_family.insert(
            FontFamily::Proportional,
            vec!["DejaVu Sans Mono".to_string()],
        );

        // Create the winit/egui integration, which manages our egui context for us.
        let platform =
            egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
                physical_width: window_size.width as u32,
                physical_height: window_size.height as u32,
                scale_factor: window.scale_factor(),
                font_definitions: font,
                style,
            });

        //Images
        let image_bytes = include_bytes!("data/images/icon_round.png");
        let image_image = image::load_from_memory(image_bytes).unwrap();
        let image_rgba = image_image.as_rgba8().unwrap();

        use image::GenericImageView;
        let dimensions = image_image.dimensions();

        let format = wgpu::TextureFormat::Rgba8UnormSrgb;

        self.project_heaven_logo = rend3_egui::EguiRenderRoutine::create_egui_texture(
            &mut egui_routine.internal,
            renderer,
            format,
            image_rgba,
            dimensions,
            Some("project_heaven_logo"),
        );

        let start_time = instant::Instant::now();
        let color: [f32; 4] = [0.0, 0.5, 0.5, 1.0];

        self.data = Some(RenderingData {
            _station_handle,
            _station_material_handle,
            _object_handle: object_vec,
            _player_handle,
            _material_handle: material_vec,
            player_material_handle,
            _directional_handle,

            egui_routine,
            platform,
            start_time,
            color,

            walk_speed: 10., /*000000000000*/
            run_speed: 20.,  /*000000000000*/

            camera_pitch: 0.,
            camera_yaw: 0.,
            camera_roll: 0.,

            camera_pitch_keyboard: 0.,
            camera_yaw_keyboard: 0.,

            ship_yaw: 0.,
            ship_pitch: 0.,
            ship_roll: 0.,

            rotation: Quat::IDENTITY,

            camera_rotation: Quat::IDENTITY,

            camera_relative_rotation: Quat::IDENTITY,

            ship_location: Vec3A::ZERO,
            ship_rotation: Quat::IDENTITY,

            side: Vec3A::X,
            up: Vec3A::Y,
            forward: Vec3A::Z,

            ship_side: Vec3A::X,
            ship_up: Vec3A::Y,
            ship_forward: Vec3A::Z,

            acceleration_max: 2.,
            acceleration: 0.,
            velocity_vec: Vec3A::ZERO,

            camera_location: Vec3A::ZERO,
            timestamp_last_frame: Instant::now(),
            timestamp_last_second: Instant::now(),
            frame_times: Histogram::new(),

            camtype: false,
            camera_fixture: 0,

            view: Mat4::IDENTITY,
        })
    }

    fn handle_event(
        &mut self,
        window: &winit::window::Window,
        renderer: &Arc<rend3::Renderer>,
        routines: &Arc<rend3_framework::DefaultRoutines>,
        base_rendergraph: &rend3_routine::base::BaseRenderGraph,
        surface: Option<&Arc<rend3::types::Surface>>,
        resolution: glam::UVec2,
        event: rend3_framework::Event<'_, ()>,
        control_flow: impl FnOnce(winit::event_loop::ControlFlow),
    ) {
        let data = self.data.as_mut().unwrap();

        // Pass the winit events to the platform integration.
        data.platform.handle_event(&event);

        let now = Instant::now();
        let delta_time = now - data.timestamp_last_frame;
        data.frame_times
            .increment(delta_time.as_micros() as u64)
            .unwrap();

        let elapsed_since_second = now - data.timestamp_last_second;
        if elapsed_since_second > std::time::Duration::from_secs(1) {
            let count = data.frame_times.entries();
            println!(
                "{:0>5} frames over {:0>5.2}s. \
                        Min: {:0>5.2}ms; \
                        Average: {:0>5.2}ms; \
                        95%: {:0>5.2}ms; \
                        99%: {:0>5.2}ms; \
                        Max: {:0>5.2}ms; \
                        StdDev: {:0>5.2}ms",
                count,
                elapsed_since_second.as_secs_f32(),
                data.frame_times.minimum().unwrap() as f32 / 1_000.0,
                data.frame_times.mean().unwrap() as f32 / 1_000.0,
                data.frame_times.percentile(95.0).unwrap() as f32 / 1_000.0,
                data.frame_times.percentile(99.0).unwrap() as f32 / 1_000.0,
                data.frame_times.maximum().unwrap() as f32 / 1_000.0,
                data.frame_times.stddev().unwrap() as f32 / 1_000.0,
            );
            data.timestamp_last_second = now;
            data.frame_times.clear();
        }

        data.timestamp_last_frame = now;

        if button_pressed(&self.scancode_status, platform::Scancodes::PERIOD) {
            data.camtype = !data.camtype;
        }

        if button_pressed(&self.scancode_status, platform::Scancodes::CTRL) {
            data.camera_fixture = (1 + data.camera_fixture) % 2;
        }

        if data.camtype == true {
            let cam_data = space_cam(
                SpaceCam {
                    camera_yaw: data.camera_yaw,
                    camera_pitch: data.camera_pitch,
                    camera_roll: data.camera_roll,
                    rotation: data.camera_rotation,
                    side: data.side,
                    up: data.up,
                    forward: data.forward,
                    run_speed: data.run_speed,
                    walk_speed: data.walk_speed,
                    delta_time,
                    camera_location: data.camera_location,
                },
                &self.scancode_status,
            );

            data.camera_rotation = cam_data.0;
            data.camera_location = cam_data.1;
            data.camera_roll = cam_data.2;

            data.camera_pitch = 0.;
            data.camera_yaw = 0.;
        } else {
            let cam_data = ship_cam(
                ShipCam {
                    camera_yaw: data.camera_yaw_keyboard,
                    camera_pitch: data.camera_pitch_keyboard,

                    ship_yaw: data.ship_yaw,
                    ship_pitch: data.ship_pitch,
                    ship_roll: data.ship_roll,

                    ship_side: data.ship_side,
                    ship_up: data.ship_up,
                    ship_forward: data.ship_forward,

                    acceleration_max: data.acceleration_max,
                    acceleration: data.acceleration,
                    velocity_vec: data.velocity_vec,

                    delta_time,

                    camera_location: data.camera_location,
                    camera_relative_rotation: data.camera_relative_rotation,
                    camera_rotation: data.rotation,

                    ship_location: data.ship_location,
                    ship_rotation: data.ship_rotation,
                },
                &self.scancode_status,
            );

            data.acceleration = cam_data.0;
            data.velocity_vec = cam_data.7;

            data.ship_yaw = cam_data.1;
            data.ship_pitch = cam_data.2;
            data.ship_roll = cam_data.3;

            data.ship_rotation = cam_data.4;
            data.ship_location = cam_data.5;

            data.rotation = cam_data.6;

            data.camera_yaw_keyboard = cam_data.8;
            data.camera_pitch_keyboard = cam_data.9;

            // Acceleration, velocity and position debug
            /*
            println!(
                "{:?}        {:?}          {:?}",
                data.acceleration, data.velocity_vec, data.ship_location
            );
            */

            data.camera_rotation = data.rotation;

            if data.camera_fixture == 0 {
                data.camera_location = data.ship_location
                    + Quat::mul_vec3a(data.ship_rotation, Vec3A::new(0., -16.5512, 90.));
            } else {
                data.camera_location = data.ship_location
                    + Quat::mul_vec3a(data.ship_rotation, Vec3A::new(0., 25., -10.));
            }

            rend3::Renderer::set_object_transform(
                renderer,
                &data._player_handle,
                glam::Mat4::from_scale_rotation_translation(
                    glam::Vec3::new(1., 1., -1.),
                    data.ship_rotation,
                    Vec3::from(data.ship_location),
                ),
            );

            data.camera_pitch = 0.;
            data.camera_yaw = 0.;
        }

        if button_pressed(&self.scancode_status, platform::Scancodes::ESCAPE) {
            self.grabber.as_mut().unwrap().request_ungrab(window);
        }

        match event {
            rend3_framework::Event::RedrawRequested(..) => {
                data.platform
                    .update_time(data.start_time.elapsed().as_secs_f64());
                data.platform.begin_frame();

                // Insert egui commands here
                let ctx = data.platform.context();
                egui::TopBottomPanel::top("Taskbar").show(&ctx, |ui| {
                    if ui.add(egui::Button::new("Menu")).clicked() {
                        self.menu_toggle = !self.menu_toggle;
                    }
                    if self.menu_toggle == true {
                        egui::Window::new("Change color")
                            .resizable(false)
                            .anchor(egui::Align2::LEFT_TOP, [3.0, 30.0])
                            .show(&ctx, |ui| {
                                if ui.add(egui::Button::new("Camera fixture")).clicked() {
                                    data.camera_fixture = !data.camera_fixture;
                                }
                                if ui.add(egui::Button::new("exit")).clicked() {
                                    std::process::exit(1);
                                }
                                ui.label("Change the color of the cube");
                                if ui
                                    .color_edit_button_rgba_unmultiplied(&mut data.color)
                                    .changed()
                                {
                                    renderer.update_material(
                                        &data.player_material_handle.clone(),
                                        rend3_routine::pbr::PbrMaterial {
                                            albedo: rend3_routine::pbr::AlbedoComponent::Value(
                                                glam::Vec4::from(data.color),
                                            ),
                                            transparency: rend3_routine::pbr::Transparency::Blend,
                                            ..rend3_routine::pbr::PbrMaterial::default()
                                        },
                                    );
                                }
                            });
                    }
                });

                // End the UI frame. Now let's draw the UI with our Backend, we could also
                // handle the output here
                let (_output, paint_commands) = data.platform.end_frame(Some(window));
                let paint_jobs = data.platform.context().tessellate(paint_commands);

                let input = rend3_egui::Input {
                    clipped_meshes: &paint_jobs,
                    context: data.platform.context(),
                };

                if data.camtype == true {
                    data.view = Mat4::from_quat(data.camera_rotation);
                } else {
                    data.view = Mat4::from_quat(data.camera_rotation.inverse());
                }
                data.view = data.view * Mat4::from_translation((-data.camera_location).into());

                renderer.set_camera_data(rend3::types::Camera {
                    projection: rend3::types::CameraProjection::Perspective {
                        vfov: 60.0,
                        near: 0.1,
                    },
                    view: data.view,
                });

                // Get a frame
                let frame = rend3::util::output::OutputFrame::Surface {
                    surface: Arc::clone(surface.unwrap()),
                };

                // Ready up the renderer
                let (cmd_bufs, ready) = renderer.ready();

                // Lock the routines
                let pbr_routine = rend3_framework::lock(&routines.pbr);
                let tonemapping_routine = rend3_framework::lock(&routines.tonemapping);

                // Build a rendergraph
                let mut graph = rend3::graph::RenderGraph::new();

                // Add the default rendergraph without a skybox
                base_rendergraph.add_to_graph(
                    &mut graph,
                    &ready,
                    &pbr_routine,
                    None,
                    &tonemapping_routine,
                    resolution,
                    SAMPLE_COUNT,
                    glam::Vec4::splat(0.),
                );

                // Add egui on top of all the other passes
                let surface = graph.add_surface_texture();
                data.egui_routine.add_to_graph(&mut graph, input, surface);

                // Dispatch a render using the built up rendergraph!
                graph.execute(renderer, frame, cmd_bufs, &ready);

                window.request_redraw();
                control_flow(winit::event_loop::ControlFlow::Poll);
            }
            rend3_framework::Event::WindowEvent {
                event: winit::event::WindowEvent::Focused(focus),
                ..
            } => {
                if !focus {
                    self.grabber.as_mut().unwrap().request_ungrab(window);
                }
            }
            rend3_framework::Event::WindowEvent {
                event:
                    winit::event::WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                scancode, state, ..
                            },
                        ..
                    },
                ..
            } => {
                self.scancode_status.insert(
                    scancode,
                    match state {
                        winit::event::ElementState::Pressed => true,
                        winit::event::ElementState::Released => false,
                    },
                );
            }
            rend3_framework::Event::WindowEvent {
                event:
                    winit::event::WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let grabber = self.grabber.as_mut().unwrap();

                if !grabber.grabbed() {
                    grabber.request_grab(window);
                }
            }
            rend3_framework::Event::DeviceEvent {
                event:
                    winit::event::DeviceEvent::MouseMotion {
                        delta: (delta_x, delta_y),
                        ..
                    },
                ..
            } => {
                if !self.grabber.as_ref().unwrap().grabbed() {
                    return;
                }

                let mouse_delta = DVec2::new(delta_x, delta_y);

                data.camera_yaw -= (mouse_delta.x / 2000.0) as f32;
                data.camera_pitch -= (mouse_delta.y / 2000.0) as f32;

                /*
                if data.camera_yaw < 0.0 {
                    data.camera_yaw += TAU;
                } else if data.camera_yaw >= TAU {
                    data.camera_yaw -= TAU;
                }
                data.camera_pitch = data
                    .camera_pitch
                    .max(-std::f32::consts::FRAC_PI_2 + 0.0001)
                    .min(std::f32::consts::FRAC_PI_2 - 0.0001);
                    */
            }
            rend3_framework::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    data.egui_routine
                        .resize(size.width, size.height, window.scale_factor() as f32);
                }
                winit::event::WindowEvent::CloseRequested => {
                    control_flow(winit::event_loop::ControlFlow::Exit);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
