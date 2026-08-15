use qw_core::DialogueConfig;

/// Speech-band / dialogue detector. Reports unknown until the classifier exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialoguePresence {
    Unknown,
    Speech,
    NonSpeech,
}

#[derive(Clone, Debug)]
pub struct DialogueLogic {
    config: DialogueConfig,
    presence: DialoguePresence,
}

impl DialogueLogic {
    pub fn new(config: DialogueConfig) -> Self {
        Self {
            config,
            presence: DialoguePresence::Unknown,
        }
    }

    pub fn presence(&self) -> DialoguePresence {
        self.presence
    }

    pub fn process(&mut self, samples: &mut [f32]) -> DialoguePresence {
        let _ = samples;
        if !self.config.enabled {
            self.presence = DialoguePresence::Unknown;
            return self.presence;
        }
        self.presence
    }

    pub fn config(&self) -> &DialogueConfig {
        &self.config
    }
}
