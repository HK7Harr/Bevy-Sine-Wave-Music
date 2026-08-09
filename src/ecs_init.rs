use crate::internal_imports::*;


#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UICamera;


#[derive(Component)]
pub struct SineWave {
    // math terms
    pub amplitude: f64, // maximum value of the sine function
    pub frequency: f64, // length from a to b where a and b are two top's next to each other
    pub phase: f64, // where the wave starts

    // music terms
    pub attack: f64, // how many cycles it takes to go to maximum
    pub sustain: f64, // how many cycles it stays at peak amplitude
    pub release: f64, // how many cycles it takes to drop to minimum

    // location
    pub x_min: f64,
    pub x_max: f64,
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