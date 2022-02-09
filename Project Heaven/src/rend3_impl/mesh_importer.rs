pub fn load_gltf(
    renderer: &rend3::Renderer,
    path: &'static str,
) -> (rend3::types::MeshHandle, rend3::types::MaterialHandle) {
    let (doc, datas, _) = gltf::import(path).unwrap();
    let mesh_data = doc.meshes().next().expect("no meshes in test.glb");

    let primitive = mesh_data.primitives().next().expect("no primitives in test.glb");
    let reader = primitive.reader(|b| Some(&datas.get(b.index())?.0[..b.length()]));

    let vertex_positions: Vec<_> = reader.read_positions().unwrap().map(glam::Vec3::from).collect();
    let vertex_normals: Vec<_> = reader.read_normals().unwrap().map(glam::Vec3::from).collect();
    let vertex_tangents: Vec<_> = reader
        .read_tangents()
        .unwrap()
        .map(glam::Vec4::from)
        .map(glam::Vec4::truncate)
        .collect();
    let vertex_uvs: Vec<_> = reader
        .read_tex_coords(0)
        .unwrap()
        .into_f32()
        .map(glam::Vec2::from)
        .collect();
    let indices = reader.read_indices().unwrap().into_u32().collect();

    let mesh = rend3::types::MeshBuilder::new(vertex_positions.to_vec(), rend3::types::Handedness::Right)
        .with_vertex_normals(vertex_normals)
        .with_vertex_tangents(vertex_tangents)
        .with_vertex_uv0(vertex_uvs)
        .with_indices(indices)
        .with_flip_winding_order()
        .build()
        .unwrap();

    // Add mesh to renderer's world
    let mesh_handle = renderer.add_mesh(mesh);

    // Add basic material with all defaults except a single color.
    let material = primitive.material();
    let metallic_roughness = material.pbr_metallic_roughness();
    let material_handle = renderer.add_material(rend3_routine::pbr::PbrMaterial {
        albedo: rend3_routine::pbr::AlbedoComponent::Value(metallic_roughness.base_color_factor().into()),
        ..Default::default()
    });

    (mesh_handle, material_handle)
}