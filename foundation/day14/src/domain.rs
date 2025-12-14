#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Click,
    Purchase,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub user_id: u64,
    pub action: Action,
    pub value: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Report {
    pub total_events: usize,
    pub total_clicks: u64,
    pub total_revenue_cents: u64,
    pub unique_users: usize,
}
