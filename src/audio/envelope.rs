//! ADSR envelope generator

/// Envelope stages
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

/// ADSR envelope generator
pub struct Envelope {
    pub attack: f32,  // seconds
    pub decay: f32,   // seconds
    pub sustain: f32, // level (0-1)
    pub release: f32, // seconds

    stage: EnvelopeStage,
    level: f32,
    time_in_stage: f32,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            stage: EnvelopeStage::Idle,
            level: 0.0,
            time_in_stage: 0.0,
        }
    }

    /// Trigger the envelope
    pub fn trigger(&mut self) {
        self.stage = EnvelopeStage::Attack;
        self.time_in_stage = 0.0;
    }

    /// Release the envelope
    pub fn release(&mut self) {
        if self.stage != EnvelopeStage::Idle {
            self.stage = EnvelopeStage::Release;
            self.time_in_stage = 0.0;
        }
    }

    /// Process and return current level
    pub fn process(&mut self, delta_time: f32) -> f32 {
        self.time_in_stage += delta_time;

        match self.stage {
            EnvelopeStage::Idle => {
                self.level = 0.0;
            }
            EnvelopeStage::Attack => {
                if self.attack > 0.0 {
                    self.level = (self.time_in_stage / self.attack).min(1.0);
                } else {
                    self.level = 1.0;
                }
                if self.time_in_stage >= self.attack {
                    self.stage = EnvelopeStage::Decay;
                    self.time_in_stage = 0.0;
                }
            }
            EnvelopeStage::Decay => {
                if self.decay > 0.0 {
                    let t = (self.time_in_stage / self.decay).min(1.0);
                    self.level = 1.0 + (self.sustain - 1.0) * t;
                } else {
                    self.level = self.sustain;
                }
                if self.time_in_stage >= self.decay {
                    self.stage = EnvelopeStage::Sustain;
                }
            }
            EnvelopeStage::Sustain => {
                self.level = self.sustain;
            }
            EnvelopeStage::Release => {
                if self.release > 0.0 {
                    let t = (self.time_in_stage / self.release).min(1.0);
                    self.level = self.sustain * (1.0 - t);
                } else {
                    self.level = 0.0;
                }
                if self.time_in_stage >= self.release {
                    self.stage = EnvelopeStage::Idle;
                    self.level = 0.0;
                }
            }
        }

        self.level
    }

    pub fn is_active(&self) -> bool {
        self.stage != EnvelopeStage::Idle
    }
}
