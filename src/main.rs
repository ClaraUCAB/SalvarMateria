use iced::{
    theme::Theme,
    widget::{
        button, checkbox, column, container, container::Style, image, keyed_column, pane_grid, row,
        scrollable, slider, text, text_input, toggler, Button, Column, Space, Text,
    },
    Border, Center, Color, ContentFit, Element, Fill, Length, Shrink, Task,
};

#[derive(Debug, Clone)]
struct Evaluacion {
    id: usize,
    nombre: String,
    peso: u8,
    nota: u8,
    slider: bool,
}

#[derive(Debug)]
enum Pane {
    SomePane,
    AnotherKindOfPane,
}

#[derive(Debug, Clone, Default)]
struct Application {
    evaluaciones: Vec<Evaluacion>,

    nota_total_acumulada: u8,
    peso_total_acumulado: u8,

    nombre_entrada: String,
    peso_entrada: String,
    nota_entrada: String,
}

#[derive(Debug, Clone)]
enum Message {
    NombreCambiada(String),
    PesoCambiada(String),
    NotaCambiada(String),
    AgregarEvaluacionPresionado,
    EliminarEvaluacionPresionado(usize),
    SliderCambiado(usize, f32),
    Ignore,
}

fn porcentaje_de_nota(nota: u8, peso: u8) -> f32 {
    (nota as f32 / 20.0 * 100.0) * (peso as f32 / 100.0)
}

impl Application {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                evaluaciones: vec![],

                nota_total_acumulada: 0,
                peso_total_acumulado: 0,

                nombre_entrada: String::from(""),
                peso_entrada: String::from(""),
                nota_entrada: String::from(""),
            },
            Task::none(),
        )
    }

    fn style(theme: &Theme) -> Style {
        let palette = theme.palette();

        Style::default().border(
            Border::default()
                //.color(Color::new(1.0, 1.0, 1.0, 1.0))
                .color(palette.primary)
                .width(2),
        )
    }

    fn title(&self) -> String {
        String::from("Salvar Materia")
    }

    fn view(&self) -> Element<'_, Message> {
        //evaluaciones.push(text("lol").into());
        let mut evaluaciones: Vec<Element<Message>> = vec![];

        for i in self.evaluaciones.clone() {
            let nombre: String = i.nombre;
            let peso: u8 = i.peso;
            let nota: u8 = i.nota;

            evaluaciones.push(
                container(row![
                    container(
                        Button::new(image("assets/trash.png").content_fit(ContentFit::Fill))
                            .on_press(Message::EliminarEvaluacionPresionado(i.id))
                            .style(button::danger)
                            .padding(15)
                            .height(66)
                            .width(66),
                    )
                    .style(container::bordered_box),
                    container(text(nombre).size(20))
                        .padding(20)
                        .width(300)
                        .style(container::bordered_box),
                    container(text(format!("{peso}%")).size(20))
                        .padding(20)
                        .width(100)
                        .align_x(Center)
                        .align_y(Center)
                        .style(container::bordered_box),
                    if !i.slider {
                        container(text(format!("{} pts", nota)).size(20)).padding(20)
                    } else {
                        container(
                            column![
                                text(format!("{} pts", nota)).size(20),
                                slider(0.0..=20.0, nota as f32, move |j| {
                                    Message::SliderCambiado(i.id, j)
                                })
                            ]
                            .align_x(Center)
                            .padding(10)
                            .spacing(4),
                        )
                        .align_x(Center)
                        .center_y(Shrink)
                        //.align_y(Center)
                    }
                    .width(100)
                    .style(container::bordered_box)
                ])
                .style(container::bordered_box)
                .into(),
            );
        }

        container(
            column![
                text(format!(
                    "Nota total acumulada: {}/20 ó {}/100% ({}% evaluado)",
                    (0.2f32 * self.nota_total_acumulada as f32).floor(),
                    self.nota_total_acumulada,
                    self.peso_total_acumulado,
                    //self.evaluaciones.iter().map(|i| i.peso).sum::<u8>(),
                ))
                .size(20),
                container(scrollable(
                    container(Column::from_vec(evaluaciones).align_x(Center).spacing(20))
                        .center_x(Fill)
                ))
                .style(Application::style)
                .center_x(Fill)
                .center_y(Fill)
                .align_y(Center)
                .padding(20),
                container(column![row![
                    text_input("Nombre de la evaluación", &self.nombre_entrada)
                        .on_input(Message::NombreCambiada),
                    text_input("Valor de la evaluación", &self.peso_entrada)
                        .on_input(Message::PesoCambiada),
                    text_input("Puntaje obtenido (Opcional)", &self.nota_entrada)
                        .on_input(Message::NotaCambiada),
                    button("Agregar")
                        .style(button::primary)
                        .on_press(Message::AgregarEvaluacionPresionado),
                ]
                .spacing(20),])
                .center_x(Fill)
                .center_y(Shrink)
                .align_y(Center)
            ]
            .spacing(20),
        )
        .center_x(Fill)
        .center_y(Fill)
        .align_y(Center)
        .padding(100)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AgregarEvaluacionPresionado => {
                // No need to check for the  last entry as that one's optional
                if self.nombre_entrada.is_empty() || self.peso_entrada.is_empty() {
                    // TODO: Add some kind of warning to the user
                    return;
                }

                let id = if let Some(i) = self.evaluaciones.iter().last() {
                    i.id + 1
                } else {
                    0
                };

                let nombre = self.nombre_entrada.to_string();
                let peso = self.peso_entrada.parse::<u8>().unwrap_or_default();
                let nota = self.nota_entrada.parse::<u8>().unwrap_or_default();
                let slider = self.nota_entrada.is_empty();

                self.nombre_entrada.clear();
                self.peso_entrada.clear();
                self.nota_entrada.clear();

                self.evaluaciones.push(Evaluacion {
                    id,
                    nombre,
                    peso,
                    nota,
                    slider,
                })
            }

            Message::EliminarEvaluacionPresionado(id) => {
                if let Some(index) = self.evaluaciones.iter().position(|i| i.id == id) {
                    self.evaluaciones.remove(index);
                }
            }

            Message::NombreCambiada(content) => {
                if content.len() <= 20 {
                    self.nombre_entrada = content;
                }
            }

            Message::PesoCambiada(content) => {
                if content.is_empty() {
                    self.peso_entrada = content;
                    return;
                } else if let Ok(peso) = content.parse::<u8>() {
                    if self.peso_total_acumulado + peso <= 100 {
                        self.peso_entrada = content;
                    }
                }
            }

            Message::NotaCambiada(content) => {
                if content.is_empty() {
                    self.nota_entrada = content;
                    return;
                } else if let Ok(nota) = content.parse::<u8>() {
                    if nota <= 20 {
                        self.nota_entrada = content;
                    }
                }
            }

            Message::SliderCambiado(id, value) => {
                if let Some(index) = self.evaluaciones.iter().position(|i| i.id == id) {
                    self.evaluaciones[index].nota = value.floor() as u8;
                }
            }

            Message::Ignore => {}
        }

        // Actualiza la nota total acumulada
        self.nota_total_acumulada = self
            .evaluaciones
            .iter()
            .map(|i| porcentaje_de_nota(i.nota, i.peso) as u8)
            .sum();
        // TODO: Handle slider

        // Actualiza el peso total acumulado
        self.peso_total_acumulado = self.evaluaciones.iter().map(|i| i.peso).sum();
    }
}

fn main() -> iced::Result {
    iced::application(Application::title, Application::update, Application::view)
        .theme(|_| Theme::CatppuccinMocha)
        .run_with(Application::new)
}
