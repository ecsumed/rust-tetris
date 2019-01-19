use stdweb::Value;

#[derive(Debug)]
pub struct GameLoop(Value);

impl GameLoop {
    pub fn new<F>(callback: F) -> Self where F: FnMut(f64) + 'static {
        GameLoop(js!(
            var callback = @{callback};

            function loop(time) {
                state.id = requestAnimationFrame(loop);
                callback(time);
            }

            var state = {
                callback: callback,
                id: requestAnimationFrame(loop)
            };

            return state;
        ))
    }
}

impl Drop for GameLoop {
    fn drop(&mut self) {
        js! { @(no_return)
            var state = @{&self.0};
            cancelAnimationFrame(state.id);
            state.callback.drop();
        }
    }
}
