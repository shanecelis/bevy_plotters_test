use bevy::prelude::*;
use bevy::color::Color;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use bevy::render::render_asset::RenderAssetUsages;
use plotters::prelude::*;
use plotters::backend::BGRXPixel;
use plotters::style::Color as PlottersColor;
use bevy::{prelude::*, reflect::TypePath, render::render_resource::*};
use bevy::input::common_conditions::{input_just_pressed, input_just_released};

pub(crate) fn plugin(app: &mut App) {
    app
        .add_systems(Startup, menu_setup)
        .add_systems(Update,
                     update_image.run_if(input_just_pressed(KeyCode::Space)))
        .add_plugins(UiMaterialPlugin::<NoAlphaUiMaterial>::default());
}

// const WIDTH: u32 = 1000;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn menu_setup(mut commands: Commands,
              mut images: ResMut<Assets<Image>>,
              mut ui_materials: ResMut<Assets<NoAlphaUiMaterial>>,
) {
    // let mut pixels: Vec<u32> = vec![0x0000ffff; (WIDTH * HEIGHT) as usize];
    // let (prefix, bytes, suffix) = pixels.align_to_mut::<u8>();
    // assert_eq!(prefix.len(), 0);
    // assert_eq!(suffix.len(), 0);
    let mut bytes: Vec<u8> = vec![0x0; (WIDTH * HEIGHT * 4) as usize];
    // for i in 0..bytes.len() / 4 {

    //     bytes[i * 4 + 0] = 0xff;
    //     bytes[i * 4 + 3] = 0xff;
    // }
    // unsafe {
    //     let (prefix, pixels, suffix) = bytes.align_to_mut::<u32>();
    //     assert_eq!(prefix.len(), 0);
    //     assert_eq!(suffix.len(), 0);
    //     for pixel in pixels {
    //         *pixel = 0xff000000; // layout is 0xaarrggbb.
    //     }
    // }

    // if false
    {
        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(&mut bytes, (WIDTH, HEIGHT)).unwrap().into_drawing_area();
        // fill with black removes the alpha channel.
        root.fill(&BLACK).unwrap();
        // root.fill(&GREEN).unwrap();
        // root.fill(&RED).unwrap();

        // root.fill(&GREEN).unwrap();
        // root.fill(&TRANSPARENT).unwrap();
        // root.draw_rect((0,0), (10, 10), &TRANSPARENT, true).unwrap();
        // root.draw(&Rectangle::new([(0.0,0.0), (10.0, 10.0)],
        //                           BLACK.filled())).unwrap();
        // let white: RGBAColor = RGBAColor(255, 0, 0, 1.0);
        // white.alpha = 1.0;
        // root.draw_pixel((0,0), &WHITE.mix(0.5)).unwrap();
        // root.draw_pixel((0,0), &white).unwrap();
        // root.draw_pixel((WIDTH as i32,HEIGHT as i32), &white).unwrap();
        // root.draw_pixel((WIDTH  as i32/ 2 ,HEIGHT  as i32/ 2), &white).unwrap();
        // root.fill(&WHITE).unwrap();
        // let style = ShapeStyle {
        //     color: WHITE.into(),
        //     filled: false,
        //     stroke_width: 2,
        // };
        let mut chart = ChartBuilder::on(&root)
        //     .caption("y=x^2", ("sans-serif", 8).into_font())
            .margin(5)
        //     .x_label_area_size(5)
        //     .y_label_area_size(5)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .unwrap();
        // // chart.configure_mesh()
        // //     .light_line_style(style)
        // //     .draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))
            .unwrap()
        //     .label("y = x^2")
        //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        ;

        // chart
        //     .configure_series_labels()
        //     .background_style(&BLACK)
        //     .border_style(&WHITE)
        //     .draw()
        //     .unwrap();

        root.present().unwrap();
    }
    // assert_eq!(bytes[2], 0xff);
    // assert_eq!(bytes[3], 0x00);

    // set_alpha(&mut bytes, 0xff);
    // Set alpha to 0xff.
    // for i in 0..bytes.len() / 4 {
    //     // bytes[i * 4 + 0] = 0xff;
    //     bytes[i * 4 + 3] = 0xff;
    // }


    let image = images.add(Image::new(
        Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Bgra8UnormSrgb,
        // TextureFormat::Argb8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD |
        RenderAssetUsages::RENDER_WORLD
    ));
    // commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {

                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {

            parent.spawn(MaterialNodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,

                    width: Val::Px(WIDTH as f32),
                    height: Val::Px(HEIGHT as f32),
                    border: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                material: ui_materials.add(NoAlphaUiMaterial {
                    color: LinearRgba::WHITE,
                    texture: image,
                }),
                ..default()
            });
            // parent.spawn(ImageBundle {
            //     image: UiImage::new(image.clone()),
            //     style: Style {
            //         width: Val::Px(WIDTH as f32),
            //         height: Val::Px(HEIGHT as f32),
            //         border: UiRect::all(Val::Px(2.0)),
            //         ..default()
            //     },
            //     // background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            //     ..default()
            // });
        });
}

fn set_alpha(bytes: &mut [u8], alpha: u8) {
    // Set alpha to 0xff.
    for i in 0..bytes.len() / 4 {
        bytes[i * 4 + 3] = alpha;
    }
}

fn update_image(query: Query<&Handle<NoAlphaUiMaterial>>,
                mut ui_materials: ResMut<Assets<NoAlphaUiMaterial>>,
                mut images: ResMut<Assets<Image>>) {
    let handle = query.single();
    // Must get mut handle of material, otherwise it won't shows changes to
    // image.
    if let Some(material) = ui_materials.get_mut(handle) {
    if let Some(image) = images.get_mut(&material.texture)
    {
        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(&mut image.data, (WIDTH, HEIGHT)).unwrap().into_drawing_area();
        // fill with black removes the alpha channel.
        root.fill(&GREEN).unwrap();
        // root.fill(&GREEN).unwrap();
        // root.fill(&RED).unwrap();

        // root.fill(&GREEN).unwrap();
        // root.fill(&TRANSPARENT).unwrap();
        // root.draw_rect((0,0), (10, 10), &TRANSPARENT, true).unwrap();
        // root.draw(&Rectangle::new([(0.0,0.0), (10.0, 10.0)],
        //                           BLACK.filled())).unwrap();
        // let white: RGBAColor = RGBAColor(255, 0, 0, 1.0);
        // white.alpha = 1.0;
        // root.draw_pixel((0,0), &WHITE.mix(0.5)).unwrap();
        // root.draw_pixel((0,0), &white).unwrap();
        // root.draw_pixel((WIDTH as i32,HEIGHT as i32), &white).unwrap();
        // root.draw_pixel((WIDTH  as i32/ 2 ,HEIGHT  as i32/ 2), &white).unwrap();
        // root.fill(&WHITE).unwrap();
        // let style = ShapeStyle {
        //     color: WHITE.into(),
        //     filled: false,
        //     stroke_width: 2,
        // };
        let mut chart = ChartBuilder::on(&root)
        //     .caption("y=x^2", ("sans-serif", 8).into_font())
            .margin(5)
        //     .x_label_area_size(5)
        //     .y_label_area_size(5)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .unwrap();
        // // chart.configure_mesh()
        // //     .light_line_style(style)
        // //     .draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x * x)),
                &RED,
            ))
            .unwrap()
        //     .label("y = x^2")
        //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        ;

        // chart
        //     .configure_series_labels()
        //     .background_style(&BLACK)
        //     .border_style(&WHITE)
        //     .draw()
        //     .unwrap();

        root.present().unwrap();
    }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct NoAlphaUiMaterial {
    /// Color multiplied with the image
    #[uniform(0)]
    color: LinearRgba,
    /// Image used to represent graph
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
}

impl UiMaterial for NoAlphaUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "no_alpha_material.wgsl".into()
    }
}
