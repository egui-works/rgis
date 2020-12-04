use bevy::{prelude::*, render::pass::ClearColor};

mod plugins;

// TODO: allow these to be controller at command line
static SOURCE_PROJECTION: &str = "EPSG:4326";
static TARGET_PROJECTION: &str = "EPSG:3857";

// System
fn load_layers_from_cli(mut events: ResMut<Events<LoadGeoJsonFile>>) {
    for geojson_file_path in rgis_cli::run().unwrap() {
        for _ in 0..2 {
            log::debug!(
                "sending LoadGeoJsonFile event: {}",
                geojson_file_path.clone()
            );
            events.send(LoadGeoJsonFile {
                path: geojson_file_path.clone(),
            });
        }
    }
}

// System
fn load_geojson_file_handler(
    mut layers: ResMut<rgis_layers::Layers>,
    load_events: Res<Events<LoadGeoJsonFile>>,
    mut load_event_reader: Local<EventReader<LoadGeoJsonFile>>,
    mut loaded_events: ResMut<Events<LayerLoaded>>,
) {
    for LoadGeoJsonFile {
        path: geojson_file_path,
    } in load_event_reader.iter(&load_events)
    {
        println!("loading {}", geojson_file_path);
        let count = rgis_file_loader::load(
            geojson_file_path.clone(),
            &mut layers,
            SOURCE_PROJECTION,
            TARGET_PROJECTION,
        );
        if count > 0 {
            loaded_events.send(LayerLoaded);
        }
    }
}

// System
fn layer_loaded(
    events: Res<Events<LayerLoaded>>,
    mut event_reader: Local<EventReader<LayerLoaded>>,
) {
    for event in event_reader.iter(&events) {
        println!("event received: {:?}", event);
    }
}

#[derive(Debug)]
struct LoadGeoJsonFile {
    path: String,
}

#[derive(Debug)]
struct LayerLoaded;

#[derive(Debug)]
struct Camera(Entity);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = commands
        .spawn(Camera2dComponents::default())
        .current_entity();

    commands.spawn((Camera(entity.expect("could not find entity")),));

    let texture_handle = asset_server.load("/Users/coreyf/Downloads/meow.png");
    commands.spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        });
}

fn main() {
    env_logger::init();

    App::build()
        .add_event::<LoadGeoJsonFile>()
        .add_event::<LayerLoaded>()
        .add_plugins(DefaultPlugins)
        .add_resource(rgis_layers::Layers::new())
        .add_startup_system(load_layers_from_cli.system())
        .add_startup_system(setup.system())
        .add_system(load_geojson_file_handler.system())
        .add_system(layer_loaded.system())
        .add_plugin(plugins::KeyboardCameraMover)
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .run();
}