use crate::internal_imports::*;


#[derive(Component)]
pub struct MainCamera;


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