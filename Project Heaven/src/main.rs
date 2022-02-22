mod rend3_impl;
use rend3_impl::Rendering;

use winit::window::Fullscreen;

fn main() {
    let stringrecord = spv_rs::parse_csv_deserialize("src/data/stars/FK6.csv");
    println!("{:?}", stringrecord);
    let image_data_icon = include_bytes!("data/images/icon_round.png");
    let image_icon = image::load_from_memory(image_data_icon).expect("Failed to load image");
    let image_buffer_icon = image_icon.to_rgba8();
    let pixels_icon = image_buffer_icon.into_vec();

    let app = Rendering::default();
    rend3_framework::start(
        app,
        winit::window::WindowBuilder::new()
            .with_title("Project Heaven")
            .with_maximized(true)
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .with_decorations(false)
            .with_window_icon(Some(
                winit::window::Icon::from_rgba(pixels_icon, 256, 256).unwrap(),
            )),
    )
}

//Ignore this stuff ;) it's from a few months ago when the project only had 2D rendering
/*
use crate::epaint::TextureId;
use eframe::{egui, epi};
use egui::{epaint, FontDefinitions, FontFamily, Vec2};
use input_actions::binding::ActionSetId;
use input_actions::binding::{self, ActionMap, ActionSet};

use input_actions::{
    action::Action,
    binding::LayoutId,
    device::GamepadKind,
    source::{self, Kind},
    System,
};
use rodio::{source::Source, Decoder, OutputStream};
use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use egui_winit_platform::{Platform, PlatformDescriptor};
use glam::UVec2;
use std::{sync::Arc, time::Instant};
use winit::{event::Event::*, event_loop::ControlFlow};

mod advanced_comms;
mod comms;
mod guppi_terminal;
mod research;
mod system_starchart;
mod todo;
mod universe_starchart;


fn main() {
    let app_state = AppSource {
        guppi_terminal: guppi_terminal::GuppiTermial { open: false },
        todo: todo::TodoTermial { open: false },
        comms: comms::CommsTermial { open: false },
        universe_starchart: universe_starchart::UniverseStarchart { active: false },
        system_starchart: system_starchart::SystemStarcharct { active: false },
        advanced_comms: advanced_comms::AdvancedCommsTerminal { active: false },
        research: research::ResearchTerminal { active: false },
        exitmenu: false,
        feedbackmenu: false,
        feedback_string: "".to_string(),
        sad: false,
        medium_sad: false,
        medium: false,
        medium_happy: false,
        happy: false,
    };

    let image_data_bkg = include_bytes!("data/images/Carina_Nebula_Detail.png");
        let image_bkg = image::load_from_memory(image_data_bkg).expect("Failed to load image");
        let image_buffer_bkg = image_bkg.to_rgba8();
        let size_bkg = (5200 as usize, 2500 as usize);
        let pixels_bkg = image_buffer_bkg.into_vec();
        let pixels_bkg: Vec<_> = pixels_bkg
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        // Allocate a texture:
        self.bkg_img = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size_bkg, &pixels_bkg);



        let image_data_placeholder = include_bytes!("data/images/placeholder.png");
        let image_placeholder =
            image::load_from_memory(image_data_placeholder).expect("Failed to load image");
        let image_buffer_placeholder = image_placeholder.to_rgba8();
        let size_placeholder = (800 as usize, 800 as usize);
        let pixels_placeholder = image_buffer_placeholder.into_vec();
        let pixels_placeholder: Vec<_> = pixels_placeholder
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        // Allocate a texture:
        self.placeholder_img = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size_placeholder, &pixels_placeholder);



        let image_data_guppi = include_bytes!("data/images/GUPPI.png");
        let image_guppi = image::load_from_memory(image_data_guppi).expect("Failed to load image");
        let image_buffer_guppi = image_guppi.to_rgba8();
        let size_guppi = (28 as usize, 28 as usize);
        let pixels_guppi = image_buffer_guppi.into_vec();
        let pixels_guppi: Vec<_> = pixels_guppi
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        // Allocate a texture:
        self.guppi_img = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size_guppi, &pixels_guppi);

        let image_data_galaxy_placeholder = include_bytes!("data/images/Milky_Way_Galaxy.png");
        let image_galaxy_placeholder =
            image::load_from_memory(image_data_galaxy_placeholder).expect("Failed to load image");
        let image_buffer_galaxy_placeholder = image_galaxy_placeholder.to_rgba8();
        let size_galaxy_placeholder = (2795 as usize, 2795 as usize);
        let pixels_galaxy_placeholder = image_buffer_galaxy_placeholder.into_vec();
        let pixels_galaxy_placeholder: Vec<_> = pixels_galaxy_placeholder
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        // Allocate a texture:
        self.placeholder_universe_starchart_img = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size_galaxy_placeholder, &pixels_galaxy_placeholder);



        let image_data_system_placeholder = include_bytes!("data/images/SolarSystem.png");
        let image_system_placeholder =
            image::load_from_memory(image_data_system_placeholder).expect("Failed to load image");
        let image_buffer_system_placeholder = image_system_placeholder.to_rgba8();
        let size_system_placeholder = (2121 as usize, 1414 as usize);
        let pixels_system_placeholder = image_buffer_system_placeholder.into_vec();
        let pixels_system_placeholder: Vec<_> = pixels_system_placeholder
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        // Allocate a texture:
        self.placeholder_system_starchart_img = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size_system_placeholder, &pixels_system_placeholder);
        let image_data_research_placeholder = include_bytes!("data/images/Research.png");
        let image_research_placeholder =
            image::load_from_memory(image_data_research_placeholder).expect("Failed to load image");
        let image_buffer_research_placeholder = image_research_placeholder.to_rgba8();
        let size_research_placeholder = (960 as usize, 420 as usize);
        let pixels_research_placeholder = image_buffer_research_placeholder.into_vec();
        let pixels_research_placeholder: Vec<_> = pixels_research_placeholder
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        // Allocate a texture:
        self.placeholder_research_img = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size_research_placeholder, &pixels_research_placeholder);
        let image_data_comms_placeholder = include_bytes!("data/images/Communicating-5.png");
        let image_comms_placeholder =
            image::load_from_memory(image_data_comms_placeholder).expect("Failed to load image");
        let image_buffer_comms_placeholder = image_comms_placeholder.to_rgba8();
        let size_comms_placeholder = (1000 as usize, 750 as usize);
        let pixels_comms_placeholder = image_buffer_comms_placeholder.into_vec();
        let pixels_comms_placeholder: Vec<_> = pixels_comms_placeholder
            .chunks_exact(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        // Allocate a texture:
        self.placeholder_comms_img = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size_comms_placeholder, &pixels_comms_placeholder);


    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("data_export/music/Bob_Frame-Rate.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Setup logging
    env_logger::init();

    // Create event loop and window
    let event_loop = winit::event_loop::EventLoop::new();
    let window = {
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title("The Bobiverse");
        builder.build(&event_loop).expect("Could not build window")
    };

    let window_size = window.inner_size();

    // Create the Instance, Adapter, and Device. We can specify preferred backend, device name, or rendering mode. In this case we let rend3 choose for us.
    let iad = pollster::block_on(rend3::create_iad(None, None, None)).unwrap();

    // The one line of unsafe needed. We just need to guarentee that the window outlives the use of the surface.
    let surface = unsafe { Arc::new(iad.instance.create_surface(&window)) };
    // Get the preferred format for the surface.
    let format = surface.get_preferred_format(&iad.adapter).unwrap();
    // Configure the surface to be ready for rendering.
    rend3::configure_surface(
        &surface,
        &iad.device,
        format,
        UVec2::new(window_size.width, window_size.height),
        rend3::types::PresentMode::Mailbox,
    );

    // Make us a renderer.
    let renderer = rend3::Renderer::new(
        iad,
        Some(window_size.width as f32 / window_size.height as f32),
    )
    .unwrap();

    // Create the egui render egui_routine
    let mut egui_routine = rend3_egui::EguiRenderRoutine::new(
        &renderer,
        format,
        1, // For now this has to be 1, until rendergraphs support multisampling
        window_size.width,
        window_size.height,
        window.scale_factor() as f32,
    );

    let render_texture_options = rend3_pbr::RenderTextureOptions {
        resolution: UVec2::new(window_size.width, window_size.height),
        samples: rend3_pbr::SampleCount::One,
    };

    // Create the pbr pipeline with the same internal resolution and 4x multisampling
    let mut pbr_routine = rend3_pbr::PbrRenderRoutine::new(&renderer, render_texture_options);

    let mut tonemapping_routine =
        rend3_pbr::TonemappingRoutine::new(&renderer, render_texture_options.resolution, format);

    pbr_routine.set_ambient_color(glam::Vec4::new(0.15, 0.15, 0.15, 1.0));

    // Create mesh and calculate smooth normals based on vertices
    let mesh = create_mesh();

    // Add mesh to renderer's world.
    //
    // All handles are refcounted, so we only need to hang onto the handle until we make an object.
    let mesh_handle = renderer.add_mesh(mesh);

    // Add PBR material with all defaults except a single color.
    let material = rend3_pbr::material::PbrMaterial {
        albedo: rend3_pbr::material::AlbedoComponent::Value(glam::Vec4::new(0.0, 0.5, 0.5, 1.0)),
        ..rend3_pbr::material::PbrMaterial::default()
    };
    let material_handle = renderer.add_material(material);

    // Combine the mesh and the material with a location to give an object.
    let object = rend3::types::Object {
        mesh: mesh_handle,
        material: material_handle.clone(),
        transform: glam::Mat4::IDENTITY,
    };

    // Creating an object will hold onto both the mesh and the material
    // even if they are deleted.
    //
    // We need to keep the object handle alive.
    let _object_handle = renderer.add_object(object);

    let camera_pitch = std::f32::consts::FRAC_PI_4;
    let camera_yaw = -std::f32::consts::FRAC_PI_4;
    // These values may seem arbitrary, but they center the camera on the cube in the scene
    let camera_location = glam::Vec3A::new(5.0, 7.5, -5.0);
    let view = glam::Mat4::from_euler(glam::EulerRot::XYZ, -camera_pitch, -camera_yaw, 0.0);
    let view = view * glam::Mat4::from_translation((-camera_location).into());

    // Set camera location data
    renderer.set_camera_data(rend3::types::Camera {
        projection: rend3::types::CameraProjection::Perspective {
            vfov: 60.0,
            near: 0.1,
        },
        view,
    });

    // Create a single directional light
    //
    // We need to keep the directional light handle alive.
    let _directional_handle = renderer.add_directional_light(rend3::types::DirectionalLight {
        color: glam::Vec3::ONE,
        intensity: 10.0,
        // Direction will be normalized
        direction: glam::Vec3::new(-1.0, -4.0, 2.0),
        distance: 400.0,
    });

    // We use the egui_winit_platform crate as the platform.
    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: window_size.width as u32,
        physical_height: window_size.height as u32,
        scale_factor: window.scale_factor(),
        font_definitions: egui::FontDefinitions::default(),
        style: Default::default(),
    });

    let start_time = Instant::now();
    let mut color: [f32; 4] = [0.0, 0.5, 0.5, 1.0];

    event_loop.run(move |event, _, control_flow| {
        // Pass the winit events to the platform integration.
        platform.handle_event(&event);

        match event {
            RedrawRequested(..) => {
                platform.update_time(start_time.elapsed().as_secs_f64());
                platform.begin_frame();

                // Insert egui commands here
                let ctx = platform.context();
                egui::TopBottomPanel::top("Task Bar").show(&ctx, |ui| {
                    egui::containers::Frame {
                        margin: Vec2::new(0., 0.),
                        corner_radius: 0.,
                        fill: egui::Color32::TRANSPARENT,
                        shadow: epaint::Shadow::big_light(),
                        ..Default::default()
                    }
                    .show(ui, |ui| {});
                });
                egui::TopBottomPanel::bottom("App Dock").show(&ctx, |ui| {});

                let mut input_sys = System::new();
                input_sys
                    // There is only 1 player/user to send inputs for.
                    .add_users(1)
                    // These action names are complete up to you.
                    // It is recommended that you store the strings as static properties
                    // so they can be referenced throughout the consuming crate.
                    .add_action("button1", Action::new(Kind::Button))
                    .add_action("button2", Action::new(Kind::Button))
                    .add_action("axis1", Action::new(Kind::Axis))
                    .add_action("axis2", Action::new(Kind::Axis))
                    // This specifies that there is 1 layout (the default layout, which is equivalent to `None`).
                    .add_layout(LayoutId::default())
                    // This adds bindings for each action for a given layout.
                    // The group of bindings per layout is called an "action set".
                    .add_action_set(
                        // This specifies the name of the set. `ActionSetId::default()` is equivalent to `None`.
                        ActionSetId::default(),
                        ActionSet::default().with(
                            // This action set contains 1 layout, the default layout added to the system above.
                            LayoutId::default(),
                            ActionMap::default()
                                .bind(
                                    "button1",
                                    vec![
                                        binding::Source::Keyboard(source::Key::Return).bound(),
                                        binding::Source::Keyboard(source::Key::NumpadEnter).bound(),
                                    ],
                                )
                                .bind(
                                    "button2",
                                    vec![
                                        binding::Source::Keyboard(source::Key::Escape).bound(),
                                    ],
                                )
                                .bind(
                                    "axis1",
                                    vec![
                                        binding::Source::Keyboard(source::Key::W)
                                            .with_modifier(1.0),
                                        binding::Source::Keyboard(source::Key::S)
                                            .with_modifier(-1.0),
                                    ],
                                )
                                .bind(
                                    "axis2",
                                    vec![
                                        binding::Source::Keyboard(source::Key::A)
                                            .with_modifier(-1.0),
                                        binding::Source::Keyboard(source::Key::D)
                                            .with_modifier(1.0),
                                    ],
                                ),
                        ),
                    )
                    // In order to use action set bindings, the user needs the action set enabled.
                    // This call says "all users should have this action set enabled",
                    // though it is equivalent to `mark_action_set_enabled` since there is only 1 user in this example.
                    .enable_action_set_for_all(ActionSetId::default());


                    egui::Window::new("EXIT?")
                        .resizable(false)
                        .anchor(egui::Align2::CENTER_CENTER, (0f32, 0f32))
                        .show(&ctx, |ui| {
                            ui.vertical(|ui| {
                                ui.label("Do you want to exit?");
                                ui.horizontal(|ui| {
                                    if ui.add(egui::Button::new("Yes")).clicked() {
                                        //frame.quit();
                                    }
                                    if ui.add(egui::Button::new("No")).clicked() {
                                        //self.exitmenu = false;
                                    }
                                });
                            });
                        });
                    egui::Window::new("Feedback:").anchor(egui::Align2::CENTER_TOP, (0f32, 50f32)).resizable(false).show(ctx, |ui| {
                            ui.add(
                                egui::Label::new(format!(
                                    "Thank you for giving feedback on our game!"
                                    ))
                                .heading(),
                            );
                            ui.add(egui::Label::new(format!(
                                "If you want to leave any comment please feel free to do so below"
                            )));
                            ui.separator();
                            let feedback = ui.add_sized(
                                [300.0, 150.0],
                                egui::TextEdit::multiline(&mut self.feedback_string),
                            );
                            ui.separator();
                            ui.horizontal(|ui| {
                                //Sad
                                if ui
                                    .add(egui::ImageButton::new(
                                        self.placeholder_img,
                                        egui::Vec2::splat(28.0),
                                    ))
                                    .clicked()
                                {
                                    self.sad = true;
                                    self.medium_sad = false;
                                    self.medium = false;
                                    self.medium_happy = false;
                                    self.happy = false;
                                }
                                //Medium sad
                                if ui
                                    .add(egui::ImageButton::new(
                                        self.placeholder_img,
                                        egui::Vec2::splat(28.0),
                                    ))
                                    .clicked()
                                {
                                    self.sad = false;
                                    self.medium_sad = true;
                                    self.medium = false;
                                    self.medium_happy = false;
                                    self.happy = false;
                                }
                                //Medium
                                if ui
                                    .add(egui::ImageButton::new(
                                        self.placeholder_img,
                                        egui::Vec2::splat(28.0),
                                    ))
                                    .clicked()
                                {
                                    self.sad = false;
                                    self.medium_sad = false;
                                    self.medium = true;
                                    self.medium_happy = false;
                                    self.happy = false;
                                }
                                //Medium happy
                                if ui
                                    .add(egui::ImageButton::new(
                                        self.placeholder_img,
                                        egui::Vec2::splat(28.0),
                                    ))
                                    .clicked()
                                {
                                    self.sad = false;
                                    self.medium_sad = false;
                                    self.medium = false;
                                    self.medium_happy = true;
                                    self.happy = false;
                                }
                                //Happy
                                if ui
                                    .add(egui::ImageButton::new(
                                        self.placeholder_img,
                                        egui::Vec2::splat(28.0),
                                    ))
                                    .clicked()
                                {
                                    self.sad = false;
                                    self.medium_sad = false;
                                    self.medium = false;
                                    self.medium_happy = false;
                                    self.happy = true;
                                }
                            });
                            ui.separator();
                            ui.horizontal(|ui| {
                                if ui.add(egui::Button::new("Submit")).clicked() {
                                    //Send feedback string & mood & position/state data
                                }
                                if ui.add(egui::Button::new("Cancel")).clicked() {
                                    drop(feedback);
                                    self.sad = false;
                                    self.medium_sad = false;
                                    self.medium = false;
                                    self.medium_happy = false;
                                    self.happy = false;
                                    self.feedbackmenu = false;
                                }
                            });
                        });
                    }


                // End the UI frame. Now let's draw the UI with our Backend, we could also handle the output here
                let (_output, paint_commands) = platform.end_frame(Some(&window));
                let paint_jobs = platform.context().tessellate(paint_commands);

                let input = rend3_egui::Input {
                    clipped_meshes: &paint_jobs,
                    context: platform.context(),
                };

                // Get a frame
                let frame = rend3::util::output::OutputFrame::Surface {
                    surface: Arc::clone(&surface),
                };

                // Ready up the renderer
                let (cmd_bufs, ready) = renderer.ready();

                // Build a rendergraph
                let mut graph = rend3::RenderGraph::new();
                // Upload culling information to the GPU and into the graph.
                pbr_routine.add_pre_cull_to_graph(&mut graph);

                // Run all culling for shadows and the camera.
                pbr_routine.add_shadow_culling_to_graph(&mut graph, &ready);
                pbr_routine.add_culling_to_graph(&mut graph);

                // Render shadows.
                pbr_routine.add_shadow_rendering_to_graph(&mut graph, &ready);

                // Depth prepass and forward pass.
                pbr_routine.add_prepass_to_graph(&mut graph);
                pbr_routine.add_forward_to_graph(&mut graph);

                // Tonemap onto the output.
                tonemapping_routine.add_to_graph(&mut graph);

                // Add egui on top of all the other passes
                egui_routine.add_to_graph(&mut graph, input);

                // Dispatch a render using the built up rendergraph!
                graph.execute(&renderer, frame, cmd_bufs, &ready);

                *control_flow = ControlFlow::Poll;
            }
            MainEventsCleared => {
                window.request_redraw();
            }
            WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    let size = UVec2::new(size.width, size.height);
                    // Reconfigure the surface for the new size.
                    rend3::configure_surface(
                        &surface,
                        &renderer.device,
                        format,
                        UVec2::new(size.x, size.y),
                        rend3::types::PresentMode::Mailbox,
                    );

                    renderer.set_aspect_ratio(size.x as f32 / size.y as f32);

                    egui_routine.resize(size.x, size.y, window.scale_factor() as f32);
                    pbr_routine.resize(
                        &renderer,
                        rend3_pbr::RenderTextureOptions {
                            resolution: size,
                            samples: rend3_pbr::SampleCount::One,
                        },
                    );
                    tonemapping_routine.resize(size);
                }
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
    /*
    let app = AppSource::default();
    let options = eframe::NativeOptions {
        always_on_top: false,
        decorated: true,
        drag_and_drop_support: false,
        initial_window_size: Some(egui::vec2(1920.0, 1080.0)),
        resizable: true,
        transparent: false,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
    */
}

fn vertex(pos: [f32; 3]) -> glam::Vec3 {
    glam::Vec3::from(pos)
}

fn create_mesh() -> rend3::types::Mesh {
    let vertex_positions = [
        // far side (0.0, 0.0, 1.0)
        vertex([-1.0, -1.0, 1.0]),
        vertex([1.0, -1.0, 1.0]),
        vertex([1.0, 1.0, 1.0]),
        vertex([-1.0, 1.0, 1.0]),
        // near side (0.0, 0.0, -1.0)
        vertex([-1.0, 1.0, -1.0]),
        vertex([1.0, 1.0, -1.0]),
        vertex([1.0, -1.0, -1.0]),
        vertex([-1.0, -1.0, -1.0]),
        // right side (1.0, 0.0, 0.0)
        vertex([1.0, -1.0, -1.0]),
        vertex([1.0, 1.0, -1.0]),
        vertex([1.0, 1.0, 1.0]),
        vertex([1.0, -1.0, 1.0]),
        // left side (-1.0, 0.0, 0.0)
        vertex([-1.0, -1.0, 1.0]),
        vertex([-1.0, 1.0, 1.0]),
        vertex([-1.0, 1.0, -1.0]),
        vertex([-1.0, -1.0, -1.0]),
        // top (0.0, 1.0, 0.0)
        vertex([1.0, 1.0, -1.0]),
        vertex([-1.0, 1.0, -1.0]),
        vertex([-1.0, 1.0, 1.0]),
        vertex([1.0, 1.0, 1.0]),
        // bottom (0.0, -1.0, 0.0)
        vertex([1.0, -1.0, 1.0]),
        vertex([-1.0, -1.0, 1.0]),
        vertex([-1.0, -1.0, -1.0]),
        vertex([1.0, -1.0, -1.0]),
    ];

    let index_data: &[u32] = &[
        0, 1, 2, 2, 3, 0, // far
        4, 5, 6, 6, 7, 4, // near
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // top
        20, 21, 22, 22, 23, 20, // bottom
    ];

    rend3::types::MeshBuilder::new(vertex_positions.to_vec())
        .with_indices(index_data.to_vec())
        .build()
}
/*
impl AppSource {
    fn popup_apps(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>, ui: &mut egui::Ui) {

        if ui.input().key_pressed(egui::Key::Z) {
            self.feedbackmenu = true;
        }


        let guppi_window =
            egui::Window::new("Terminal").anchor(egui::Align2::LEFT_TOP, (10f32, 50f32));
        if self.guppi_terminal.open || ctx.memory().everything_is_visible() {
            guppi_window.show(ctx, |ui| {
                self.guppi_terminal.ui(ui, frame);
            });
        }
        let todo_window =
            egui::Window::new("Todo").anchor(egui::Align2::RIGHT_TOP, (-10f32, 50f32));
        if self.todo.open || ctx.memory().everything_is_visible() {
            todo_window.show(ctx, |ui| {
                self.todo.ui(ui, frame);
            });
        }
        let comms_window =
            egui::Window::new("Comms").anchor(egui::Align2::RIGHT_CENTER, (-10f32, -300f32));
        if self.comms.open || ctx.memory().everything_is_visible() {
            comms_window.show(ctx, |ui| {
                self.comms.ui(ui, frame);
            });
        }
    }
    fn universe_starchart_windows(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let selected_star_window =
            egui::Window::new("Star").anchor(egui::Align2::LEFT_BOTTOM, (10f32, -50f32));
        if self.universe_starchart.active || ctx.memory().everything_is_visible() {
            selected_star_window.show(ctx, |ui| {
                self.universe_starchart.ui(ui, frame);
            });
        }
    }
    fn system_starchart_windows(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let selected_object_window =
            egui::Window::new("Object").anchor(egui::Align2::LEFT_BOTTOM, (10f32, -50f32));
        if self.system_starchart.active || ctx.memory().everything_is_visible() {
            selected_object_window.show(ctx, |ui| {
                self.system_starchart.ui(ui, frame);
            });
        }
    }
    fn research_windows(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let selected_tech_window =
            egui::Window::new("Tech").anchor(egui::Align2::LEFT_BOTTOM, (10f32, -50f32));
        if self.research.active || ctx.memory().everything_is_visible() {
            selected_tech_window.show(ctx, |ui| {
                self.research.ui(ui, frame);
            });
        }
    }
    fn advanced_comms_windows(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let selected_transmission_window =
            egui::Window::new("Comms").anchor(egui::Align2::LEFT_BOTTOM, (10f32, -50f32));
        if self.advanced_comms.active || ctx.memory().everything_is_visible() {
            selected_transmission_window.show(ctx, |ui| {
                self.advanced_comms.ui(ui, frame);
            });
        }
    }
}
*/

#[derive(Serialize, Deserialize)]
struct ColorPallet {
    eerie_black: String,
    eerie_black_r: u8,
    eerie_black_g: u8,
    eerie_black_b: u8,
    dark_slate_gray: String,
    dark_slate_gray_r: u8,
    dark_slate_gray_g: u8,
    dark_slate_gray_b: u8,
    black: String,
    black_r: u8,
    black_g: u8,
    black_b: u8,
    rich_black_fogra_39: String,
    rich_black_fogra_39_r: u8,
    rich_black_fogra_39_g: u8,
    rich_black_fogra_39_b: u8,
    rich_black_fogra_29: String,
    rich_black_fogra_29_r: u8,
    rich_black_fogra_29_g: u8,
    rich_black_fogra_29_b: u8,
    red_orange_color_wheel: String,
    red_orange_color_wheel_r: u8,
    red_orange_color_wheel_g: u8,
    red_orange_color_wheel_b: u8,
    red_ryb: String,
    red_ryb_r: u8,
    red_ryb_g: u8,
    red_ryb_b: u8,
    red_salsa: String,
    red_salsa_r: u8,
    red_salsa_g: u8,
    red_salsa_b: u8,
    redwood: String,
    redwood_r: u8,
    redwood_g: u8,
    redwood_b: u8,
    venetian_red: String,
    venetian_red_r: u8,
    venetian_red_g: u8,
    venetian_red_b: u8,
    tuscan_red: String,
    tuscan_red_r: u8,
    tuscan_red_g: u8,
    tuscan_red_b: u8,
    copper_red: String,
    copper_red_r: u8,
    copper_red_g: u8,
    copper_red_b: u8,
    pacific_blue: String,
    pacific_blue_r: u8,
    pacific_blue_g: u8,
    pacific_blue_b: u8,
    zomb: String,
    zomb_r: u8,
    zomb_g: u8,
    zomb_b: u8,
    celtic_blue: String,
    celtic_blue_r: u8,
    celtic_blue_g: u8,
    celtic_blue_b: u8,
    space_cadet: String,
    space_cadet_r: u8,
    space_cadet_g: u8,
    space_cadet_b: u8,
    prussian_blue: String,
    prussian_blue_r: u8,
    prussian_blue_g: u8,
    prussian_blue_b: u8,
    charcoal: String,
    charcoal_r: u8,
    charcoal_g: u8,
    charcoal_b: u8,
    indigo_dye: String,
    indigo_dye_r: u8,
    indigo_dye_g: u8,
    indigo_dye_b: u8,
    davys_grey: String,
    davys_grey_r: u8,
    davys_grey_g: u8,
    davys_grey_b: u8,
    jet: String,
    jet_r: u8,
    jet_g: u8,
    jet_b: u8,
    sonic_silver: String,
    sonic_silver_r: u8,
    sonic_silver_g: u8,
    sonic_silver_b: u8,
    cadet_grey: String,
    cadet_grey_r: u8,
    cadet_grey_g: u8,
    cadet_grey_b: u8,
    light_grey: String,
    light_grey_r: u8,
    light_grey_g: u8,
    light_grey_b: u8,
    platinum: String,
    platinum_r: u8,
    platinum_g: u8,
    platinum_b: u8,
    magnolia: String,
    magnolia_r: u8,
    magnolia_g: u8,
    magnolia_b: u8,
    cultured: String,
    cultured_r: u8,
    cultured_g: u8,
    cultured_b: u8,
    antique_white: String,
    antique_white_r: u8,
    antique_white_g: u8,
    antique_white_b: u8,
    sage: String,
    sage_r: u8,
    sage_g: u8,
    sage_b: u8,
    baby_powder: String,
    baby_powder_r: u8,
    baby_powder_g: u8,
    baby_powder_b: u8,
}

#[derive(Default)]

pub struct AppSource {
    guppi_terminal: guppi_terminal::GuppiTermial,
    todo: todo::TodoTermial,
    comms: comms::CommsTermial,
    universe_starchart: universe_starchart::UniverseStarchart,
    system_starchart: system_starchart::SystemStarcharct,
    advanced_comms: advanced_comms::AdvancedCommsTerminal,
    research: research::ResearchTerminal,
    /*
    bkg_img: TextureId,
    guppi_img: TextureId,
    placeholder_img: TextureId,
    placeholder_universe_starchart_img: TextureId,
    placeholder_system_starchart_img: TextureId,
    placeholder_research_img: TextureId,
    placeholder_comms_img: TextureId,
    */
    exitmenu: bool,
    feedbackmenu: bool,
    feedback_string: String,
    sad: bool,
    medium_sad: bool,
    medium: bool,
    medium_happy: bool,
    happy: bool,
}
/*
impl epi::App for AppSource {
    fn name(&self) -> &str {
        "The Bobiverse"
    }
    #[allow(unused_variables)]
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        let loaded_file = include_str!("data/color/color.json");
        let color_pallet: ColorPallet = serde_json::from_str(loaded_file).unwrap();
        let mut style: egui::Style = (*ctx.style()).clone();
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(
            color_pallet.baby_powder_r,
            color_pallet.baby_powder_g,
            color_pallet.baby_powder_b,
        );
        style.visuals.faint_bg_color = egui::Color32::from_rgb(
            color_pallet.sage_r,
            color_pallet.sage_g,
            color_pallet.sage_b,
        );
        style.visuals.code_bg_color = egui::Color32::from_rgb(
            color_pallet.platinum_r,
            color_pallet.platinum_g,
            color_pallet.platinum_b,
        );
        style.visuals.hyperlink_color = egui::Color32::from_rgb(
            color_pallet.copper_red_r,
            color_pallet.copper_red_g,
            color_pallet.copper_red_b,
        );
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(
            color_pallet.eerie_black_r,
            color_pallet.eerie_black_g,
            color_pallet.eerie_black_b,
        ));
        style.visuals.window_corner_radius = 0.1;
        style.visuals.button_frame = false;
        style.visuals.collapsing_header_frame = true;
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(
            color_pallet.platinum_r,
            color_pallet.platinum_g,
            color_pallet.platinum_b,
        );
        style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(
            color_pallet.copper_red_r,
            color_pallet.copper_red_g,
            color_pallet.copper_red_b,
        );
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(
            color_pallet.venetian_red_r,
            color_pallet.venetian_red_g,
            color_pallet.venetian_red_b,
        );
        style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(
            color_pallet.red_orange_color_wheel_r,
            color_pallet.red_orange_color_wheel_g,
            color_pallet.red_orange_color_wheel_b,
        );
        ctx.set_style(style);
        let font_ubuntu = include_bytes!("data/fonts/Ubuntu/UbuntuMono-Regular.ttf");
        let mut font = FontDefinitions::default();
        font.font_data
            .insert("Ubuntu".to_string(), Cow::from(&font_ubuntu[..]));
        font.fonts_for_family
            .insert(FontFamily::Monospace, vec!["Ubuntu".to_string()]);
        font.fonts_for_family
            .insert(FontFamily::Proportional, vec!["Ubuntu".to_string()]);
        /*
        font.family_and_size.insert(
            epaint::text::TextStyle::Body,
            (epaint::text::FontFamily::Proportional, 10.0),
        );
        font.family_and_size.insert(
            epaint::text::TextStyle::Body,
            (epaint::text::FontFamily::Monospace, 10.0),
        );
        */
        ctx.set_fonts(font);


    }
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }
    fn warm_up_enabled(&self) -> bool {
        return true;
    }
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if self.universe_starchart.active == true {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::containers::Frame {
                    margin: Vec2::new(-20., -20.),
                    corner_radius: 0.,
                    ..Default::default()
                }
                .show(ui, |ui| {
                    ui.image(self.placeholder_universe_starchart_img, [2795.0, 2795.0]);
                });
                self.popup_apps(ctx, frame, ui);
                self.universe_starchart_windows(ctx, frame);
            });
        }
        else if self.system_starchart.active == true {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::containers::Frame {
                    margin: Vec2::new(-20., -20.),
                    corner_radius: 0.,
                    ..Default::default()
                }
                .show(ui, |ui| {
                    ui.image(self.placeholder_system_starchart_img, [2121.0, 1414.0]);
                });
                self.popup_apps(ctx, frame, ui);
                self.system_starchart_windows(ctx, frame);
            });
        }
        else if self.research.active == true {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::containers::Frame {
                    margin: Vec2::new(-20., -20.),
                    corner_radius: 0.,
                    ..Default::default()
                }
                .show(ui, |ui| {
                    ui.image(self.placeholder_research_img, [960.0, 420.0]);
                });
                self.popup_apps(ctx, frame, ui);
                self.research_windows(ctx, frame);
            });
        }
        else if self.advanced_comms.active == true {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::containers::Frame {
                    margin: Vec2::new(-20., -20.),
                    corner_radius: 0.,
                    ..Default::default()
                }
                .show(ui, |ui| {
                    ui.image(self.placeholder_comms_img, [1000.0, 750.0]);
                });
                self.popup_apps(ctx, frame, ui);
                self.advanced_comms_windows(ctx, frame);
            });
        }
        else {
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::containers::Frame {
                    margin: Vec2::new(-20., -20.),
                    corner_radius: 0.,
                    ..Default::default()
                }
                .show(ui, |_ui| {
                    //ui.image(self.bkg_img, [2600.0, 1250.0]);
                });
                self.popup_apps(ctx, frame, ui);
            });
        }
        egui::TopBottomPanel::top("Task Bar").show(ctx, |ui| {
            egui::containers::Frame {
                margin: Vec2::new(0., 0.),
                corner_radius: 0.,
                fill: egui::Color32::TRANSPARENT,
                shadow: epaint::Shadow::big_light(),
                ..Default::default()
            }
            .show(ui, |ui| {
                egui::trace!(ui);
                self.bar_contents(ui, frame);
            });
        });
        egui::TopBottomPanel::bottom("App Dock").show(ctx, |ui| {
            egui::trace!(ui);
            self.app_bar_contents(ui, frame);
        });
    }
}

impl AppSource {
    fn bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>) {
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(egui::ImageButton::new(
                    self.guppi_img,
                    egui::Vec2::splat(28.0),
                ))
                .clicked()
            {
                self.guppi_terminal.open = !self.guppi_terminal.open;
            }
            if ui
                .add(egui::ImageButton::new(
                    self.placeholder_img,
                    egui::Vec2::splat(28.0),
                ))
                .clicked()
            {
                self.todo.open = !self.todo.open;
            }
            if ui
                .add(egui::ImageButton::new(
                    self.placeholder_img,
                    egui::Vec2::splat(28.0),
                ))
                .clicked()
            {
                self.comms.open = !self.comms.open;
            }
            let tau = 1;
            let currentsystem = "Epsilon Eridani";
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                ui.add(egui::Label::new(format!("12:00:00  01/01/01  τ {}", tau)));
                ui.add(egui::Label::new(format!("{} System", currentsystem)));
            });
        });
    }
    fn app_bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut epi::Frame<'_>) {
        ui.horizontal_wrapped(|ui| {
            //Home
            if ui
                .add(egui::ImageButton::new(
                    self.placeholder_img,
                    egui::Vec2::splat(28.0),
                ))
                .clicked()
            {
                self.universe_starchart.active = false;
                self.system_starchart.active = false;
                self.research.active = false;
                self.advanced_comms.active = false;
            }
            //Universe Starchart
            if ui
                .add(egui::ImageButton::new(
                    self.placeholder_img,
                    egui::Vec2::splat(28.0),
                ))
                .clicked()
            {
                self.universe_starchart.active = true;
                self.system_starchart.active = false;
                self.research.active = false;
                self.advanced_comms.active = false;
            }
            //System Starchart
            if ui
                .add(egui::ImageButton::new(
                    self.placeholder_img,
                    egui::Vec2::splat(28.0),
                ))
                .clicked()
            {
                self.universe_starchart.active = false;
                self.system_starchart.active = true;
                self.research.active = false;
                self.advanced_comms.active = false;
            }
            //Research
            if ui
                .add(egui::ImageButton::new(
                    self.placeholder_img,
                    egui::Vec2::splat(28.0),
                ))
                .clicked()
            {
                self.universe_starchart.active = false;
                self.system_starchart.active = false;
                self.research.active = true;
                self.advanced_comms.active = false;
            }
            //Comms
            if ui
                .add(egui::ImageButton::new(
                    self.placeholder_img,
                    egui::Vec2::splat(28.0),
                ))
                .clicked()
            {
                self.universe_starchart.active = false;
                self.system_starchart.active = false;
                self.research.active = false;
                self.advanced_comms.active = true;
            }
        });
    }
}
*/
*/
