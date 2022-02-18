use egui::{FontDefinitions, FontFamily};
use egui_wgpu_backend::RenderPass;
use std::borrow::Cow;
use std::sync::Arc;

mod mesh_generator;
//use mesh_generator::create_mesh;

mod mesh_importer;
use mesh_importer::load_gltf;

struct RenderingData {
    _object_handle: rend3::types::ObjectHandle,
    material_handle: rend3::types::MaterialHandle,
    _directional_handle: rend3::types::DirectionalLightHandle,

    egui_routine: rend3_egui::EguiRenderRoutine,
    platform: egui_winit_platform::Platform,
    start_time: instant::Instant,
    color: [f32; 4],
}

const SAMPLE_COUNT: rend3::types::SampleCount = rend3::types::SampleCount::One;

#[derive(Default)]
pub struct Rendering {
    data: Option<RenderingData>,
    menu_toggle: bool,
    gltf_cube_toggle: bool,
    placeholder_img: egui::TextureId,
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
        let (mesh, material) = load_gltf(
            renderer,
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/data/3d/Asteroids.glb"),
        );

        // Add PBR material with all defaults except a single color.
        let material = rend3_routine::pbr::PbrMaterial {
            albedo: rend3_routine::pbr::AlbedoComponent::Value(glam::Vec4::new(0.0, 0.5, 0.5, 1.0)),
            transparency: rend3_routine::pbr::Transparency::Blend,
            ..rend3_routine::pbr::PbrMaterial::default()
        };
        let material_handle = renderer.add_material(material);

        // Combine the mesh and the material with a location to give an object.
        let object = rend3::types::Object {
            mesh_kind: rend3::types::ObjectMeshKind::Static(mesh),
            material: material_handle.clone(),
            transform: glam::Mat4::from_scale(glam::Vec3::new(1.0, 1.0, -1.0)),
        };

        // We need to keep the object alive.
        let _object_handle = renderer.add_object(object);

        //self.object_handle = Some(renderer.add_object(object));
        let view_location = glam::Vec3::new(3.0, 3.0, -5.0);
        let view = glam::Mat4::from_euler(glam::EulerRot::XYZ, -0.55, 0.5, 0.0);
        let view = view * glam::Mat4::from_translation(-view_location);

        // Set camera's location
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
            intensity: 1.0,
            // Direction will be normalized
            direction: glam::Vec3::new(-1.0, -4.0, 2.0),
            distance: 400.0,
        });

        let mut style: egui::Style = Default::default();

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.faint_bg_color = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.code_bg_color = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.hyperlink_color = egui::Color32::from_rgb(255, 0, 0);

        style.visuals.override_text_color = Some(egui::Color32::from_rgb(173, 186, 199));

        style.visuals.window_corner_radius = 10.0;

        style.visuals.button_frame = true;

        style.visuals.collapsing_header_frame = true;

        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(35, 39, 46);

        style.visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(0., egui::Color32::from_rgb(173, 186, 199));

        style.visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;

        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(45, 51, 59);

        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke {
            width: 10.0,
            color: egui::Color32::from_rgb(173, 186, 199),
        };

        let font_dejavusansmono = include_bytes!("data/fonts/DejaVuSansMono.ttf");
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

        // Create the winit/egui integration, which manages our egui context for us.
        let platform =
            egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
                physical_width: window_size.width as u32,
                physical_height: window_size.height as u32,
                scale_factor: window.scale_factor(),
                font_definitions: font,
                style: style,
            });

        //Images

        let device = &renderer.device;
        let queue = &renderer.queue;

        let image_bytes = include_bytes!("data/images/icon_round.png");
        let image_image = image::load_from_memory(image_bytes).unwrap();
        let image_rgba = image_image.as_rgba8().unwrap();

        use image::GenericImageView;
        let dimensions = image_image.dimensions();

        self.placeholder_img = rend3_egui::EguiRenderRoutine::image_to_egui(
            &mut egui_routine.internal, renderer, image_rgba, dimensions,
        );

        let start_time = instant::Instant::now();
        let color: [f32; 4] = [0.0, 0.5, 0.5, 1.0];

        self.data = Some(RenderingData {
            _object_handle,
            material_handle,
            _directional_handle,

            egui_routine,
            platform,
            start_time,
            color,
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
                                if ui.add(egui::Button::new("GLTF/Cube")).clicked() {
                                    self.gltf_cube_toggle = !self.gltf_cube_toggle;
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
                                        &data.material_handle.clone(),
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
                egui::TopBottomPanel::bottom("appdock").show(&ctx, |ui| {
                    if ui
                        .add(egui::widgets::ImageButton::new(
                            self.placeholder_img,
                            egui::Vec2::splat(256.0),
                        ))
                        .clicked()
                    {}
                });

                // End the UI frame. Now let's draw the UI with our Backend, we could also
                // handle the output here
                let (_output, paint_commands) = data.platform.end_frame(Some(window));
                let paint_jobs = data.platform.context().tessellate(paint_commands);

                let input = rend3_egui::Input {
                    clipped_meshes: &paint_jobs,
                    context: data.platform.context(),
                };

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
                    glam::Vec4::ZERO,
                );

                // Add egui on top of all the other passes
                let surface = graph.add_surface_texture();
                data.egui_routine.add_to_graph(&mut graph, input, surface);

                // Dispatch a render using the built up rendergraph!
                graph.execute(renderer, frame, cmd_bufs, &ready);

                control_flow(winit::event_loop::ControlFlow::Poll);
            }
            rend3_framework::Event::MainEventsCleared => {
                window.request_redraw();
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
