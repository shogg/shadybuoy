use bevy::{
    asset::AssetServerSettings,
	ecs::system::{lifetimeless::SRes, SystemParamItem},
	prelude::*,
	reflect::TypeUuid,
	render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            *,
        },
        renderer::{RenderDevice, RenderQueue},
        RenderStage, RenderApp,
    },
	sprite::*, asset::Assets,
};

fn main() {
    let mut app = App::new();

    app.insert_resource(AssetServerSettings {
            watch_for_changes: true,
            asset_folder: "assets".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<ShaderMaterial>::default())
        .add_system(log_asset_events)
        .add_startup_system(setup);

    app.sub_app_mut(RenderApp)
    .add_system_to_stage(RenderStage::Extract, extract_material)
    .add_system_to_stage(RenderStage::Prepare, prepare_material);

    app.run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ShaderMaterial>>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // quad
    commands.spawn().insert_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(256.)),
        material: materials.add(ShaderMaterial {
            color: Vec4::from_slice(&Color::GOLD.as_linear_rgba_f32()),
            frame: 0,
        }),
        ..Default::default()
    });
}

pub fn log_asset_events(
    mut ev_reader: EventReader<AssetEvent<Shader>>,
) {
    for ev in ev_reader.iter() {
        println!("{:?}", ev);
    }
}

pub fn extract_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<ShaderMaterial>>,
) {
    for m in materials.iter_mut() {
        m.1.frame += 1;
        commands.insert_resource(m.1.clone());
    }
}

pub fn prepare_material(
    mut material: ResMut<ShaderMaterial>,
    mut material_gpu_assets: ResMut<RenderAssets<ShaderMaterial>>,
    render_queue: Res<RenderQueue>,
) {
    for material_gpu in material_gpu_assets.values_mut() {
        material.color = Vec4::from_slice(&Color::GOLD.as_linear_rgba_f32());
        material.frame += 1;
        render_queue.write_buffer(&material_gpu.buffer, 0, material.as_std140().as_bytes());
    }
    println!("Frame {:?}", material.frame);
}

#[derive(Debug, Clone, TypeUuid, AsStd140)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct ShaderMaterial {
	pub color: Vec4,
    pub frame: u32,
}
pub struct ShaderMaterialGpu {
    pub buffer: Buffer,
    pub bind_group: BindGroup,
}

impl RenderAsset for ShaderMaterial {

    type ExtractedAsset = ShaderMaterial;
    type PreparedAsset = ShaderMaterialGpu;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: extracted_asset.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
            ],
            label: None,
            layout: &material_pipeline.material2d_layout,
        });

        Ok(ShaderMaterialGpu {
            buffer,
            bind_group,
        })
    }
}

impl Material2d for ShaderMaterial {

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/custom_material.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64 + 16),
                    },
                    count: None,
                },
            ],
            label: None,
        })
    }
}