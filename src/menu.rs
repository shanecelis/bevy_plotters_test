use bevy::prelude::*;
use bevy::color::Color;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use bevy::render::render_asset::RenderAssetUsages;
use plotters::prelude::*;
use plotters::backend::BGRXPixel;
use plotters::style::Color as PlottersColor;


pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, menu_setup);
}

// const WIDTH: u32 = 1000;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn menu_setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut bytes: Vec<u8> = vec![0; (WIDTH * HEIGHT * 4) as usize];

    {
        let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(&mut bytes, (WIDTH, HEIGHT)).unwrap().into_drawing_area();
        root.fill(&WHITE).unwrap();
        let white: RGBAColor = RGBAColor(255, 0, 0, 1.0);
        // white.alpha = 1.0;
        // root.draw_pixel((0,0), &WHITE.mix(0.5)).unwrap();
        root.draw_pixel((0,0), &white).unwrap();
        root.draw_pixel((WIDTH as i32,HEIGHT as i32), &white).unwrap();
        root.draw_pixel((WIDTH  as i32/ 2 ,HEIGHT  as i32/ 2), &white).unwrap();
        // root.fill(&WHITE).unwrap();
        // let style = ShapeStyle {
        //     color: WHITE.into(),
        //     filled: false,
        //     stroke_width: 2,
        // };
        let mut chart = ChartBuilder::on(&root)
        //     .caption("y=x^2", ("sans-serif", 8).into_font())
        //     .margin(5)
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



    let image = images.add(Image::new(
        Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Bgra8UnormSrgb,
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
            parent.spawn(ImageBundle {
                image: UiImage::new(image.clone()),
                style: Style {
                    width: Val::Px(WIDTH as f32),
                    height: Val::Px(HEIGHT as f32),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                // background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            });
        });
}
