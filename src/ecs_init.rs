use crate::internal_imports::*;


#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UICamera;


#[derive(Component)]
pub struct SineWave {
    // Math 
    pub frequency: f64,     // Hz
    pub period: f64,        // peak to peak
    pub amplitude: f64,     // Peak volume 0-1
    pub phase_offset: f64,  // Initial phase shift in radians (0.0 to 2π)
    pub detune_cents: f64,  // Fine tuning in cents -100-100

    // envelope adsr in seconds
    pub attack: f64,        // Time to reach peak amplitude
    pub decay: f64,         // Time to fall from peak to sustain level
    pub sustain_level: f64, // Amplitude multiplier during sustain 0-1
    pub release: f64,       // Time to fade to 0 after key release 

    // Audio Output & State
    pub pan: f32,           // Stereo pan: -1.0 (Left) to 1.0 (Right)
    pub current_phase: f64, // Tracks current wave position to prevent audio clicks
    pub is_active: bool,     // Whether the wave is currently generating sound

    // x bounds
    pub x_start: f64,
    pub x_stop: f64,
}

#[derive(Resource)]
pub struct AudioSettings {
    pub time_multiplier: f32,
    pub is_playing: (bool, usize), //if it is playing and what is playing, 0 is collective, the indecies up from that is the track playing 
    pub repeat: bool,
}

#[derive(Default, Resource)]
pub struct CompositionWaves {
    pub waves: Vec<SineWave>,
}

#[derive(Resource)]
pub struct PanelInfo {
    pub width: f32, // in pixels
    pub ui_scalar: f32, // default 1.0
}
impl PanelInfo {
    pub fn update_ui_scalar(&mut self) {
        self.ui_scalar = self.width / DEFAULT_PANEL_WIDTH;
    }
}
impl Default for PanelInfo {
    fn default() -> Self {
        Self {
            width: DEFAULT_PANEL_WIDTH,
            ui_scalar: 1.0,
        }
    }
}


// FRAME SCHEDULER
struct Schedule {
    label: &'static str,
    frame_cycle: u32,
    on_frame: u32
}
impl Schedule {
    pub fn add_frame(&mut self) {
        if self.on_frame == self.frame_cycle {
            self.on_frame = 0;
        }
        else {
            self.on_frame += 1;
        }
    }
}

#[derive(Default, Resource)]
pub struct Scheduler {
    schedules: Vec<Schedule>,
}

impl Scheduler {
    pub fn add(&mut self, label: &'static str, frame_cycle: u32) {
        self.schedules.push(Schedule {
            label: label,
            frame_cycle: frame_cycle,
            on_frame: 0
        })
    }
    pub fn add_frame(&mut self) {
        for i in &mut self.schedules {
            i.add_frame();
        }
    }
    pub fn check(&self, label: &'static str) -> bool {
        for i in &self.schedules {
            if i.label == label {
                if i.on_frame == i.frame_cycle {
                    return true;
                }
                return false;
            }
        }
        panic!("You inputted the wrong label: Scheduler::check");
    }
}