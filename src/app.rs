use crate::agents::{Agent, AgentLoader, AgentRegistry};
use crate::openrouter::OpenRouterClient;
use crate::render::{OutputRenderer, OutputType};
use crate::settings::Settings;
use crate::storage::Storage;
use crate::ui::{InputRenderer, View, Colors, Typography, Spacing, horizontal_rule, primary_button, secondary_button, card_frame, inverted_card_frame};
use egui::Color32;
use std::collections::HashMap;

pub struct OpenAgentApp {
    view: View,
    storage: Storage,
    settings: Settings,
    agent_loader: AgentLoader,
    installed_agents: Vec<Agent>,
    community_agents: Vec<Agent>,
    selected_agent: Option<Agent>,
    input_values: HashMap<String, String>,
    search_queries: HashMap<String, String>,
    output: Option<OutputType>,
    is_loading: bool,
    error_message: Option<String>,
    registry: Option<AgentRegistry>,
    agent_promise: Option<poll_promise::Promise<Result<OutputType, String>>>,
    registry_promise: Option<poll_promise::Promise<Result<Vec<Agent>, String>>>,
}

impl OpenAgentApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let storage = Storage::new();
        let settings = Settings::load(&storage);
        let agent_loader = AgentLoader::new(storage.clone());
        let installed_agents = agent_loader.load_installed_agents();

        Self {
            view: View::InstalledAgents,
            storage,
            settings,
            agent_loader,
            installed_agents,
            community_agents: Vec::new(),
            selected_agent: None,
            input_values: HashMap::new(),
            search_queries: HashMap::new(),
            output: None,
            is_loading: false,
            error_message: None,
            registry: None,
            agent_promise: None,
            registry_promise: None,
        }
    }

    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(Spacing::XXS);
        
        // App title - uppercase kinetic style
        ui.label(
            egui::RichText::new("OPENAGENT")
                .size(Typography::XL)
                .strong()
                .color(Colors::FOREGROUND)
        );
        
        ui.add_space(Spacing::XXS);
        
        // Minimal horizontal rule
        ui.add_space(Spacing::XXS);
        let rect = ui.available_rect_before_wrap();
        ui.painter().hline(
            rect.x_range(),
            ui.cursor().min.y,
            egui::Stroke::new(2.0, Colors::BORDER),
        );
        ui.add_space(Spacing::XXS);
        
        ui.add_space(Spacing::XXS);

        // Navigation items with full-width highlighting
        let nav_items = [
            (View::InstalledAgents, "INSTALLED"),
            (View::CommunityAgents, "COMMUNITY"),
            (View::CreateAgent, "CREATE"),
            (View::Settings, "SETTINGS"),
        ];

        for (view, label) in nav_items {
            ui.add_space(Spacing::XXXS);
            
            let is_selected = std::mem::discriminant(&self.view) == std::mem::discriminant(&view);
            
            // Full width button with proper colors
            let available_width = ui.available_width();
            
            // Custom button with vertical padding for better appearance
            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(available_width, Typography::SM + Spacing::BASE * 2.0),
                egui::Sense::click()
            );
            
            if ui.is_rect_visible(rect) {
                // Background: accent when selected, surface when hovered
                let bg_color = if is_selected {
                    Colors::ACCENT
                } else if response.hovered() {
                    Colors::SURFACE_ELEVATED
                } else {
                    Color32::TRANSPARENT
                };
                
                // Text: black when selected, off-white otherwise
                let text_color = if is_selected {
                    Colors::ACCENT_FOREGROUND
                } else {
                    Colors::FOREGROUND
                };
                
                ui.painter().rect_filled(rect, egui::Rounding::ZERO, bg_color);
                
                let text_pos = egui::Align2::LEFT_CENTER.pos_in_rect(&rect);
                ui.painter().text(
                    text_pos + egui::vec2(Spacing::SM, 0.0),
                    egui::Align2::LEFT_CENTER,
                    label,
                    egui::FontId::new(Typography::SM, egui::FontFamily::Proportional),
                    text_color,
                );
            }
            
            if response.clicked() {
                self.view = view.clone();
                if matches!(view, View::CommunityAgents) {
                    self.load_community_agents();
                }
            }
            
            ui.add_space(Spacing::XXXS);
        }
    }

    fn render_installed_agents(&mut self, ui: &mut egui::Ui) {
        ui.add_space(Spacing::XXS);
        
        // Section header - uppercase kinetic style
        ui.label(
            egui::RichText::new("INSTALLED AGENTS")
                .size(Typography::XXXL)
                .strong()
                .color(Colors::FOREGROUND)
        );
        
        ui.add_space(Spacing::XXS);
        horizontal_rule(ui, 2.0);
        ui.add_space(Spacing::XS);

        if self.installed_agents.is_empty() {
            ui.label(
                egui::RichText::new("No agents installed yet.")
                    .size(Typography::LG)
                    .color(Colors::MUTED_FOREGROUND)
            );
            ui.add_space(Spacing::XS);
            ui.label(
                egui::RichText::new("Copy YAML files to ~/.openagent/agents/")
                    .size(Typography::BASE)
                    .color(Colors::MUTED_FOREGROUND)
            );
        } else {
            for agent in &self.installed_agents {
                ui.add_space(Spacing::XS);
                
                // Agent card with sharp borders
                card_frame(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(&agent.name)
                                    .size(Typography::XL)
                                    .strong()
                                    .color(Colors::FOREGROUND)
                            );
                            
                            if let Some(desc) = &agent.description {
                                ui.add_space(Spacing::XXXS);
                                ui.label(
                                    egui::RichText::new(desc)
                                        .size(Typography::BASE)
                                        .color(Colors::MUTED_FOREGROUND)
                                );
                            }
                        });
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if secondary_button(ui, "SELECT").clicked() {
                                self.selected_agent = Some(agent.clone());
                                self.input_values.clear();
                                self.output = None;
                                self.error_message = None;
                            }
                        });
                    });
                });
                
                ui.add_space(Spacing::XXS);
            }
        }
    }

    fn render_community_agents(&mut self, ui: &mut egui::Ui) {
        ui.add_space(Spacing::XXS);
        
        ui.label(
            egui::RichText::new("COMMUNITY AGENTS")
                .size(Typography::XXXL)
                .strong()
                .color(Colors::FOREGROUND)
        );
        
        ui.add_space(Spacing::XXS);
        horizontal_rule(ui, 2.0);
        ui.add_space(Spacing::XS);

        if self.community_agents.is_empty() && !self.is_loading {
            ui.label(
                egui::RichText::new("Loading community agents...")
                    .size(Typography::LG)
                    .color(Colors::MUTED_FOREGROUND)
            );
        }

        let mut agent_to_install: Option<Agent> = None;

        for agent in &self.community_agents {
            ui.add_space(Spacing::XS);
            
            card_frame(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&agent.name)
                                .size(Typography::XL)
                                .strong()
                        );
                        ui.add_space(Spacing::XXXS);
                        ui.label(
                            egui::RichText::new(agent.description.as_deref().unwrap_or(""))
                                .size(Typography::BASE)
                                .color(Colors::MUTED_FOREGROUND)
                        );
                    });
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let is_installed = self.installed_agents.iter().any(|a| a.id == agent.id);
                        
                        if is_installed {
                            ui.label(
                                egui::RichText::new("✓ INSTALLED")
                                    .size(Typography::SM)
                                    .strong()
                            );
                        } else if primary_button(ui, "INSTALL").clicked() {
                            agent_to_install = Some(agent.clone());
                        }
                    });
                });
            });
            
            ui.add_space(Spacing::XXS);
        }

        if let Some(agent) = agent_to_install {
            self.install_agent(agent);
        }
    }

    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.add_space(Spacing::XXS);
        
        ui.label(
            egui::RichText::new("SETTINGS")
                .size(Typography::XXXL)
                .strong()
        );
        
        ui.add_space(Spacing::XXS);
        horizontal_rule(ui, 2.0);
        ui.add_space(Spacing::XS);

        // API Key section
        ui.label(
            egui::RichText::new("OPENROUTER API KEY")
                .size(Typography::BASE)
                .strong()
        );
        ui.add_space(Spacing::XXS);
        
        let mut api_key = self.settings.openrouter_api_key.clone();
        let response = ui.add(
            egui::TextEdit::singleline(&mut api_key)
                .desired_width(ui.available_width())
                .font(egui::TextStyle::Monospace)
        );
        if response.changed() {
            self.settings.openrouter_api_key = api_key;
        }

        ui.add_space(Spacing::SM);

        // Registry URL section
        ui.label(
            egui::RichText::new("GITHUB REGISTRY URL")
                .size(Typography::BASE)
                .strong()
        );
        ui.add_space(Spacing::XXS);
        
        let mut registry_url = self.settings.github_registry_url.clone();
        let response = ui.add(
            egui::TextEdit::singleline(&mut registry_url)
                .desired_width(ui.available_width())
                .font(egui::TextStyle::Monospace)
        );
        if response.changed() {
            self.settings.github_registry_url = registry_url;
        }

        ui.add_space(Spacing::LG);
        horizontal_rule(ui, 1.0);
        ui.add_space(Spacing::SM);

        // Actions
        ui.horizontal(|ui| {
            if primary_button(ui, "SAVE SETTINGS").clicked() {
                self.settings.save(&self.storage);
            }
            
            ui.add_space(Spacing::BASE);
            
            if secondary_button(ui, "CLEAR CACHE").clicked() {
                self.storage.clear_cache();
            }
        });
    }

    fn render_create_agent(&mut self, ui: &mut egui::Ui) {
        ui.add_space(Spacing::SM);
        
        ui.label(
            egui::RichText::new("CREATE")
                .size(Typography::XXXXXL)
                .strong()
        );
        ui.label(
            egui::RichText::new("CUSTOM AGENT")
                .size(Typography::XXXXXL)
                .strong()
        );
        
        ui.add_space(Spacing::SM);
        horizontal_rule(ui, 4.0);
        ui.add_space(Spacing::LG);
        
        // Coming soon message in inverted card
        inverted_card_frame(ui, |ui| {
            ui.label(
                egui::RichText::new("COMING SOON")
                    .size(Typography::XXXL)
                    .strong()
            );
            ui.add_space(Spacing::XXS);
            ui.label(
                egui::RichText::new("Agent creation interface in development.")
                    .size(Typography::LG)
            );
        });
    }

    fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        // Back button (check first to avoid borrow issues)
        let back_clicked = ui.horizontal(|ui| {
            ui.add(
                egui::Button::new(
                    egui::RichText::new("< BACK")
                        .size(Typography::SM)
                        .color(Colors::FOREGROUND)
                )
                .fill(Colors::BACKGROUND)
                .stroke(egui::Stroke::new(2.0, Colors::BORDER))
                .rounding(egui::Rounding::same(6.0))
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
        }).inner;
        
        if back_clicked {
            self.selected_agent = None;
            self.output = None;
            self.error_message = None;
            self.input_values.clear();
            return;
        }
        
        if let Some(agent) = &self.selected_agent {
            // Agent name - uppercase kinetic style
            ui.label(
                egui::RichText::new(agent.name.to_uppercase())
                    .size(Typography::XXXXL)
                    .strong()
            );
            
            if let Some(desc) = &agent.description {
                ui.label(
                    egui::RichText::new(desc)
                        .size(Typography::SM)
                        .color(Colors::MUTED_FOREGROUND)
                );
            }
            
            horizontal_rule(ui, 1.0);


            // Input form
            ui.label(
                egui::RichText::new("CONFIGURE")
                    .size(Typography::LG)
                    .strong()
            );
            
            InputRenderer::render(agent, &mut self.input_values, &mut self.search_queries, ui);

            ui.add_space(Spacing::XXS);
            
            if primary_button(ui, "> EXECUTE AGENT").clicked() {
                // Validate required fields
                let mut missing_fields = Vec::new();
                for (name, input) in &agent.inputs {
                    if input.required.unwrap_or(false) {
                        let value = self.input_values.get(name).map(|s| s.trim()).unwrap_or("");
                        if value.is_empty() {
                            missing_fields.push(name.clone());
                        }
                    }
                }
                
                if !missing_fields.is_empty() {
                    self.error_message = Some(format!(
                        "Please fill required fields: {}",
                        missing_fields.join(", ")
                    ));
                } else {
                    self.run_agent();
                }
            }

            ui.add_space(Spacing::XXS);
            horizontal_rule(ui, 1.0);
            ui.add_space(Spacing::XXXS);

            // Output section
            if let Some(output) = &self.output {
                ui.label(
                    egui::RichText::new("OUTPUT")
                        .size(Typography::XL)
                        .strong()
                );
                ui.add_space(Spacing::XXXS);
                
                OutputRenderer::render(output, ui);
            }

            // Error display
            if let Some(error) = &self.error_message {
                ui.add_space(Spacing::XS);
                inverted_card_frame(ui, |ui| {
                    ui.label(
                        egui::RichText::new("ERROR")
                            .size(Typography::BASE)
                            .strong()
                    );
                    ui.add_space(Spacing::XXXS);
                    ui.label(
                        egui::RichText::new(error)
                            .size(Typography::SM)
                    );
                });
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new("SELECT AN AGENT")
                        .size(Typography::XXL)
                        .color(Colors::MUTED_FOREGROUND)
                );
            });
        }
    }

    fn render_status_bar(&self, ui: &mut egui::Ui) {
        // Modern status bar
        egui::Frame::none()
            .fill(Colors::SURFACE)
            .inner_margin(Spacing::XXS)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if self.is_loading {
                        ui.label(
                            egui::RichText::new("● PROCESSING...")
                                .size(Typography::SM)
                                .color(Colors::ACCENT)
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("● READY")
                                .size(Typography::SM)
                                .color(Colors::SUCCESS)
                        );
                    }
                });
            });
    }

    fn load_community_agents(&mut self) {
        if self.registry.is_some() {
            return;
        }

        let registry_url = self.settings.github_registry_url.clone();

        self.is_loading = true;
        let promise = poll_promise::Promise::spawn_thread("fetch_registry", move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async { AgentRegistry::fetch(&registry_url).await })
        });

        self.registry_promise = Some(promise);
    }

    fn install_agent(&mut self, agent: Agent) {
        if let Err(e) = self.agent_loader.install_agent(&agent) {
            self.error_message = Some(format!("Failed to install: {}", e));
        } else {
            self.installed_agents = self.agent_loader.load_installed_agents();
        }
    }

    fn run_agent(&mut self) {
        let agent = match &self.selected_agent {
            Some(a) => a.clone(),
            None => return,
        };

        let api_key = self.settings.openrouter_api_key.clone();
        let input_values = self.input_values.clone();

        self.is_loading = true;
        self.error_message = None;

        let promise = poll_promise::Promise::spawn_thread("run_agent", move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async {
                    let client = OpenRouterClient::new(api_key);
                    client.execute_agent(&agent, &input_values).await
                })
        });

        self.agent_promise = Some(promise);
    }

    fn check_promises(&mut self) {
        if let Some(promise) = &self.agent_promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(output) => {
                        self.output = Some(output.clone());
                        self.is_loading = false;
                    }
                    Err(e) => {
                        self.error_message = Some(e.clone());
                        self.is_loading = false;
                    }
                }
                self.agent_promise = None;
            }
        }

        if let Some(promise) = &self.registry_promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(agents) => {
                        self.community_agents = agents.clone();
                        self.is_loading = false;
                    }
                    Err(e) => {
                        self.error_message = Some(e.clone());
                        self.is_loading = false;
                    }
                }
                self.registry_promise = None;
            }
        }
    }
}

impl eframe::App for OpenAgentApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_promises();
        
        // Full-width loading progress bar at top
        if self.is_loading {
            egui::TopBottomPanel::top("loading_bar")
                .frame(egui::Frame::none())
                .show_separator_line(false)
                .exact_height(3.0)
                .show(ctx, |ui| {
                    let time = ui.input(|i| i.time);
                    let progress = (time * 2.0).fract() as f32; // Continuous animation
                    
                    let rect = ui.max_rect();
                    let bar_width = rect.width() * 0.3; // 30% width moving bar
                    let x_pos = (rect.width() - bar_width) * progress;
                    
                    // Background
                    ui.painter().rect_filled(
                        rect,
                        egui::Rounding::ZERO,
                        Colors::SURFACE,
                    );
                    
                    // Moving progress bar
                    let bar_rect = egui::Rect::from_min_size(
                        egui::pos2(rect.min.x + x_pos, rect.min.y),
                        egui::vec2(bar_width, rect.height()),
                    );
                    ui.painter().rect_filled(
                        bar_rect,
                        egui::Rounding::ZERO,
                        Colors::ACCENT,
                    );
                    
                    // Request repaint for animation
                    ctx.request_repaint();
                });
        }

        // Sidebar - elegant surface panel
        egui::SidePanel::left("sidebar")
            .min_width(220.0)
            .resizable(false)
            .frame(
                egui::Frame::none()
                    .fill(Colors::SURFACE)
                    .stroke(egui::Stroke::new(1.0, Colors::BORDER))
            )
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        // Status bar at bottom
        egui::TopBottomPanel::bottom("status")
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                self.render_status_bar(ui);
            });

        // Main content area
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Colors::BACKGROUND)
                    .inner_margin(Spacing::XS)
            )
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.view {
                    View::InstalledAgents => {
                        if self.selected_agent.is_some() {
                            self.render_main_panel(ui);
                        } else {
                            self.render_installed_agents(ui);
                        }
                    }
                    View::CommunityAgents => self.render_community_agents(ui),
                    View::CreateAgent => self.render_create_agent(ui),
                    View::Settings => self.render_settings(ui),
                }
            });
        });

        ctx.request_repaint();
    }
}
