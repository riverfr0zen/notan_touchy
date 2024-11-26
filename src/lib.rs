use notan_core::events::Event;
// use notan_log as log;


#[derive(Debug)]
pub enum TouchGesture {
    SwipeUp,
    SwipeDown,
    SwipeLeft,
    SwipeRight,
    Tap,
    LongTap,
    DoubleTap,
}


#[derive(Debug)]
pub struct TouchState {
    pub touch_interface_detected: bool,
    /// The minimum distance on an axis after which the interaction is considered a swipe
    pub swipe_threshold: f32,
    /// The maximum length of a touch for an interaction to be considered a short tap
    pub tap_threshold: f32,
    started_at: f32,
    start_x: f32,
    start_y: f32,
    ended_at: f32,
    end_x: f32,
    end_y: f32,
}

impl Default for TouchState {
    fn default() -> Self {
        Self {
            touch_interface_detected: false,
            swipe_threshold: 100.0,
            tap_threshold: 0.5,
            started_at: 0.0,
            start_x: 0.0,
            start_y: 0.0,
            ended_at: 0.0,
            end_x: 0.0,
            end_y: 0.0,
        }
    }
}

impl TouchState {
    pub fn reset(&self) -> Self {
        Self {
            touch_interface_detected: self.touch_interface_detected,
            swipe_threshold: self.swipe_threshold,
            tap_threshold: self.tap_threshold,
            ..Default::default()
        }
    }

    pub fn get_gesture(&mut self, time: &f32, evt: &Event) -> Option<TouchGesture> {
        let mut gesture: Option<TouchGesture> = None;
        match evt {
            Event::TouchStart { x, y, .. } => {
                // log::debug!("touch start x {} y {} at {}", x, y, time);
                self.started_at = time.clone();
                self.start_x = x.clone();
                self.start_y = y.clone();
            }
            Event::TouchEnd { x, y, .. } => {
                // log::debug!("touch end x {} y {} at {}", x, y, time);
                self.ended_at = time.clone();
                self.end_x = x.clone();
                self.end_y = y.clone();
            }
            _ => {}
        }
        let touch_duration = self.ended_at - self.started_at;
        if touch_duration > 0.0 {
            // log::debug!("touch interface detected");
            self.touch_interface_detected = true;
            // log::debug!("touch duration {}", touch_duration);
            let x_diff = self.end_x - self.start_x;
            let y_diff = self.end_y - self.start_y;
            // log::debug!("xdiff {} ydiff {}", x_diff, y_diff);
            if x_diff.abs() > y_diff.abs() {
                if x_diff > self.swipe_threshold {
                    // log::debug!("swipe right");
                    gesture = Some(TouchGesture::SwipeRight);
                }
                if x_diff < -self.swipe_threshold {
                    // log::debug!("swipe left");
                    gesture = Some(TouchGesture::SwipeLeft);
                }
            }
            if gesture.is_none() && y_diff.abs() > x_diff.abs() {
                if y_diff > self.swipe_threshold {
                    // log::debug!("swipe down");
                    gesture = Some(TouchGesture::SwipeDown);
                }
                if y_diff < -self.swipe_threshold {
                    // log::debug!("swipe up");
                    gesture = Some(TouchGesture::SwipeUp);
                }
            }
            if gesture.is_none() {
                if touch_duration < self.tap_threshold {
                    // log::debug!("tap");
                    gesture = Some(TouchGesture::Tap);
                } else if touch_duration >= self.tap_threshold {
                    // log::debug!("long tap");
                    gesture = Some(TouchGesture::LongTap);
                }
            }
        }
        if gesture.is_some() {
            *self = self.reset();
        }
        gesture
    }
}
