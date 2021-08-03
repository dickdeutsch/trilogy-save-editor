use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{
    gui::{components::Table, RcUi},
    save_data::mass_effect_2::player::Power as Me2Power,
};

#[derive(Clone)]
pub enum BonusPowerType {
    Me2(RcUi<Vec<RcUi<Me2Power>>>),
    Me3(RcUi<Vec<RcUi<Me2Power>>>),
}

impl PartialEq for BonusPowerType {
    fn eq(&self, other: &BonusPowerType) -> bool {
        match self {
            BonusPowerType::Me2(powers) => match other {
                BonusPowerType::Me2(other) => powers == other,
                _ => false,
            },
            BonusPowerType::Me3(powers) => match other {
                BonusPowerType::Me3(other) => powers == other,
                _ => false,
            },
        }
    }
}

pub enum Msg {
    ToggleBonusPower(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub power_list: &'static [(&'static str, &'static str)], // TODO: add name
    pub powers: BonusPowerType,
}

pub struct BonusPowers {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for BonusPowers {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        BonusPowers { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleBonusPower(power_class_name) => {
                match self.props.powers {
                    BonusPowerType::Me2(ref mut powers) => {
                        let idx = powers.borrow().iter().enumerate().find_map(|(i, power)| {
                            unicase::eq(&power_class_name, &power.borrow().power_class_name())
                                .then(|| i)
                        });

                        if let Some(idx) = idx {
                            powers.borrow_mut().remove(idx);
                        } else {
                            let mut power = Me2Power::default();
                            *power.power_class_name.borrow_mut() = power_class_name;
                            powers.borrow_mut().push(RcUi::new(power));
                        }
                    }
                    BonusPowerType::Me3(ref mut powers) => {
                        let idx = powers.borrow().iter().enumerate().find_map(|(i, power)| {
                            unicase::eq(&power_class_name, &power.borrow().power_class_name())
                                .then(|| i)
                        });

                        if let Some(idx) = idx {
                            powers.borrow_mut().remove(idx);
                        } else {
                            let mut power = Me2Power::default();
                            *power.power_class_name.borrow_mut() = power_class_name;
                            powers.borrow_mut().push(RcUi::new(power));
                        }
                    }
                }

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Props { power_list, powers } = &self.props;

        let selectables = power_list.iter().map(|&(power_class_name, power_name)| {
            let selected = match powers {
                BonusPowerType::Me2(powers) => powers.borrow()
                .iter()
                .any(|power| unicase::eq(power_class_name, &power.borrow().power_class_name())),
                BonusPowerType::Me3(powers) => powers.borrow()
                .iter()
                .any(|power| unicase::eq(power_class_name, &power.borrow().power_class_name())),
            };

            html_nested! {
                <button
                    class=classes![
                        "rounded-none",
                        "hover:bg-theme-hover",
                        "active:bg-theme-active",
                        "px-1",
                        "w-full",
                        "text-left",
                        selected.then(|| "bg-theme-bg"),
                    ]
                    onclick=self.link.callback(move |_| Msg::ToggleBonusPower(power_class_name.to_owned()))
                >
                    {power_name}
                </button>
            }
        });

        html! {
            <Table title=String::from("Bonus Powers //TODO: (?)")>
                { for selectables }
            </Table>
        }
    }
}