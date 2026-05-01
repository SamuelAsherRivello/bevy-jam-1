use crate::propeller_system::propeller_is_animated_name;

#[test]
fn propeller_name_detection_matches_active_and_named_plane_nodes() {
    assert!(propeller_is_animated_name("Cylinder.001"));
    assert!(propeller_is_animated_name("Cube.010_Propeller_0"));
    assert!(propeller_is_animated_name("Propeller"));
    assert!(!propeller_is_animated_name("Player Model"));
}
