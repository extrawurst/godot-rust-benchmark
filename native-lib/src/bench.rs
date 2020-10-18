use gdnative::{api::OS, prelude::*};

#[derive(NativeClass)]
#[inherit(CanvasItem)]
pub struct MyBench {
    #[property]
    rad: f32,
    #[property]
    cnt: i32,
    #[property]
    start: Vector2,
}

#[methods]
impl MyBench {
    fn new(_owner: &CanvasItem) -> Self {
        Self {
            rad: 200.0,
            cnt: 4000,
            start: Vector2::new(200.0, 200.0),
        }
    }

    #[export]
    fn _draw(&self, owner: &CanvasItem) {
        let start_time = OS::godot_singleton().get_ticks_usec();

        let cntf = self.cnt as f32;

        for n in 0..self.cnt {
            let x = f32::sin(n as f32 / cntf * 360.0) * self.rad;
            let y = f32::cos(n as f32 / cntf * 360.0) * self.rad;
            let target = Vector2::new(x, y) + self.start;

            owner.draw_line(self.start, target, Color::rgb(0.0, 0.0, 1.0), 1.0, false)
        }

        godot_print!(
            "bench: {}",
            OS::godot_singleton().get_ticks_usec() - start_time
        );
    }

    #[export]
    fn _process(&self, owner: &CanvasItem, _delta: f64) {
        owner.update();
    }
}
