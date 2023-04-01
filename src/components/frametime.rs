use std::collections::VecDeque;
use std::time::Duration;
use std::time::Instant;

pub struct FrameTimeHandler {
    frame_times: VecDeque<Duration>,
    frame_start: Instant,
    frame_count: usize,
}

impl FrameTimeHandler {
    pub fn new(frame_count: usize) -> FrameTimeHandler {
        FrameTimeHandler {
            frame_times: VecDeque::with_capacity(frame_count),
            frame_start: Instant::now(),
            frame_count,
        }
    }

    pub fn tick(&mut self) {
        let frame_start = self.frame_start;
        let frame_time = Instant::now().duration_since(frame_start);
        self.frame_start = Instant::now();

        self.frame_times.push_back(frame_time);
        if self.frame_times.len() > self.frame_count {
            self.frame_times.pop_front();
        }
    }

    pub fn get_average_frame_time(&self) -> Duration {
        if self.frame_times.is_empty() {
            return Duration::new(0, 0);
        } else {
            let total_time = self.frame_times.iter().sum::<Duration>();
            total_time / self.frame_times.len() as u32
        }
    }

    pub fn get_frames_per_second(&self) -> f64 {
        let average_frame_time = self.get_average_frame_time().as_secs_f64();
        if average_frame_time == 0.0 {
            return average_frame_time;
        }
        1.0 / average_frame_time
    }
}
