use crate::components::frametime::FrameTimeHandler;
use crate::components::graphics::Graphics;
use crate::components::simulation::Simulation;

pub struct DrawCall {
    pub graphics: Graphics,
    pub frame_timing: FrameTimeHandler,
    pub simulation: Simulation,
}

impl DrawCall {
    pub fn new(graphics: Graphics, simulation: Simulation) -> DrawCall {
        let frame_timing = FrameTimeHandler::new(30);
        DrawCall {
            graphics,
            frame_timing,
            simulation,
        }
    }
}
