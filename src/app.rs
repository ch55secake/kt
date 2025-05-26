use ratatui::style::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Pods,
    Deployments,
    Services,
}

impl Page {
    pub fn next(&self) -> Self {
        match self {
            Page::Pods => Page::Deployments,
            Page::Deployments => Page::Services,
            Page::Services => Page::Pods,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Page::Pods => Page::Services,
            Page::Deployments => Page::Pods,
            Page::Services => Page::Deployments,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Page::Pods => "Pods",
            Page::Deployments => "Deployments",
            Page::Services => "Services",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Page::Pods => Color::Yellow,
            Page::Deployments => Color::Cyan,
            Page::Services => Color::Green,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pods" => Some(Page::Pods),
            "deployments" => Some(Page::Deployments),
            "services" => Some(Page::Services),
            _ => None,
        }
    }
}

pub struct App {
    pub current_page: Page,
    pub selected_index: usize,
    pub command_mode: bool,
    pub command_input: String,
    pub show_help: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: Page::Pods,
            selected_index: 0,
            command_mode: false,
            command_input: String::new(),
            show_help: false,
        }
    }

    pub fn items(&self) -> Vec<&'static str> {
        match self.current_page {
            Page::Pods => vec![
                "pod-auth-1",
                "pod-ui-2",
                "pod-db-3",
                "pod-redis-4",
                "pod-api-5",
                "pod-metrics-6",
                "pod-logger-7",
                "pod-worker-8",
            ],
            Page::Deployments => vec!["deploy-auth", "deploy-ui", "deploy-redis"],
            Page::Services => vec!["svc-auth", "svc-ui", "svc-db", "svc-monitoring"],
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index + 1 < self.items().len() {
            self.selected_index += 1;
        }
    }

    pub fn reset_selection(&mut self) {
        self.selected_index = 0;
    }
}
