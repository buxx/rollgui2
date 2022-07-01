use crate::event::model::ItemModel;
use macroquad::prelude::*;

use super::gui::{component::ProgressBar, resume::ResumeItem};

#[derive(Debug, Clone)]
pub enum Health {
    Ok,
    Middle,
    Bad,
    Critical,
}

impl Health {
    pub fn from_item(item: &ItemModel) -> Result<Self, String> {
        if let Some(value) = &item.value_str {
            match value.as_str() {
                "Ok" => return Ok(Self::Ok),
                "Moyen" => return Ok(Self::Middle),
                "Mauvais" => return Ok(Self::Bad),
                "Critique" => return Ok(Self::Critical),
                _ => return Err(format!("Unable to understand Health name ! '{}'", value)),
            }
        };

        Err("Unable to understand Health : no value".to_string())
    }

    pub fn item(&self) -> ResumeItem {
        match self {
            Health::Ok => ResumeItem::GoodSmiley,
            Health::Middle => ResumeItem::NeutralSmiley,
            Health::Bad => ResumeItem::BadSmiley,
            Health::Critical => ResumeItem::CriticalSmiley,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CanEat {
    Yes,
    Lower,
    No,
}

impl CanEat {
    pub fn from_item(item: &ItemModel) -> Result<Self, String> {
        if let Some(value_str) = &item.value_str {
            match value_str.as_str() {
                "Oui" => Ok(Self::Yes),
                "Non" => Ok(Self::No),
                "Faible" => Ok(Self::Lower),
                _ => {
                    return Err(format!(
                        "Unable to understand A manger value : '{}'",
                        value_str
                    ))
                }
            }
        } else {
            return Err("Unable to understand A manger : no value".to_string());
        }
    }
}

#[derive(Debug, Clone)]
pub enum CanDrink {
    Yes,
    Lower,
    No,
}

impl CanDrink {
    pub fn from_item(item: &ItemModel) -> Result<Self, String> {
        if let Some(value_str) = &item.value_str {
            match value_str.as_str() {
                "Oui" => Ok(Self::Yes),
                "Non" => Ok(Self::No),
                "Faible" => Ok(Self::Lower),
                _ => {
                    return Err(format!(
                        "Unable to understand A boire value : '{}'",
                        value_str
                    ))
                }
            }
        } else {
            return Err("Unable to understand A boire : no value".to_string());
        }
    }
}

#[derive(Debug)]
pub struct CharacterResume {
    pub health: Health,
    pub action_points: f32,
    pub hungry: ProgressBar,
    pub thirsty: ProgressBar,
    pub tiredness: ProgressBar,
    pub can_drink: CanDrink,
    pub can_eat: CanEat,
    pub follower: i32,
    pub follow: i32,
    pub fighters: i32,
}

impl CharacterResume {
    pub fn from_resume_texts(resume_texts: Vec<ItemModel>) -> Result<Self, String> {
        let mut health: Option<Health> = None;
        let mut action_points: Option<f32> = None;
        let mut hungry: Option<ProgressBar> = None;
        let mut thirsty: Option<ProgressBar> = None;
        let mut tiredness: Option<ProgressBar> = None;
        let mut can_drink: Option<CanDrink> = None;
        let mut can_eat: Option<CanEat> = None;
        let mut follow: Option<i32> = None;
        let mut follower: Option<i32> = None;
        let mut fighters: Option<i32> = None;

        for item in &resume_texts {
            match &item.name.as_str() {
                &"PV" => {
                    health = Some(Health::from_item(item)?);
                }
                &"PA" => {
                    if let Some(value_float) = item.value_float {
                        action_points = Some(value_float);
                    } else {
                        return Err("Unable to understand PA : no value".to_string());
                    }
                }
                &"Faim" => {
                    hungry = Some(
                        ProgressBar::from_item(item)
                            .or_else(|e| Err(format!("Unable to understand Faim : '{}'", e)))?,
                    );
                }
                &"Soif" => {
                    thirsty = Some(
                        ProgressBar::from_item(item)
                            .or_else(|e| Err(format!("Unable to understand Soif : '{}'", e)))?,
                    );
                }
                &"Fatigue" => {
                    tiredness = Some(
                        ProgressBar::from_item(item)
                            .or_else(|e| Err(format!("Unable to understand Fatigue : '{}'", e)))?,
                    );
                }
                &"A boire" => can_drink = Some(CanDrink::from_item(item)?),
                &"A manger" => can_eat = Some(CanEat::from_item(item)?),
                &"Suivis" => {
                    if let Some(value) = item.value_float {
                        follow = Some(value as i32)
                    } else {
                        return Err("Unable to understand Suivis : no value".to_string());
                    }
                }
                &"Suiveurs" => {
                    if let Some(value) = item.value_float {
                        follower = Some(value as i32)
                    } else {
                        return Err("Unable to understand Suivis : no value".to_string());
                    }
                }
                &"Combattants" => {
                    if let Some(value) = item.value_float {
                        fighters = Some(value as i32)
                    } else {
                        return Err("Unable to understand Combattants : no value".to_string());
                    }
                }
                _ => {}
            }
        }

        if let (
            Some(health),
            Some(action_points),
            Some(hungry),
            Some(thirsty),
            Some(tiredness),
            Some(can_drink),
            Some(can_eat),
            Some(follow),
            Some(follower),
            Some(fighters),
        ) = (
            health.clone(),
            action_points.clone(),
            hungry.clone(),
            thirsty.clone(),
            tiredness.clone(),
            can_drink.clone(),
            can_eat.clone(),
            follow,
            follower,
            fighters,
        ) {
            return Ok(Self {
                health,
                action_points,
                hungry,
                thirsty,
                tiredness,
                can_drink,
                can_eat,
                follower,
                follow,
                fighters,
            });
        }

        if health.is_none() {
            return Err("No PV resume found".to_string());
        };

        if action_points.is_none() {
            return Err("No AP resume found".to_string());
        };

        if hungry.is_none() {
            return Err("No hungry resume found".to_string());
        };

        if thirsty.is_none() {
            return Err("No PV thirsty found".to_string());
        };

        if tiredness.is_none() {
            return Err("No PV tiredness found".to_string());
        };

        if can_drink.is_none() {
            return Err("No PV can_drink found".to_string());
        };

        if can_eat.is_none() {
            return Err("No PV can_eat found".to_string());
        };

        if follow.is_none() {
            return Err("No PV follow found".to_string());
        };

        if follower.is_none() {
            return Err("No PV follower found".to_string());
        };

        return Err("No PV resume found".to_string());
    }
}
