use iced::border::Radius;
use iced::widget::{Canvas, canvas};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, mouse};

pub struct CycleBackground {
    pub progress: f32,
    pub fill_color: Color,
    pub background_color: Color,
}

impl<Message> canvas::Program<Message> for CycleBackground {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let size = bounds.size();
        let progress = self.progress.clamp(0.0, 1.0);

        let mut frame = canvas::Frame::new(renderer, size);

        // Background
        let background = canvas::Path::rounded_rectangle(Point::ORIGIN, size, Radius::from(8.0));

        frame.fill(&background, self.background_color);

        // Filled progress
        let progress = canvas::Path::rounded_rectangle(
            Point::ORIGIN,
            Size::new(size.width * progress, size.height),
            Radius::new(8.0),
        );

        frame.stroke(
            &background,
            canvas::Stroke {
                width: 1.0,
                style: canvas::Style::Solid(Color::from_rgb8(255, 255, 255)),
                ..Default::default()
            },
        );

        frame.fill(&progress, self.fill_color);

        vec![frame.into_geometry()]
    }
}

pub fn view<'a, Message>(progress: f32, fill_color: Color) -> Element<'a, Message>
where
    Message: 'a,
{
    Canvas::new(CycleBackground {
        progress,
        fill_color,
        background_color: Color::from_rgb8(125, 125, 125),
    })
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}
