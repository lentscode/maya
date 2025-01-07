use inquire::{ui::{Color, RenderConfig, StyleSheet}, Select, Text};
use std::{
  fs::write,
  io::Result,
  path::{Path, PathBuf},
};

mod templates;

use templates::Template;

pub struct Create {}

impl Create {
  pub fn run() -> Result<()> {
    let template = &Self::get_template();

    let filename = Text::new("Enter the filename:")
      .with_default(&template.filename)
      .with_render_config(RenderConfig::default_colored().with_default_value(Self::get_default_value_style()))
      .prompt()
      .expect("Failed to enter filename");
    
    let directory = Text::new("Enter the directory to create the file in:")
    .with_default("./")
    .with_render_config(RenderConfig::default_colored().with_default_value(Self::get_default_value_style()))
      .prompt()
      .expect("Failed to enter directory");
    let path = Path::new(&directory).join(filename);

    Self::create_file(template, &path)?;

    Ok(())
  }

  fn get_template() -> Template {
    let template_chosen = Select::new(
      "Select a template:",
      {
        let mut templates: Vec<_> = templates::TEMPLATES.iter().collect();
        templates.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        templates
          .into_iter()
          .map(|t| format!("{} - {}", t.name, t.description))
          .collect()
      }
    )
    .prompt()
    .expect("Failed to select template");

    let template_name = template_chosen.split(" - ").next().expect("Failed to get template name");
    let template = templates::TEMPLATES
      .iter()
      .find(|t| t.name == template_name)
      .expect("Template not found");

    template.clone()
  }

  fn create_file(template: &Template, path: &PathBuf) -> Result<()> {
    write(path, template.content)?;
    Ok(())
  }

  fn get_default_value_style() -> StyleSheet {
    StyleSheet::new().with_fg(Color::Black)
  }
}
