use egui::{Color32, Response, Ui};
use walkers::Projector;
use walkers::{MapMemory, Position};

#[derive(Copy, Clone)]
enum State {
    Idle,
    StartPoint(Position),
    Line(Position, Position),
}

pub struct DrawPlugin {
    state: State,
}
impl Default for DrawPlugin {
    fn default() -> Self {
        Self { state: State::Idle }
    }
}
impl walkers::Plugin for &mut DrawPlugin {
    fn run(
        self: Box<Self>,
        ui: &mut Ui,
        response: &Response,
        projector: &Projector,
        _map_memory: &MapMemory,
    ) {
        if !response.changed() && response.clicked_by(egui::PointerButton::Middle) {
            let clicked_at = response
                .interact_pointer_pos()
                .map(|p| projector.unproject(p.to_vec2()));
            match (self.state, clicked_at) {
                (State::Idle, Some(pos)) => {
                    self.state = State::StartPoint(pos);
                }
                (State::StartPoint(start), Some(end)) => {
                    self.state = State::Line(start, end);
                }
                (State::Line(_, _), Some(_)) => {
                    self.state = State::Idle;
                }
                _ => {}
            }
        }

        match self.state {
            State::Idle => {}
            State::StartPoint(start) => {
                ui.painter()
                    .circle_filled(projector.project(start).to_pos2(), 5.0, Color32::GREEN);
            }
            State::Line(start, end) => {
                let start_pos = projector.project(start).to_pos2();
                let end_pos = projector.project(end).to_pos2();
                ui.painter()
                    .line_segment([start_pos, end_pos], (3.0, Color32::RED));
                ui.painter().circle_filled(start_pos, 5.0, Color32::GREEN);
                ui.painter().circle_filled(end_pos, 5.0, Color32::BLUE);
            }
        }
    }
}
