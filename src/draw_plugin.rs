use egui::{Color32, Response, Stroke, Ui};
use geo::{Distance, Haversine};
use uom::si::f64::Length;
use uom::si::length::{meter, nautical_mile};
use walkers::{MapMemory, Position};
use walkers::{Projector, lat_lon};

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
                if let Some(hover) = hover {
                    draw_segment(ui, start, hover, projector, (1.0, Color32::RED));
                }
                ui.painter()
                    .circle_filled(projector.project(start).to_pos2(), 5.0, Color32::GREEN);
            }
            State::Line { start, end } => {
                draw_segment(ui, start, end, projector, (3.0, Color32::RED));
                ui.painter()
                    .circle_filled(projector.project(start).to_pos2(), 5.0, Color32::GREEN);
                ui.painter()
                    .circle_filled(projector.project(end).to_pos2(), 5.0, Color32::BLUE);
            }
        }
    }
}

fn draw_segment(
    ui: &mut Ui,
    start: Position,
    end: Position,
    projector: &Projector,
    stroke: impl Into<Stroke>,
) {
    // draw line from start to end
    let start_pos = projector.project(start).to_pos2();
    let end_pos = projector.project(end).to_pos2();
    ui.painter().line_segment([start_pos, end_pos], stroke);

    // draw distance text
    let mid_point = lat_lon((start.y() + end.y()) / 2.0, (start.x() + end.x()) / 2.0);
    let mid_pos = projector.project(mid_point).to_pos2();
    let distance = Length::new::<meter>(Haversine.distance(start, end));
    let text = format!("{:.1} NM", distance.get::<nautical_mile>());
    ui.painter().text(
        mid_pos,
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::default(),
        Color32::BLACK,
    );
}
