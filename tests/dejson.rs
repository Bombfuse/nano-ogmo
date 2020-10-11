use nano_ogmo::*;

#[test]
fn test() {
    let json = include_str!("../assets/nano.ogmo").to_string();
    let project = Project::from_json(&json).unwrap();

    assert_eq!(project.name, "NanoOgmoExample");
    assert_eq!(project.layers.len(), 3);
    assert_eq!(project.entities.len(), 1);
    assert_eq!(project.entities[0].name, "player");

}