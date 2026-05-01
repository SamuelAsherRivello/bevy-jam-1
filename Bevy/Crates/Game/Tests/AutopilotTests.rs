use crate::autopilot_utility::{AutopilotPattern, autopilot_command};

#[test]
fn autopilot_pattern_follows_command_durations() {
    let pattern = AutopilotPattern::new(
        autopilot_command(0.0, 3.0),
        autopilot_command(1.0, 2.0),
        autopilot_command(0.0, 1.0),
        autopilot_command(-1.0, 4.0),
    );

    assert_eq!(pattern.bank_input(0.0), 0.0);
    assert_eq!(pattern.bank_input(3.0), 1.0);
    assert_eq!(pattern.bank_input(5.0), 0.0);
    assert_eq!(pattern.bank_input(6.0), -1.0);
    assert_eq!(pattern.bank_input(10.0), 0.0);
}
