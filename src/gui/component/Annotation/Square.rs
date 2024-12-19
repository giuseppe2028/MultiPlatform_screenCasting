use futures::pending;
use iced::{Length, mouse, Pixels, Point, Size};
use iced::widget::{canvas};
use iced::{Color, Rectangle, Renderer};
use iced::mouse::Cursor;

use iced::widget::canvas::{Cache, Frame, Geometry, Path, Program, Stroke, Style, Text};
use iced::widget::canvas::path::lyon_path::geom::size;
use crate::gui::app::Message;
use crate::gui::theme::Theme;
use crate::gui::theme::widget::Canvas;
use iced::widget::canvas::event::{Event,self};
use url::Position;
// First, we define the data we need for drawing

#[derive(Debug)]
pub struct CanvasWidget {
    pub shapes: Vec<Shape>,
    pub start_point:Point,
    pub end_point:Point,
    pub cache: canvas::Cache,
    pub selected_shape: Option<Shape>,
    pub textSelected:TextCanva,
    pub all_text_selected:Vec<TextCanva>,
    pub text_status:Status
}

#[derive(Debug)]
pub enum Status{
    None,
    TextPressed,
    TextPositioned,
    TextAdded
}

#[derive(Debug, Clone)]
pub enum Shape {
    Rectangle(RectangleCanva),
    Circle(CircleCanva),
    Arrow(ArrowCanva),
    Line(LineCanva)
}

#[derive(Debug, Clone)]
pub struct RectangleCanva {
    pub(crate) startPoint:Point,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Clone, Debug)]
pub struct TextCanva{
    pub(crate) position:Point,
    pub(crate) text:String
}

#[derive(Debug, Clone)]
pub struct CircleCanva {
    pub(crate) center: Point,
    pub(crate) radius: f32,
}

#[derive(Debug, Clone)]
pub struct ArrowCanva {
    pub(crate) starting_point: Point,
    pub(crate) ending_point: Point,
}

#[derive(Debug, Clone)]
pub struct LineCanva {
    pub(crate) starting_point: Point,
    pub(crate) ending_point: Point,
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

            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default().with_width(3.0),
            );

            for shape in &self.shapes {
                println!("{:?}",self.shapes);
                match shape {
                    Shape::Line(line)=>{
                        let rect_path = Path::line(line.starting_point,line.ending_point);
                        frame.stroke(&rect_path, Stroke{
                            style:Style::Solid(Color::BLACK),
                            width: 3.0,
                            line_cap: Default::default(),
                            line_join: Default::default(),
                            line_dash: Default::default(),
                        });
                    }
                    Shape::Rectangle(rect) => {
                        let rect_path = Path::rectangle(
                            rect.startPoint,
                            Size::new(rect.width, rect.height),
                        );
                        frame.stroke(&rect_path, Stroke{
                            style:Style::Solid(Color::BLACK),
                            width: 3.0,
                            line_cap: Default::default(),
                            line_join: Default::default(),
                            line_dash: Default::default(),
                        });
                    }
                    Shape::Circle(circle) => {
                        let circle_path = Path::circle(circle.center, circle.radius);
                        frame.stroke(&circle_path, Stroke{
                            style:Style::Solid(Color::BLACK),
                            width: 3.0,
                            line_cap: Default::default(),
                            line_join: Default::default(),
                            line_dash: Default::default(),
                        });
                    }
                    Shape::Arrow(arrow) => {
                        // Definisci i punti della freccia
                        let starting_point = arrow.starting_point; // Ad esempio, Point::new(50.0, 50.0)
                        let ending_point = arrow.ending_point;     // Ad esempio, Point::new(150.0, 100.0)

                        // Calcola i punti per la punta della freccia
                        let arrow_head_length = 10.0;
                        let arrow_head_width = 5.0;

                        let direction = Point::new(
                            ending_point.x - starting_point.x,
                            ending_point.y - starting_point.y,
                        );

                        let magnitude = (direction.x.powi(2) + direction.y.powi(2)).sqrt();
                        let unit_direction = Point::new(direction.x / magnitude, direction.y / magnitude);

                        let left_wing = Point::new(
                            ending_point.x - unit_direction.x * arrow_head_length - unit_direction.y * arrow_head_width,
                            ending_point.y - unit_direction.y * arrow_head_length + unit_direction.x * arrow_head_width,
                        );

                        let right_wing = Point::new(
                            ending_point.x - unit_direction.x * arrow_head_length + unit_direction.y * arrow_head_width,
                            ending_point.y - unit_direction.y * arrow_head_length - unit_direction.x * arrow_head_width,
                        );

                        // Crea il percorso della freccia
                        let mut arrow_path = Path::new(|builder| {
                            // Linea principale
                            builder.move_to(starting_point);
                            builder.line_to(ending_point);

                            // Aggiungi la punta della freccia
                            builder.move_to(left_wing);
                            builder.line_to(ending_point);
                            builder.line_to(right_wing);
                        });

                        // Disegna la freccia
                        frame.stroke(&arrow_path,Stroke{
                            style:Style::Solid(Color::BLACK),
                            width: 3.0,
                            line_cap: Default::default(),
                            line_join: Default::default(),
                            line_dash: Default::default(),
                        });
                    }
                }
            }
            print!("{:?}",self.all_text_selected);
            for text in &self.all_text_selected{
                let my_text = Text {
                    content: text.clone().text,
                    position:text.position, // Posizione del testo
                    color: Color::from_rgb(0.0, 0.0, 0.0), // Colore nero
                    size: Pixels(20.), // Dimensione del testo
                    ..Default::default()
                };

                frame.fill_text(my_text)
            }
        });

        if let Some(Pending::One { from }) = state {
            if let Some(cursor_position) = cursor.position_in(bounds) {
                print!("{:?}",cursor_position);
                // Disegno dell'anteprima in base alla forma selezionata
                let preview_path = match self.selected_shape {
                    Some(Shape::Rectangle(_)) => {
                        // Anteprima rettangolo
                        Path::rectangle(
                            *from,
                            Size::new(
                                cursor_position.x - from.x,
                                cursor_position.y - from.y,
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
                    },
                    Some(Shape::Line(_)) => {
                        // Anteprima freccia
                        Path::line(*from, cursor_position)
                    }
                    _ => {
                        // Forma di default (rettangolo)
                        Path::rectangle(
                            *from,
                            Size::new(
                                cursor_position.x - from.x,
                                cursor_position.y - from.y,
                            ),
                        )
                    }
                };

                let mut frame = Frame::new(renderer, bounds.size());
                frame.stroke(&preview_path, Stroke::default().with_color(Color::from_rgba(0.0, 0.5, 0.0, 0.5)).with_width(3.0));
                return vec![content, frame.into_geometry()];
            }
        }

        vec![content]
    }

    fn update(
        & self,
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
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match *state {
                            None => {
                                println!("{:?}",self.text_status);
                                if let Status::TextPressed = self.text_status{
                                    println!("pressed");
                                    /*
                                    self.textSelected.position = cursor_position;*/
                                    *state =None;
                                    return (
                                        event::Status::Captured,
                                        Some(Message::SaveTextPosition(cursor_position))
                                    );
                                }else{
                                    *state = Some(Pending::One {
                                        from: cursor_position,
                                    });
                                    Some(Message::PendingOne(Pending::One {
                                        from: cursor_position,
                                    }))
                                }

                            }
                            Some(Pending::One { from }) => {
                                // Secondo clic: salva il punto finale e crea il rettangolo
                                *state = None; // Resetta lo stato
                                if let Some(shape) = &self.selected_shape {
                                   match shape {
                                       Shape::Line(_)=>{
                                           return (
                                               event::Status::Captured,
                                               Some(Message::AddShape(Shape::Line(LineCanva{
                                                   starting_point: from,
                                                   ending_point: cursor_position,
                                               }))),
                                           );
                                       }
                                       Shape::Rectangle(_) => {
                                           return (
                                               event::Status::Captured,
                                               Some(Message::AddShape(Shape::Rectangle(RectangleCanva {
                                                   startPoint: from,
                                                   width: cursor_position.x - from.x,
                                                   height: cursor_position.y - from.y,
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
                                       },

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
            selected_shape:None,
            textSelected:TextCanva{ position: Default::default(), text: "".to_string() },
            all_text_selected: vec![],
            text_status: Status::None,
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::PendingOne(Pending::One { from }) => {
                self.start_point = from;
                println!("almento o qua");
            }
            Message::PendingTwo(Pending::Two { from, to }) => {
                println!("almento o qua1");
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
                println!(" SONO IN Add shape");
                self.shapes.push(shape);
                self.cache.clear(); // Forza il ridisegno
            },
            Message::SaveTextPosition(cord)=>{
                println!("almento qua");
            }
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

        frame.stroke(&curves, Stroke::default().with_width(2.0).with_width(3.0));
    }
}




#[derive(Debug, Clone, Copy)]
pub enum Pending {
    One { from: Point },
    Two { from: Point, to: Point },
    None {position: Point}
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
                    frame.stroke(&line, Stroke::default().with_width(2.0).with_width(3.0));
                }
                Pending::Two { from, to } => {
                    let curve = Curve {
                        from,
                        to,
                    };

                    Curve::draw_all(&[curve], &mut frame);
                }
                Pending::None{position} => {}
            };
        }

        frame.into_geometry()
    }
}