use crate::components::{sidebar::Sidebar, content::Content, content::Tab};
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    current_tab: Tab,
}

// Message represents a variety of messages that can be processed by the component 
// to trigger some side effect. For example, you may have a Click message which triggers
// an API request or toggles the appearance of a UI component.
pub enum Msg {
    SwitchTab(Tab)
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            current_tab: Tab::Home,
            link: link
        }
    }

    // Update life cycle method is called for each asynchronous message
    // Messages can be triggered by HTML elements listeners or be sent by child components,
    // Agents, Services, or Futures.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchTab(tab) => {
                self.current_tab = tab;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Sidebar: onsignal=self.link.callback(|tab| Msg::SwitchTab(tab))/>
                <Content: tab=self.current_tab.clone()/>
            </>
        }
    }
}