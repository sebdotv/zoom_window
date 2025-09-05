use egui::{Color32, Response, Ui};
use walkers::Projector;
use walkers::{MapMemory, Position};

#[derive(Copy, Clone)]
enum State {
    Idle,
    StartPoint {
        start: Position,
        hover: Option<Position>,
    },
    Line {
        start: Position,
        end: Position,
    },
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
        // handle middle click to set points
        if !response.changed() && response.clicked_by(egui::PointerButton::Middle) {
            let clicked_at = response
                .interact_pointer_pos()
                .map(|p| projector.unproject(p.to_vec2()));
            match (self.state, clicked_at) {
                (State::Idle, Some(pos)) => {
                    self.state = State::StartPoint {
                        start: pos,
                        hover: None,
                    };
                }
                (State::StartPoint { start, .. }, Some(end)) => {
                    self.state = State::Line { start, end };
                }
                (State::Line { .. }, Some(_)) => {
                    self.state = State::Idle;
                }
                _ => {}
            }
        }

        // update hover position
        if let State::StartPoint { hover, .. } = &mut self.state
            && let Some(hover_pos) = response.hover_pos()
        {
            *hover = Some(projector.unproject(hover_pos.to_vec2()));
        }

        match self.state {
            State::Idle => {}
            State::StartPoint { start, hover } => {
                // draw line from start to hover
                if let Some(hover) = hover {
                    let start_pos = projector.project(start).to_pos2();
                    let end_pos = projector.project(hover).to_pos2();
                    let line = egui::Shape::line_segment(
                        [start_pos, end_pos],
                        egui::Stroke::new(1.0, Color32::RED),
                    );
                    ui.painter().add(line);
                }

                // draw start point
                ui.painter()
                    .circle_filled(projector.project(start).to_pos2(), 5.0, Color32::GREEN);
            }
            State::Line { start, end } => {
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
