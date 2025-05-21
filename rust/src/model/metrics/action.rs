use crate::model::domain::{Cost, Period, Purchase, Sale};

#[derive(Debug, Clone)]
pub enum Action {
    ProcessPeriod,
    ProcessYear,
    Purchase(Purchase),
    Sale(Sale),
    Cost(Cost),
    ClosePeriod(Period),
    OpenPeriod(Period),
    PeriodReport(Period),
    YearReport(Period),
}

impl Action {
    pub fn code(&self) -> &'static str {
        match self {
            Action::ProcessPeriod => { "PROCESS_PERIOD" }
            Action::ProcessYear => { "PROCESS_YEAR" }
            Action::Purchase(_) => { "PURCHASE" }
            Action::Sale(_) => { "SALE" }
            Action::Cost(_) => { "COST" }
            Action::ClosePeriod(_) => { "CLOSE_PERIOD" }
            Action::OpenPeriod(_) => { "OPEN_PERIOD" }
            Action::PeriodReport(_) => { "PERIOD_REPORT" }
            Action::YearReport(_) => { "YEAR_REPORT" }
        }
    }
}
