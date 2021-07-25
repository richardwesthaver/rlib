use orbtk::{
  prelude::*,
  widgets::themes::theme_orbtk::{THEME_DEFAULT, THEME_DEFAULT_COLORS_DARK, THEME_DEFAULT_FONTS},
};

type List = Vec<String>;
use serde_derive::{Deserialize, Serialize};

static THEME: &str = include_str!("../tests/assets/theme.ron");

fn theme() -> Theme {
  orbtk::widgets::themes::theme_orbtk::register_default_fonts(Theme::from_config(
    ThemeConfig::from(THEME)
      .extend(ThemeConfig::from(THEME_DEFAULT))
      .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
      .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
  ))
}

widget!(Canvas { text: String });

impl Template for Canvas {
  fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
    self.name("Canvas").text("DMC").child(
      Grid::new()
        .rows("80, *")
        .child(
          Container::new()
            .padding(8)
            .attach(Grid::row(0))
            .child(
              Grid::new()
                .child(
                  ScrollViewer::new()
                    .mode(("custom", "disabled"))
                    .child(
                      TextBlock::new()
                        .width(0)
                        .height(14)
                        .text("")
                        .style("input")
                        .id("input")
                        .v_align("start")
                        .build(ctx),
                    )
                    .build(ctx),
                )
                .child(
                  TextBlock::new()
                    .style("result")
                    .text(id)
                    .v_align("end")
                    .h_align("end")
                    .build(ctx),
                )
                .build(ctx),
            )
            .build(ctx),
        )
        .build(ctx),
    )
  }
}

fn main() {
  let app = Application::new().theme(theme()).window(|ctx| {
    Window::new()
      .title("Anti_Corp")
      .resizeable(true)
      .position((0., 0.))
      .size(710.0, 710.0)
      .child(
        Stack::new()
          .child(
            TextBlock::new()
              .text("Greetings Stranger,")
              .style("header")
              .build(ctx),
          )
          .child(Button::new().text("GET").margin(4.0).build(ctx))
          .child(
            ImageWidget::new()
              .image("tests/assets/image.png")
              .margin(4.0)
              .build(ctx),
          )
          .child(Canvas::new().build(ctx))
          .build(ctx),
      )
      .build(ctx)
  });
  app.run();
}
