use iced::{Length, mouse, Point, Size};
use iced::widget::{canvas};
use iced::{Color, Rectangle, Renderer};
use iced::mouse::Cursor;

use iced::widget::canvas::{Cache, Frame, Geometry, Path, Program, Stroke};
use iced::widget::canvas::path::lyon_path::geom::size;
use crate::gui::app::Message;
use crate::gui::theme::Theme;
use crate::gui::theme::widget::Canvas;
use iced::widget::canvas::event::{Event,self};
// First, we define the data we need for drawing
#[derive(Debug)]
pub struct SquareCanva {
    pub radius: f32,
    cache: canvas::Cache,
}


// Then, we implement the `Program` trait
impl canvas::Program<Message,Theme> for SquareCanva{
    type State = Option<Pending>;

    fn draw(&self, state: &Self::State, renderer: &Renderer, theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<canvas::Geometry> {
        let content = self.cache.draw(
            renderer,
            bounds.size(),
            |frame: &mut Frame| {
                let height = frame.height();
                let width = frame.width();
                let background = Path::rectangle(
                    Point { x: 40.0, y: 40.0 }, // Coordinate dell'angolo superiore sinistro
                    Size::new(width - 80.0, height - 80.0), // Dimensione del rettangolo
                );
                frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));
            },
        );
        vec![content]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (event::Status::Ignored, Some(Message::None));
        };
        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match *state {
                            None => {
                                *state = Some(Pending::One {
                                    from: cursor_position,
                                });
                                Some(Message::PendingOne(Pending::One {
                                    from: cursor_position,
                                }))

                            }
                            Some(Pending::One { from }) => {
                                println!("Mouse event");
                                *state = Some(Pending::Two {
                                    from,
                                    to: cursor_position,
                                });
                                Some(Message::PendingTwo(Pending::Two {
                                    from,
                                    to: cursor_position,
                                }))
                            }
                            _ => {None}
                        }
                    }
                    _ => Some(Message::None),
                };

                (event::Status::Captured, Some(Message::None))
            }
            _ => (event::Status::Ignored, Some(Message::None)),
        }
    }
}


impl SquareCanva {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            cache: Cache::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Curve {
    from: Point,
    to: Point,
}

impl Curve {
    fn draw_all(curves: &[Curve], frame: &mut Frame) {
        let curves = Path::new(|p| {
            for curve in curves {
                p.move_to(curve.from);
            }
        });

        frame.stroke(&curves, Stroke::default().with_width(2.0));
    }
}




#[derive(Debug, Clone, Copy)]
pub enum Pending {
    One { from: Point },
    Two { from: Point, to: Point },
}

impl Pending {
    fn draw(
        &self,
        renderer: &Renderer,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Geometry {
        let mut frame = Frame::new(renderer, bounds.size());

        if let Some(cursor_position) = cursor.position_in(bounds) {
            match *self {
                Pending::One { from } => {
                    let line = Path::line(from, cursor_position);
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                }
                Pending::Two { from, to } => {
                    let curve = Curve {
                        from,
                        to,
                    };

                    Curve::draw_all(&[curve], &mut frame);
                }
            };
        }

        frame.into_geometry()
    }
}