use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

#[derive(Resource)]
struct Rule30State {
    current_row: Vec<u8>,
    current_y: usize,
}

#[derive(Component)]
struct GridSprite;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Rule30State {
            current_row: initialize_first_row(WIDTH),
            current_y: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_rule_30)
        .run();
}

fn initialize_first_row(width: usize) -> Vec<u8> {
    let mut row = vec![0; width];
    row[width / 2] = 1;
    for x in row.iter_mut() {
        *x = fastrand::u8(0..=1);
    }
    row
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    let extent = Extent3d {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        depth_or_array_layers: 1,
    };

    let mut data = vec![0; WIDTH * HEIGHT * 4];
    for chunk in data.chunks_exact_mut(4) {
        chunk[3] = 255;
    }
    let image = Image::new(
        extent,
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    let image_handle = images.add(image);

    commands.spawn((
        Sprite {
            image: image_handle,
            custom_size: Some(Vec2::new(1024.0, 1024.0)),
            ..default()
        },
        GridSprite,
    ));
}

fn update_rule_30(
    mut state: ResMut<Rule30State>,
    mut images: ResMut<Assets<Image>>,
    sprite_query: Query<&Sprite, With<GridSprite>>,
) {
    if state.current_y >= HEIGHT - 1 {
        return;
    }
    let mut next_row = vec![0; WIDTH];
    let prev = &state.current_row;

    for i in 0..WIDTH {
        let left = if i == 0 { prev[WIDTH - 1] } else { prev[i - 1] };
        let center = prev[i];
        let right = if i == WIDTH - 1 { prev[0] } else { prev[i + 1] };

        // next_row[i] = left ^ (center | right);
        // next_row[i] = left ^ (center ^ right);
        let critical: f64 = 3.1415926f64.powi(-1);
        if fastrand::f64() < critical {
            next_row[i] = left & center & right;
        } else {
            next_row[i] = left ^ center ^ right;
        }
    }

    let next_y = state.current_y + 1;
    if let Ok(sprite) = sprite_query.single()
        && let Some(image) = images.get_mut(&sprite.image)
        && let Some(data) = &mut image.data
    {
        next_row.iter().enumerate().for_each(|(x, val)| {
            let color_val = val * 255;
            let pixel_idx = (next_y * WIDTH + x) * 4;

            data[pixel_idx] = color_val;
            data[pixel_idx + 1] = color_val;
            data[pixel_idx + 2] = color_val;
            data[pixel_idx + 3] = 255;
        });
    }
    state.current_row = next_row;
    state.current_y = next_y;
}
