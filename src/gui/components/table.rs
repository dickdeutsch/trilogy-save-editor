use yew::prelude::*;
use yewtil::NeqAssign;

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: Option<String>,
    pub children: Children,
    #[prop_or(true)]
    pub opened: bool,
}

pub struct Table {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Table {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Table { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.props.opened = !self.props.opened;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let opened = self.props.title.is_none() || self.props.opened;

        let title = self
            .props
            .title
            .as_ref()
            .map(|title| {
                let chevron = if opened { "table-chevron-down" } else { "table-chevron-right" };

                html! {
                    <div class="flex-1 bg-table-odd p-px">
                        <button
                            class=classes![
                                "rounded-none",
                                "hover:bg-theme-hover",
                                "active:bg-theme-active",
                                "px-1",
                                "w-full",
                                "text-left",
                                "pl-6",
                                chevron,
                            ]
                            onclick=self.link.callback(|_| Msg::Toggle)
                        >
                            {title}
                        </button>
                    </div>
                }
            })
            .unwrap_or_default();

        let rows = opened
            .then(|| {
                self.props
                    .children
                    .iter()
                    .map(|child| {
                        html_nested! {
                            <div class=classes![
                                "table-row",
                                self.props.title.is_some().then(|| "!pl-6"),
                            ]>
                                {child}
                            </div>
                        }
                    })
                    .collect::<Html>()
            })
            .unwrap_or_default();

        html! {
            <div class="flex flex-col border border-default-border">
                {title}
                {rows}
            </div>
        }
    }
}