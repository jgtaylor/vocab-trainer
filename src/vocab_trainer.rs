use egui::{CentralPanel, Widget, RichText};
use merriam_webster_model::Entry;
use reqwest::{blocking, RequestBuilder};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VocabTrainer {
    // Example stuff:
    pub prefered_dictionary: Dictionary,
    pub current_word: Option<String>,
    pub entries: Vec<Entry>, // Store fetched entries here
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Dictionary {
    Learners(String),
    Collegiate(String),
}

impl Default for VocabTrainer {
    fn default() -> Self {
        let prefered_dictionary = Dictionary::Learners(
            "https://dictionaryapi.com/api/v3/references/learners/json/".to_string(),
        );
        Self {
            prefered_dictionary,
            current_word: None,
            entries: Vec::new(), // Initialize as an empty vector
            // Example stuff:
        }
    }
}

impl VocabTrainer {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn fetch_definition(&mut self, word: String) {
        let url = match &self.prefered_dictionary {
            Dictionary::Learners(base_url) => format!("{}{word}?key=a677e0ca-3c64-49e3-8366-ffaed5d8979a", base_url),
            Dictionary::Collegiate(base_url) => format!("{}{word}?key=a677e0ca-3c64-49e3-8366-ffaed5d8979a", base_url),
        };

        let response = blocking::get(url).expect("Failed to fetch definition");
        self.entries = response.json::<Vec<Entry>>().expect("Failed to parse JSON");
    }
}

impl eframe::App for VocabTrainer {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("Vocabulary Trainer").strong().size(24.0));
            
            let mut word_to_lookup = self.current_word.clone().unwrap_or_else(|| "Enter a word to look up".to_string());
            
            if ui.text_edit_singleline(&mut word_to_lookup).changed() {
                self.current_word = Some(word_to_lookup.clone());
            }

            if ui.button("Get the definition!").clicked() {
                if let Some(ref current_word) = self.current_word {
                    self.fetch_definition(current_word.clone());
                }
            }

            for entry in &self.entries {
                ui.add(egui::Label::new(format!("{:?}", entry)));
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {}
