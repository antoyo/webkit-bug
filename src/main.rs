extern crate gtk;
extern crate webkit2gtk;

use gtk::{Cast, ContainerExt, Inhibit, WidgetExt, Window, WindowType};
use gtk::prelude::ToVariant;
use webkit2gtk::{NavigationPolicyDecision, UserContentManager, WebContext, WebView, WebViewExt};
use webkit2gtk::PolicyDecisionType::NavigationAction;

fn main() {
    gtk::init().unwrap();

    let context = WebContext::get_default().unwrap();
    context.set_web_extensions_directory("./webkit-bug-extension/target/debug");
    context.set_web_extensions_initialization_user_data(&0.to_variant());

    let window = Window::new(WindowType::Toplevel);
    let webview = WebView::new_with_context_and_user_content_manager(&context, &UserContentManager::new());
    webview.load_html(include_str!("../tests/index.html"), None);
    window.add(&webview);

    webview.connect_decide_policy(|_, policy_decision, policy_decision_type| {
        if policy_decision_type == NavigationAction {
            let navigation_policy_decision = policy_decision.clone().downcast::<NavigationPolicyDecision>()
                .unwrap();
            let navigation_action = navigation_policy_decision.get_navigation_action().unwrap();
            println!("{}", navigation_action.get_modifiers());
        }
        true
    });

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
