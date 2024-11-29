use iced::{
    theme::Theme,
    widget::{
        button, column, container, container::Style, row, scrollable, text, text_input, Column,
        Text,
    },
    Border, Center, Color, Element, Fill, Shrink, Task,
};

#[derive(Debug, Clone)]
struct Evaluacion {
    nombre: String,
    peso: u8,
    nota: u8,
}

#[derive(Debug, Clone, Default)]
struct Application {
    evaluaciones_pasadas: Vec<Evaluacion>,
    evaluaciones_futuras: Vec<Evaluacion>,

    nota_total_acumulada: u8,

    nombre_entrada: String,
    peso_entrada: String,
    nota_entrada: String,
}

#[derive(Debug, Clone)]
enum Message {
    NombreCambiada(String),
    PesoCambiada(String),
    NotaCambiada(String),
    AgregarPasadaPresionado,
    AgregarFuturaPresionado,
    EliminarEvaluacionPresionado,
}

impl Application {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                evaluaciones_pasadas: vec![],
                evaluaciones_futuras: vec![],

                nota_total_acumulada: 0,

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

        for i in self.evaluaciones_pasadas.clone() {
            let nombre = i.nombre;
            let peso = i.peso;
            let nota = i.nota;

            evaluaciones.push(
                container(text(format!("{nombre} ({peso}) -> {nota}")).size(20))
                    .padding(20)
                    .style(container::bordered_box)
                    .into(),
            );
        }

        container(column![
            text(format!(
                "Nota total acumulada: {} ({}%)",
                20 * self.nota_total_acumulada,
                self.nota_total_acumulada
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
            container(
                row![
                    button("Agregar Evaluación")
                        .style(button::secondary)
                        .on_press(Message::AgregarPasadaPresionado),
                    button("Agregar Pendiente")
                        .style(button::primary)
                        .on_press(Message::AgregarFuturaPresionado),
                    button("Eliminar Evaluación")
                        .style(button::danger)
                        .on_press(Message::EliminarEvaluacionPresionado),
                ]
                .align_y(Center)
                .spacing(10)
                .padding(20),
            )
            //.style(Application::style)
            .center_x(Fill)
            .center_y(Shrink)
            .align_y(Center)
            .padding(20),
        ])
        .center_x(Fill)
        .center_y(Fill)
        .align_y(Center)
        .padding(20)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AgregarPasadaPresionado => self.evaluaciones_pasadas.push(Evaluacion {
                nombre: "Pene".to_string(),
                peso: 25,
                nota: 30,
            }),

            Message::AgregarFuturaPresionado => {}

            _ => {}
        }
        //    Message::FirstContentChanged(content) => {
        //        if content.is_empty() {
        //            //self.text_input1 = String::from("");
        //            //self.first = 0;
        //        }
        //
        //        if let Ok(number) = content.parse::<i64>() {
        //            //self.text_input1 = content;
        //            //self.first = number;
        //        }
        //    }
        //
        //    Message::SecondContentChanged(content) => {
        //        if content.is_empty() {
        //            //self.text_input2 = String::from("");
        //            //self.second = 0;
        //        }
        //
        //        if let Ok(number) = content.parse::<i64>() {
        //            //self.text_input2 = content;
        //            //self.second = number;
        //        }
        //    }
        //
        //    Message::Calculate => {
        //        //self.result = self.first + self.second;
        //    }
        //}
    }
}

fn main() -> iced::Result {
    iced::application(Application::title, Application::update, Application::view)
        .theme(|_| Theme::CatppuccinMocha)
        .run_with(Application::new)
}
