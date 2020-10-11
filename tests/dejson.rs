use nano_ogmo::*;

#[test]
fn project() {
    let json = include_str!("../assets/nano.ogmo").to_string();
    let project = Project::from_json(&json).unwrap();

    assert_eq!(project.name, "NanoOgmoExample");
    assert_eq!(project.layers.len(), 3);
    assert_eq!(project.entities.len(), 1);
    assert_eq!(project.entities[0].name, "player");

}

#[test]
fn level() {
    let json = include_str!("../assets/example.json").to_string();
    let level = Level::from_json(&json).unwrap();

    assert_eq!(level.ogmo_version, "3.3.0");
    assert_eq!(level.width, 704);
    assert_eq!(level.height, 448);
    assert_eq!(level.offset_x, 0);
    assert_eq!(level.offset_y, 0);
    assert_eq!(level.layers.len(), 3);
    assert_eq!(level.layers[0].name, "foreground");
    assert_eq!(level.layers[0].tileset.clone().unwrap(), "New Tileset");
}