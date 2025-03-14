use egui::{CentralPanel, Layout, RichText, ScrollArea, SidePanel, TopBottomPanel};
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
        TopBottomPanel::top("Vocabulary_Trainer")
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Vocabulary Trainer").strong().size(24.0))
                });
            });
        // Display each entry in the 'entries' vector as a label.
        SidePanel::left("Word Entries").show(ctx, |ui| {
            for (i, entry) in self.entries.iter().enumerate() {
                if let Some(new_word) = Some(&entry.hwi.value) {
                    ui.heading(
                        RichText::new(format!("{}: {}", i, new_word))
                            .strong()
                            .size(18.0),
                    );
                    ui.separator();
                }
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            // Clone the current word or provide a placeholder text if it's None.
            let mut word_to_lookup = self
                .current_word
                .clone()
                .unwrap_or_else(|| "Enter a word to look up".to_string());
            // Check if the single-line text edit has changed and update the state accordingly.

            ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.text_edit_singleline(&mut word_to_lookup).changed() {
                    self.current_word = Some(word_to_lookup.clone());
                }
                if ui.button("Get the definition!").clicked() {
                    // If the button "Get the definition!" is clicked, fetch the definition for the current word.
                    if let Some(ref current_word) = self.current_word {
                        self.fetch_definition(current_word.clone());
                    }
                }
            });
            ui.separator();
            // ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
            ScrollArea::both().show(ui, |ui| {
                if let Some(ref current_word) = self.current_word {
                    ui.vertical_centered(|ui| {
                        ui.heading(RichText::new(current_word).strong().underline().size(18.0));
                    });
                    ui.vertical(|ui| {
                        for entry in &self.entries {
                            ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                                if let Some(fl) = &entry.fl {
                                    let fmt_hw =
                                        RichText::new(&entry.hwi.value).italics().size(16.0);
                                    let fmt_fl = RichText::new(fl).italics().size(14.0);
                                    ui.label(fmt_hw);
                                    ui.label(fmt_fl);
                                }
                            });
                            match &entry.def {
                                Some(ref defs) => {
                                    for (idx, d) in defs.iter().enumerate() {
                                        ui.label(format!("{:?}", d.sense_sequence));
                                        ui.separator();
                                    }
                                }
                                None => {} // Handle the case where 'def' is None
                            };
                        }
                    });
                }
            });
            // }); // ui.with_layout()
        });
    }
}
