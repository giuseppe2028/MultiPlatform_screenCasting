use futures::pending;
use iced::{Length, mouse, Pixels, Point, Size};
use iced::widget::{canvas};
use iced::{Color, Rectangle, Renderer};
use iced::mouse::Cursor;

use iced::widget::canvas::{Cache, Frame, Geometry, Path, Program, Stroke, Text};
use iced::widget::canvas::path::lyon_path::geom::size;
use crate::gui::app::Message;
use crate::gui::theme::Theme;
use crate::gui::theme::widget::Canvas;
use iced::widget::canvas::event::{Event,self};
// First, we define the data we need for drawing

#[derive(Debug)]
pub struct CanvasWidget {
    pub shapes: Vec<Shape>,
    pub start_point:Point,
    pub end_point:Point,
    pub cache: canvas::Cache,
    pub selected_shape: Option<Shape>
}

#[derive(Debug, Clone)]
pub enum Shape {
    Rectangle(RectangleCanva),
    Circle(CircleCanva),
    Arrow(ArrowCanva),
}

#[derive(Debug, Clone)]
pub struct RectangleCanva {
    pub(crate) startPoint:Point,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Debug, Clone)]
pub struct CircleCanva {
    pub(crate) center: Point,
    pub(crate) radius: f32,
}

#[derive(Debug, Clone)]
pub struct ArrowCanva {
    starting_point: Point,
    ending_point: Point,
}




// Then, we implement the `Program` trait
impl canvas::Program<Message,Theme> for CanvasWidget{
    type State = Option<Pending>;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<canvas::Geometry> {
        let content = self.cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            let my_text = Text {
                content: "Hello, Canvas!".to_string(),
                position: Point::new(50.0, 50.0), // Posizione del testo
                color: Color::from_rgb(0.0, 0.0, 0.0), // Colore nero
                size: Pixels(20.0), // Dimensione del testo
                ..Default::default()
            };
            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default().with_width(2.0),
            );
            frame.fill_text(my_text);
            print!("Sono dentro selection: {:?}",self.shapes);
            for shape in &self.shapes {
                println!("{:?}",self.shapes);
                match shape {
                    Shape::Rectangle(rect) => {
                        let rect_path = Path::rectangle(
                            rect.startPoint,
                            Size::new(rect.width, rect.height),
                        );
                        frame.stroke(&rect_path, Stroke::default());
                    }
                    Shape::Circle(circle) => {
                        let circle_path = Path::circle(circle.center, circle.radius);
                        frame.stroke(&circle_path, Stroke::default());
                    }
                    Shape::Arrow(arrow) => {
                        let arrow_path =
                            Path::line(arrow.starting_point, arrow.ending_point);
                        frame.stroke(&arrow_path, Stroke::default());
                    }
                }
            }
        });

        if let Some(Pending::One { from }) = state {
            if let Some(cursor_position) = cursor.position_in(bounds) {
                // Disegno dell'anteprima in base alla forma selezionata
                let preview_path = match self.selected_shape {
                    Some(Shape::Rectangle(_)) => {
                        // Anteprima rettangolo
                        Path::rectangle(
                            *from,
                            Size::new(
                                (cursor_position.x - from.x).abs(),
                                (cursor_position.y - from.y).abs(),
                            ),
                        )
                    }
                    Some(Shape::Circle(_)) => {
                        // Anteprima cerchio
                        let radius = ((cursor_position.x - from.x).powi(2)
                            + (cursor_position.y - from.y).powi(2))
                            .sqrt();
                        Path::circle(*from, radius)
                    }
                    Some(Shape::Arrow(_)) => {
                        // Anteprima freccia
                        Path::line(*from, cursor_position)
                    }
                    _ => {
                        // Forma di default (rettangolo)
                        Path::rectangle(
                            *from,
                            Size::new(
                                (cursor_position.x - from.x).abs(),
                                (cursor_position.y - from.y).abs(),
                            ),
                        )
                    }
                };

                let mut frame = Frame::new(renderer, bounds.size());
                frame.stroke(&preview_path, Stroke::default().with_color(Color::from_rgba(0.0, 0.5, 0.0, 0.5)));
                return vec![content, frame.into_geometry()];
            }
        }

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
                                // Secondo clic: salva il punto finale e crea il rettangolo
                                *state = None; // Resetta lo stato
                                if let Some(shape) = &self.selected_shape {
                                   match shape {
                                       Shape::Rectangle(_) => {
                                           return (
                                               event::Status::Captured,
                                               Some(Message::AddShape(Shape::Rectangle(RectangleCanva {
                                                   startPoint: from,
                                                   width: (cursor_position.x - from.x).abs(),
                                                   height: (cursor_position.y - from.y).abs(),
                                               }))),
                                           );
                                       }
                                       Shape::Circle(_) => {
                                           let radius = ((cursor_position.x - from.x).powi(2)
                                               + (cursor_position.y - from.y).powi(2))
                                               .sqrt();
                                           return (
                                               event::Status::Captured,
                                               Some(Message::AddShape(Shape::Circle(CircleCanva {
                                                   center: from,
                                                   radius,
                                               }))),
                                           );
                                       }
                                       Shape::Arrow(_) => {
                                           return (
                                               event::Status::Captured,
                                               Some(Message::AddShape(Shape::Arrow(ArrowCanva {
                                                   starting_point: from,
                                                   ending_point: cursor_position,
                                               }))),
                                           );
                                       }

                                   }
                                }else{
                                    let radius = ((cursor_position.x - from.x).powi(2)
                                        + (cursor_position.y - from.y).powi(2))
                                        .sqrt();
                                    return (
                                        event::Status::Captured,
                                        Some(Message::AddShape(Shape::Circle(CircleCanva {
                                            center: from,
                                            radius,
                                        }))),
                                    );
                                }
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


impl CanvasWidget {
    pub fn new() -> Self {
        Self {
            shapes:Vec::new(),
            start_point: Default::default(),
            end_point: Default::default(),
            cache: Cache::new(),
            selected_shape:None
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::PendingOne(Pending::One { from }) => {
                self.start_point = from;
            }
            Message::PendingTwo(Pending::Two { from, to }) => {
                self.start_point = from;
                self.end_point = to;

                // Aggiungi una freccia al vettore shapes
                self.shapes.push(Shape::Arrow(ArrowCanva {
                    starting_point: from,
                    ending_point: to,
                }));
                self.cache.clear(); // Forza il ridisegno
            }
            Message::AddShape(shape) => {
                self.shapes.push(shape);
                self.cache.clear(); // Forza il ridisegno
            },
            Message::ClearShape => {
                self.shapes.clear();
                self.cache.clear(); // Forza il ridisegno
            }
            Message::None => {}
            _ => {}
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