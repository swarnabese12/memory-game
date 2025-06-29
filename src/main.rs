use eframe::egui;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, PartialEq)]
enum CardState {
    Hidden,
    Revealed,
    Matched,
}

#[derive(Clone)]
struct Card {
    symbol: char,
    state: CardState,
}

pub struct MemoryGameApp {
    grid: Vec<Card>,
    first_selection: Option<usize>,
    second_selection: Option<usize>,
    moves: u32,
    wait_timer: f32,
}

impl Default for MemoryGameApp {
    fn default() -> Self {
        let symbols = [
            '🐱', '🐱',
            '🐶', '🐶',
            '👻', '👻',
            '👽', '👽',
            '💻', '💻',
            '🚀', '🚀',
            '🔥', '🔥',
            '🐞', '🐞',
        ];
        let mut cards: Vec<Card> = symbols
            .iter()
            .map(|&s| Card {
                symbol: s,
                state: CardState::Hidden,
            })
            .collect();

        cards.shuffle(&mut thread_rng());

        Self {
            grid: cards,
            first_selection: None,
            second_selection: None,
            moves: 0,
            wait_timer: 0.0,
        }
    }
}

impl eframe::App for MemoryGameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🧠 Memory Game");
            ui.label(format!("Moves: {}", self.moves));

            let all_matched = self.grid.iter().all(|card| card.state == CardState::Matched);

            if all_matched {
                ui.label(egui::RichText::new("🎉 You Win! 🎉").size(32.0).strong().color(egui::Color32::GOLD));
                return;
            }

            if self.wait_timer > 0.0 {
                self.wait_timer -= ctx.input(|i| i.unstable_dt);
                if self.wait_timer <= 0.0 {
                    if let (Some(a), Some(b)) = (self.first_selection, self.second_selection) {
                        if self.grid[a].symbol != self.grid[b].symbol {
                            self.grid[a].state = CardState::Hidden;
                            self.grid[b].state = CardState::Hidden;
                        } else {
                            self.grid[a].state = CardState::Matched;
                            self.grid[b].state = CardState::Matched;
                        }
                        self.first_selection = None;
                        self.second_selection = None;
                    }
                }
                ctx.request_repaint();
                return;
            }

            egui::Grid::new("game_grid").show(ui, |ui| {
                for (i, card) in self.grid.iter_mut().enumerate() {
                    let label = match card.state {
                        CardState::Hidden => "🎴".to_string(),
                        CardState::Revealed | CardState::Matched => card.symbol.to_string(),
                    };

                    let rich_label = egui::RichText::new(label).size(24.0);
                    let button = egui::Button::new(rich_label);

                    if ui
                        .add_sized([60.0, 60.0], button)
                        .clicked()
                        && card.state == CardState::Hidden
                    {
                        card.state = CardState::Revealed;
                        if self.first_selection.is_none() {
                            self.first_selection = Some(i);
                        } else if self.second_selection.is_none() {
                            self.second_selection = Some(i);
                            self.moves += 1;
                            self.wait_timer = 1.0;
                        }
                    }

                    if (i + 1) % 4 == 0 {
                        ui.end_row();
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "🧠 Memory Game",
        options,
        Box::new(|_cc| Box::new(MemoryGameApp::default())),
    )
}
