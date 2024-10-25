use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, ShaderType, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowResized,
};

const RES_WIDTH: u32 = 640;
const RES_HEIGHT: u32 = 360;

pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
// pub const LIGHTS_LAYERS: RenderLayers = RenderLayers::layer(1);
// pub const HEIGHT_LAYERS: RenderLayers = RenderLayers::layer(2);
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(3);

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
struct Canvas;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct Cameras;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct InGameCamera;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct LightsCamera;

#[derive(Reflect, Clone, Component)]
#[reflect(Component)]
pub struct HeightCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct OuterCamera;

#[derive(Default, Debug, Clone, Copy, ShaderType, Reflect)]
struct Light {
    position: Vec2,
    aim: Vec2,
    angle: f32,
    range: f32,
    _nothing: Vec2,
}

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
            .add_plugins(Material2dPlugin::<CustomMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, fit_canvas);
    }
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();

    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..Default::default()
    };

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // let mut canvas_lights = canvas.clone();
    // let mut canvas_height = canvas.clone();

    // fill image.data with zeroes
    canvas.resize(canvas_size);
    // canvas_lights.resize(canvas_size);
    // canvas_height.resize(canvas_size);

    let image_handle = images.add(canvas);
    // let image_lights_handle = images.add(canvas_lights);
    // let image_height_handle = images.add(canvas_height);

    commands
        .spawn((
            TransformBundle {
                ..Default::default()
            },
            Cameras,
        ))
        .with_children(|parent| {
            // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
            parent.spawn((
                Camera2dBundle {
                    camera: Camera {
                        // render before the "main pass" camera
                        order: -1,
                        target: RenderTarget::Image(image_handle.clone()),
                        ..default()
                    },
                    ..default()
                },
                InGameCamera,
                PIXEL_PERFECT_LAYERS,
            ));

            // this camera renders whatever is on LIGHTS_LAYERS` to the canvas
            // parent.spawn((
            //     Camera2dBundle {
            //         camera: Camera {
            //             // render before the "height" camera
            //             order: -3,
            //             target: RenderTarget::Image(image_lights_handle.clone()),
            //             ..default()
            //         },
            //         ..default()
            //     },
            //     LightsCamera,
            //     LIGHTS_LAYERS,
            // ));

            // this camera renders whatever is on `HEIGHT_LAYERS` to the canvas
            // parent.spawn((
            //     Camera2dBundle {
            //         camera: Camera {
            //             // render before the "pixel perfect" camera
            //             order: -2,
            //             target: RenderTarget::Image(image_height_handle.clone()),
            //             ..default()
            //         },
            //         ..default()
            //     },
            //     HeightCamera,
            //     HEIGHT_LAYERS,
            // ));
        });

    // commands.spawn((
    //     SpriteBundle {
    //         texture: image_handle,
    //         ..Default::default()
    //     },
    //     Canvas,
    //     HIGH_RES_LAYERS,
    // ));

    // quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default().with_scale(Vec3::new(
                window.physical_width() as f32,
                window.physical_height() as f32,
                0.,
            )),
            material: materials.add(CustomMaterial {
                color_texture: image_handle,
                // lights_texture: image_lights_handle,
                // height_texture: image_height_handle,
            }),
            ..default()
        },
        Canvas,
        HIGH_RES_LAYERS,
    ));

    // commands.spawn((
    //     SpriteBundle {
    //         texture: image_handle,
    //         ..default()
    //     },
    //     Canvas,
    //     HIGH_RES_LAYERS,
    // ));

    // the "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2dBundle::default(), OuterCamera, HIGH_RES_LAYERS));
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    color_texture: Handle<Image>,
    // #[texture(2)]
    // lights_texture: Handle<Image>,
    // #[texture(3)]
    // height_texture: Handle<Image>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lighting.wgsl".into()
    }
}
