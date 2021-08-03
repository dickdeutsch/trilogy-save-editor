use std::cell::Ref;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{
    gui::{
        components::{
            shared::{BonusPowerType, BonusPowers},
            Select, Table,
        },
        raw_ui::RawUi,
        RcUi,
    },
    save_data::{
        mass_effect_2::{player::Player, Difficulty},
        shared::{
            player::{Notoriety, Origin},
            plot::PlotTable,
            EndGameState,
        },
    },
};

use super::Me2Type;

#[derive(Clone, RawUi)]
enum Me2Class {
    Adept,
    Engineer,
    Infiltrator,
    Sentinel,
    Soldier,
    Vanguard,
}

impl Me2Class {
    fn names() -> &'static [&'static str] {
        &[
            "SFXGame.SFXPawn_PlayerAdept",
            "SFXGame.SFXPawn_PlayerEngineer",
            "SFXGame.SFXPawn_PlayerInfiltrator",
            "SFXGame.SFXPawn_PlayerSentinel",
            "SFXGame.SFXPawn_PlayerSoldier",
            "SFXGame.SFXPawn_PlayerVanguard",
        ]
    }
}

pub enum Msg {
    Gender(usize),
    Origin(usize),
    Notoriety(usize),
    PlayerClass(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub save_game: Me2Type,
}

pub struct Me2General {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Me2General {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Me2General { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let (mut player, mut me1_plot, mut plot) = match self.props.save_game {
            Me2Type::Vanilla(ref me2) => {
                let me2 = me2.borrow();
                (RcUi::clone(&me2.player), RcUi::clone(&me2.me1_plot), RcUi::clone(&me2.plot))
            }
            Me2Type::Legendary(ref me2) => {
                let me2 = me2.borrow();
                (RcUi::clone(&me2.player), RcUi::clone(&me2.me1_plot), RcUi::clone(&me2.plot))
            }
        };
        let (mut player, mut me1_plot, mut plot) =
            (player.borrow_mut(), me1_plot.borrow_mut(), plot.borrow_mut());

        match msg {
            Msg::Gender(gender) => {
                let gender = gender != 0;

                // Player
                *player.is_female_mut() = gender;

                // Plot
                // ME1
                if let Some(mut is_female) = me1_plot.booleans_mut().get_mut(4639) {
                    *is_female = gender;
                }
                // ME2
                if let Some(mut is_female) = plot.booleans_mut().get_mut(66) {
                    *is_female = gender;
                }

                false
            }
            Msg::Origin(origin_idx) => {
                // Player
                *player.origin_mut() = Origin::from(origin_idx);

                // ME1 imported
                match *player.origin() {
                    Origin::None => {}
                    Origin::Spacer => {
                        if let Some(mut spacer) = plot.booleans_mut().get_mut(1533) {
                            *spacer = true;
                        }
                        if let Some(mut colonist) = plot.booleans_mut().get_mut(1535) {
                            *colonist = false;
                        }
                        if let Some(mut eathborn) = plot.booleans_mut().get_mut(1534) {
                            *eathborn = false;
                        }
                    }
                    Origin::Colonist => {
                        if let Some(mut spacer) = plot.booleans_mut().get_mut(1533) {
                            *spacer = false;
                        }
                        if let Some(mut colonist) = plot.booleans_mut().get_mut(1535) {
                            *colonist = true;
                        }
                        if let Some(mut eathborn) = plot.booleans_mut().get_mut(1534) {
                            *eathborn = false;
                        }
                    }
                    Origin::Earthborn => {
                        if let Some(mut spacer) = plot.booleans_mut().get_mut(1533) {
                            *spacer = false;
                        }
                        if let Some(mut colonist) = plot.booleans_mut().get_mut(1535) {
                            *colonist = false;
                        }
                        if let Some(mut eathborn) = plot.booleans_mut().get_mut(1534) {
                            *eathborn = true;
                        }
                    }
                }

                // ME1 plot
                if let Some(me1_origin) = me1_plot.integers_mut().get_mut(1) {
                    *me1_origin.borrow_mut() = origin_idx as i32;
                }

                false
            }
            Msg::Notoriety(notoriety_idx) => {
                // Player
                *player.notoriety_mut() = Notoriety::from(notoriety_idx);

                // ME1 imported
                match *player.notoriety() {
                    Notoriety::None => {}
                    Notoriety::Survivor => {
                        if let Some(mut survivor) = plot.booleans_mut().get_mut(1537) {
                            *survivor = true;
                        }
                        if let Some(mut war_hero) = plot.booleans_mut().get_mut(1538) {
                            *war_hero = false;
                        }
                        if let Some(mut ruthless) = plot.booleans_mut().get_mut(1539) {
                            *ruthless = false;
                        }
                    }
                    Notoriety::Warhero => {
                        if let Some(mut survivor) = plot.booleans_mut().get_mut(1537) {
                            *survivor = false;
                        }
                        if let Some(mut war_hero) = plot.booleans_mut().get_mut(1538) {
                            *war_hero = true;
                        }
                        if let Some(mut ruthless) = plot.booleans_mut().get_mut(1539) {
                            *ruthless = false;
                        }
                    }
                    Notoriety::Ruthless => {
                        if let Some(mut survivor) = plot.booleans_mut().get_mut(1537) {
                            *survivor = false;
                        }
                        if let Some(mut war_hero) = plot.booleans_mut().get_mut(1538) {
                            *war_hero = false;
                        }
                        if let Some(mut ruthless) = plot.booleans_mut().get_mut(1539) {
                            *ruthless = true;
                        }
                    }
                }

                // ME1 plot
                if let Some(me1_notoriety) = me1_plot.integers_mut().get_mut(2) {
                    *me1_notoriety.borrow_mut() = notoriety_idx as i32;
                }

                false
            }
            Msg::PlayerClass(class_idx) => {
                *player.class_name_mut() = Me2Class::names()[class_idx].to_owned();
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let (difficulty, end_game_state, player, plot) = match self.props.save_game {
            Me2Type::Vanilla(ref me2) => {
                let me2 = me2.borrow();
                (
                    RcUi::clone(&me2.difficulty),
                    RcUi::clone(&me2.end_game_state),
                    RcUi::clone(&me2.player),
                    RcUi::clone(&me2.plot),
                )
            }
            Me2Type::Legendary(ref me2) => {
                let me2 = me2.borrow();
                (
                    RcUi::clone(&me2.difficulty),
                    RcUi::clone(&me2.end_game_state),
                    RcUi::clone(&me2.player),
                    RcUi::clone(&me2.plot),
                )
            }
        };

        html! {
            <div class="flex flex-row divide-solid divide-x divide-default-border">
                <div class="flex-1 pr-1 flex flex-col gap-1">
                    { self.role_play(player.borrow()) }
                    { self.morality(plot.borrow()) }
                    { self.gameplay(player.borrow()) }
                    { self.resources(player.borrow()) }
                </div>
                <div class="flex-1 pl-1 flex flex-col gap-1">
                    { self.general(difficulty, end_game_state) }
                    { self.bonus_powers(player.borrow()) }
                </div>
            </div>
        }
    }
}

impl Me2General {
    fn role_play(&self, player: Ref<'_, Player>) -> Html {
        let genders: &'static [&'static str] = &["Male", "Female"];
        html! {
            <Table title=String::from("Role-Play")>
                { player.first_name.view("Name") }
                <div class="flex gap-1 cursor-default">
                    <Select
                        options=genders
                        current_idx=*player.is_female() as usize
                        onselect=self.link.callback(Msg::Gender)
                    />
                    {"Gender //TODO: (?)"}
                </div>
                <div class="flex gap-1 cursor-default">
                    <Select
                        options=Origin::variants()
                        current_idx=player.origin().clone() as usize
                        onselect=self.link.callback(Msg::Origin)
                    />
                    {"Origin"}
                </div>
                <div class="flex gap-1 cursor-default">
                    <Select
                        options=Notoriety::variants()
                        current_idx=player.notoriety().clone() as usize
                        onselect=self.link.callback(Msg::Notoriety)
                    />
                    {"Notoriety"}
                </div>
                { player.face_code.view("Identity Code //TODO: (?)") }
            </Table>
        }
    }

    fn morality(&self, plot: Ref<'_, PlotTable>) -> Html {
        html! {
            <Table title=String::from("Morality")>
                { plot.integers().get(2).map(|paragon| paragon.view("Paragon")).unwrap_or_default() }
                { plot.integers().get(3).map(|renegade| renegade.view("Renegade")).unwrap_or_default() }
            </Table>
        }
    }

    fn gameplay(&self, player: Ref<'_, Player>) -> Html {
        let Player { level, current_xp, talent_points, credits, medigel, .. } = &*player;

        let class_idx = Me2Class::names()
            .iter()
            .enumerate()
            .find_map(|(i, &name)| unicase::eq(name, &player.class_name()).then(|| i))
            .unwrap_or_default();

        html! {
            <Table title=String::from("Gameplay")>
                <div class="flex gap-1 cursor-default">
                    <Select
                        options=Me2Class::variants()
                        current_idx=class_idx
                        onselect=self.link.callback(Msg::PlayerClass)
                    />
                    {"Class"}
                </div>
                { level.view("Level") }
                { current_xp.view("Current XP") }
                { talent_points.view("Talent Points") }
                { credits.view("Credits") }
                { medigel.view("Medigel") }
            </Table>
        }
    }

    fn resources(&self, player: Ref<'_, Player>) -> Html {
        let Player { eezo, iridium, palladium, platinum, probes, current_fuel, .. } = &*player;

        html! {
            <Table title=String::from("Resources")>
                { eezo.view("Eezo") }
                { iridium.view("Iridium") }
                { palladium.view("Palladium") }
                { platinum.view("Platinum") }
                { probes.view("Probes") }
                { current_fuel.view("Current Fuel") }
            </Table>
        }
    }

    fn general(&self, difficulty: RcUi<Difficulty>, end_game_state: RcUi<EndGameState>) -> Html {
        html! {
            <Table title=String::from("General")>
                { difficulty.view("Difficulty") }
                { end_game_state.view("End Game Stage") }
            </Table>
        }
    }

    fn bonus_powers(&self, player: Ref<'_, Player>) -> Html {
        let power_list: &'static [(&'static str, &'static str)] = &[
            ("SFXGameContent_Powers.SFXPower_Crush_Player", "Slam"),
            ("SFXGameContent_Powers.SFXPower_Barrier_Player", "Barrier"),
            ("SFXGameContent_Powers.SFXPower_WarpAmmo_Player", "Warp Ammo"),
            ("SFXGameContent_Powers.SFXPower_Fortification_Player", "Fortification"),
            ("SFXGameContent_Powers.SFXPower_ArmorPiercingAmmo_Player", "Armor Piercing Ammo"),
            ("SFXGameContent_Powers.SFXPower_NeuralShock_Player", "Neural Shock"),
            ("SFXGameContent_Powers.SFXPower_ShieldJack_Player", "Energy Drain"),
            ("SFXGameContent_Powers.SFXPower_Reave_Player", "Reave"),
            ("SFXGameContent_Powers.SFXPower_Dominate_Player", "Dominate"),
            ("SFXGameContent_Powers.SFXPower_AntiOrganicAmmo_Player", "Shredder Ammo"),
            ("SFXGameContent_Powers.SFXPower_GethShieldBoost_Player", "Geth Shield Boost"),
            ("SFXGameContentDLC_HEN_VT.SFXPower_ZaeedUnique_Player", "Inferno Grenade"),
            ("SFXGameContentKasumi.SFXPower_KasumiUnique_Player", "Flashbang Grenade"),
            ("SFXGameContentLiara.SFXPower_StasisNew", "Stasis"),
        ];

        html! {
            <BonusPowers power_list=power_list powers=BonusPowerType::Me2(RcUi::clone(&player.powers)) />
        }
    }
}