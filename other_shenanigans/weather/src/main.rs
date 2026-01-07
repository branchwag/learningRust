use eframe::egui;
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

struct WeatherApp {
    city: String,
    temperature: Option<f32>,
    description: Option<String>,
    error: Option<String>,
}

impl WeatherApp {
    fn new() -> Self {
        Self {
            city: String::new(),
            temperature: None,
            description: None,
            error: None,
        }
    }
}

impl WeatherApp {
    fn fetch_weather(&mut self) {
        let api_key = "YOUR_API_KEY";
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
            self.city, api_key
        );

        match reqwest::blocking::get(url) {
            Ok(resp) => match resp.json::<WeatherResponse>() {
                Ok(data) => {
                    self.temperature = Some(data.main.temp);
                    self.description = Some(data.weather[0].description.clone());
                    self.error = None;
                }
                Err(_) => self.error = Some("Failed to parse weather data".into()),
            },
            Err(_) => self.error = Some("Failed to fetch weather".into()),
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height();

            // Push content down to vertical center
            ui.add_space(available_height * 0.25);

            ui.vertical_centered(|ui| {
                ui.heading("Rain or Shine?");
                ui.add_space(20.0);

                // Big "City" label
                ui.label(egui::RichText::new("City").size(24.0).strong());

                ui.add_space(8.0);

                // Centered, wide text input
                ui.add_sized(
                    [400.0, 36.0],
                    egui::TextEdit::singleline(&mut self.city).hint_text("Enter a city…"),
                );

                ui.add_space(12.0);

                if ui.button("Get Weather").clicked() {
                    self.fetch_weather();
                }
            });

            ui.add_space(20.0);

            if let Some(temp) = self.temperature {
                ui.label(format!("Temperature: {:.1}°C", temp));
            }

            if let Some(desc) = &self.description {
                ui.label(format!("Conditions: {}", desc));
            }

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, err);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Weather App",
        options,
        Box::new(|_cc| Ok(Box::new(WeatherApp::new()))),
    )
}
