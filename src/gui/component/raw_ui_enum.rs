use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::{component::Select, RcUi};

pub enum Msg {
    Changed(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where
    T: From<usize> + Into<usize> + Clone + PartialEq + 'static,
{
    pub label: String,
    pub items: &'static [&'static str],
    pub value: RcUi<T>,
}

pub struct RawUiEnum<T>
where
    T: From<usize> + Into<usize> + Clone + PartialEq + 'static,
{
    props: Props<T>,
    link: ComponentLink<Self>,
}

impl<T> Component for RawUiEnum<T>
where
    T: From<usize> + Into<usize> + Clone + PartialEq + 'static,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RawUiEnum { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Changed(idx) => {
                *self.props.value.borrow_mut() = T::from(idx);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let current_idx: usize = self.props.value.borrow().clone().into();
        html! {
            <div class="flex gap-1 cursor-default">
                <Select
                    options=self.props.items
                    current_idx=current_idx
                    onselect=self.link.callback(Msg::Changed)
                />
                { &self.props.label }
            </div>
        }
    }
}