use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WizardTower {
    wizard_quest_step: usize,
    crumbs_laid_out: usize,
}
