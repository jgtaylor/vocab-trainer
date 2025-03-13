use egui::{CentralPanel, RichText, SidePanel};
use merriam_webster_model::Entry;
use reqwest::blocking;

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
    Learners,
    Collegiate,
}

impl Default for VocabTrainer {
    fn default() -> Self {
        let prefered_dictionary = Dictionary::Learners;
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
        let base_url = "https://dictionaryapi.com/api/v3/references/".to_string();
        let url = match &self.prefered_dictionary {
            Dictionary::Learners => format!(
                "{}learners/json/{word}?key=a677e0ca-3c64-49e3-8366-ffaed5d8979a",
                base_url
            ),
            Dictionary::Collegiate => format!(
                "{}collegiate/json/{word}?key=a677e0ca-3c64-49e3-8366-ffaed5d8979a",
                base_url
            ),
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
            // Add a heading with the title "Vocabulary Trainer".
            ui.heading(RichText::new("Vocabulary Trainer").strong().size(24.0));

            // Clone the current word or provide a placeholder text if it's None.
            let mut word_to_lookup = self
                .current_word
                .clone()
                .unwrap_or_else(|| "Enter a word to look up".to_string());
            // Check if the single-line text edit has changed and update the state accordingly.
            if ui.text_edit_singleline(&mut word_to_lookup).changed() {
                self.current_word = Some(word_to_lookup.clone());
            }
            // If the button "Get the definition!" is clicked, fetch the definition for the current word.
            if ui.button("Get the definition!").clicked() {
                if let Some(ref current_word) = self.current_word {
                    self.fetch_definition(current_word.clone());
                }
            }
            // Display each entry in the 'entries' vector as a label.
            SidePanel::left("Word Entries").show_inside(ui, |ui| {
                for (i, entry) in self.entries.iter().enumerate() {
                    if let Some(main_word) = Some(&entry.hwi.value) {
                        ui.heading(
                            RichText::new(format!("{}: {}", i, main_word))
                                .strong()
                                .size(18.0),
                        );
                        ui.separator();
                    }
                }
            });
        });
    }
}
