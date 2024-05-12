use std::cell::RefCell;
mod chaikin;
use chaikin::chaikin;

use iced::keyboard::key;
use iced::mouse;
use iced::time::Duration;
use iced::widget::canvas::{self, stroke, Canvas, Frame, LineDash, LineJoin, Path, Stroke, Style};
use iced::{keyboard, Color, Command, Element, Length, Point, Rectangle, Renderer, Theme};
pub use iced::{Application, Settings};
use iced_graphics::core::SmolStr;

pub struct App {
    // cache: Cache,
    points: Vec<Point>,
    chaikin_points: Vec<Point>,
    iteration: usize,
    current_iteration: usize,
    run_animation: RefCell<bool>,
    control_pressed: RefCell<bool>,
    dragging: RefCell<bool>,
    drag_point: RefCell<Option<(usize,Point)>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    RunChaikinAnimation,
    Tick,
    MousePressed(Point),
    Reset,
    Quit,
    MovePoint(Point),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                // cache: Cache::default(),
                points: Vec::new(),
                chaikin_points: Vec::new(),
                iteration: 7,
                current_iteration: 0,
                run_animation: RefCell::new(false),
                control_pressed: RefCell::new(false),
                dragging: RefCell::new(false),
                drag_point: RefCell::new(None),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Chaikin's Algorithm Animation")
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::MousePressed(position) => {
                // let position = Point::new(position.x-2.5, position.y-2.5);
                self.points.push(position);
            }
            Message::RunChaikinAnimation => {
                if !*self.run_animation.borrow() {
                    self.run_animation.replace(true);
                }
            }
            Message::Tick => {
                if *self.run_animation.borrow() {
                    if self.current_iteration < self.iteration && self.points.len() > 1 {
                        if self.points.len() == 2 {
                            self.chaikin_points = self.points.clone();
                            // self.run_animation.replace(false);
                            self.current_iteration = 0;
                        } else {
                            self.chaikin_points = chaikin(&self.points, self.current_iteration);
                            self.current_iteration += 1;
                        }
                    } else {
                        if self.points.is_empty() {
                            self.chaikin_points.clear();
                            println!("No points to animate")
                        }
                        // self.run_animation.replace(false);
                        self.current_iteration = 0;
                    }
                }
            } // Handle other messages...
            Message::Quit => {
                std::process::exit(0);
            }
            Message::Reset => {
                self.points.clear();
                self.chaikin_points.clear();
                self.current_iteration = 0;
                self.run_animation.replace(false);
            }
            Message::MovePoint(position) => {
                if *self.dragging.borrow() {
                    if let Some((index,_)) = self.drag_point.borrow().as_ref() {
                        self.points[*index] = Point::new(position.x, position.y);
                    }
                }
            }
             // _ => {}
        }

        // self.cache.clear();
        iced::Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        if *self.run_animation.borrow() {
            iced::time::every(Duration::from_millis(250)).map(|_| Message::Tick)
        } else {
            iced::Subscription::none()
        }
    }

    // Implement other required methods...
}

impl canvas::Program<Message> for App {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if *self.control_pressed.borrow() {
                    self.dragging.replace(true);
                    let position = cursor.position_in(bounds).unwrap();
                    let near_point_index =
                        self.points.iter().enumerate().find_map(|(index, &point)| {
                            let distance = (point.x - position.x).hypot(point.y - position.y);
                            if distance <= 5.0 {
                                Some(index)
                            } else {
                                None
                            }
                        });
                    if let Some(index) = near_point_index {
                        self.drag_point.replace(Some((index,self.points[index])));
                        (canvas::event::Status::Captured, None)
                    } else {
                        (canvas::event::Status::Ignored, None)
                    }
                } else if !*self.run_animation.borrow() {
                    let position = cursor.position_in(bounds);
                    (
                        canvas::event::Status::Captured,
                        Some(Message::MousePressed(position.unwrap())),
                    )
                } else {
                    (canvas::event::Status::Ignored, None)
                }
            }
            canvas::Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
                if key == keyboard::Key::Named(key::Named::Enter) {
                    (
                        canvas::event::Status::Captured,
                        Some(Message::RunChaikinAnimation),
                    )
                } else if key == keyboard::Key::Named(key::Named::Escape) {
                    (canvas::event::Status::Captured, Some(Message::Quit))
                } else if key == keyboard::Key::Character(SmolStr::new("r")) {
                    (canvas::event::Status::Captured, Some(Message::Reset))
                } else if key == keyboard::Key::Named(key::Named::Control) {
                    self.control_pressed.replace(true);
                    (canvas::event::Status::Ignored, None)
                } else {
                    (canvas::event::Status::Ignored, None)
                }
            }
            canvas::Event::Keyboard(keyboard::Event::KeyReleased { key, .. }) => {
                if key == keyboard::Key::Named(key::Named::Control) {
                    self.control_pressed.replace(false);
                    self.dragging.replace(false);
                }
                (canvas::event::Status::Ignored, None)
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if *self.dragging.borrow() {
                    let position = cursor.position_in(bounds).unwrap();
                    (canvas::event::Status::Captured, Some(Message::MovePoint(position)))
                } else {
                    (canvas::event::Status::Ignored, None)
                }
            }
            _ => (canvas::event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<<Renderer as iced::widget::canvas::Renderer>::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        for point in &self.points {
            let start = Point::new(point.x, point.y);
            let end = Point::new(point.x, point.y);

            let circles = Path::new(|b| {
                b.circle(start, 2.5);
                b.move_to(end);
                b.circle(end, 2.5);
            });

            frame.fill(&circles, Color::BLACK);
        }

        // Draw line from first to last point in chaikin_points
        let chaikin_points = &self.chaikin_points.clone();
        if !chaikin_points.is_empty() {
            let path = Path::new(|path_builder| {
                path_builder.move_to(chaikin_points[0]);
                for point in chaikin_points {
                    path_builder.line_to(*point);
                }
            });

            let stroke = Stroke {
                style: Style::Solid(Color::BLACK),
                line_dash: LineDash {
                    segments: &[],
                    offset: 0,
                },
                width: 1.0,
                line_cap: stroke::LineCap::Round,
                line_join: LineJoin::Round,
            };

            frame.stroke(&path, stroke);
        }

        vec![frame.into_geometry()]
    }
}
