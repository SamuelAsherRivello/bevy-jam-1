#[derive(Clone, Copy)]
pub(crate) struct AutopilotCommand {
    pub bank_input: f32,
    pub duration_seconds: f32,
}

#[derive(Clone, Copy)]
pub(crate) struct AutopilotPattern {
    commands: [AutopilotCommand; 4],
}

impl AutopilotPattern {
    pub(crate) const fn new(
        first_command: AutopilotCommand,
        second_command: AutopilotCommand,
        third_command: AutopilotCommand,
        fourth_command: AutopilotCommand,
    ) -> Self {
        Self {
            commands: [first_command, second_command, third_command, fourth_command],
        }
    }

    pub(crate) fn bank_input(&self, elapsed_seconds: f32) -> f32 {
        let cycle_seconds: f32 = self
            .commands
            .iter()
            .map(|command| command.duration_seconds.max(0.0))
            .sum();
        if cycle_seconds <= f32::EPSILON {
            return 0.0;
        }

        let mut phase_seconds = elapsed_seconds.rem_euclid(cycle_seconds);
        for command in self.commands {
            let duration_seconds = command.duration_seconds.max(0.0);
            if phase_seconds < duration_seconds {
                return command.bank_input;
            }
            phase_seconds -= duration_seconds;
        }

        0.0
    }
}

pub(crate) const fn autopilot_command(bank_input: f32, duration_seconds: f32) -> AutopilotCommand {
    AutopilotCommand {
        bank_input,
        duration_seconds,
    }
}
