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
            <body ng-app="Connect4App">
                // hackerman styling
                <style>{"
                    body,h1,h2,h3,h4,h5 {font-family: Poppins, sans-serif}
                    body {font-size:16px;}
                    .w3-half img{margin-bottom:-6px;margin-top:16px;opacity:0.8;cursor:pointer}
                    .w3-half img:hover{opacity:1}
                    
                    table, th , td  {
                        border: 1px solid grey;
                        border-collapse: collapse;
                        padding: 5px;
                    }
                    table tr:nth-child(odd) {
                        background-color: #f1f1f1;
                    }
                    table tr:nth-child(even) {
                        background-color: #ffffff;
                    }
                "}</style>
                <Sidebar: onsignal=self.link.callback(|tab| Msg::SwitchTab(tab))/>
                <Content: tab=self.current_tab.clone()/>
            </body>
        }
    }
}