pub mod calculate {
    use std::time::{Duration,SystemTime,UNIX_EPOCH};
    pub fn get_current_time() -> i32{
        let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as i32;
        time
    }
    pub struct Fps{
        i: i32,
        time_now: i32,
        time_before: i32,
        fps: f32,
    }

    impl Fps {
        pub fn new(time_now :i32) -> Fps {
            Fps {
                i : 0,
                time_now : 0,
                time_before : get_current_time(),
                fps: 0 as f32,
            }
        }
        fn fps_calculate(&mut self) -> f32 {
            self.time_now = get_current_time();
            self.i += 1;
            let different = self.time_now - self.time_before;
            if different >= 1 {
                self.time_before = get_current_time();
                self.fps = self.i as f32;
                self.i = 0;
            }

            self.fps
        }

        pub fn show_fps(&mut self) {
            println!("{}",self.fps_calculate())
        }

    }
}