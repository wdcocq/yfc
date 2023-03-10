mod common;

#[cfg(target_arch = "wasm32")]
use std::time::Duration;

use common::render;
// use validator::Validate;
#[cfg(not(target_arch = "wasm32"))]
use tokio::test;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;
#[cfg(target_arch = "wasm32")]
use web_sys::InputEventInit;
#[cfg(target_arch = "wasm32")]
use yew::platform::time::sleep;
use yew::prelude::*;
use yfc::{components::*, prelude::*};

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
async fn test_checkbox() {
    #[derive(Clone, PartialEq, Model)]
    struct Model {
        check: bool,
    }

    #[function_component(Test)]
    pub fn test() -> Html {
        let form = use_form(|| Model { check: true });

        html! {
            <>
                <CheckBox form={form.check_form()}/>
                <div id="value">{form.state().check()}</div>
                <div id="model">{form.model().check}</div>
            </>
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        render::<Test>().await;
        let output = gloo::utils::document().get_element_by_id("output").unwrap();
        let elem = output
            .first_element_child()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>();
        let value = gloo::utils::document().get_element_by_id("value").unwrap();
        let model = gloo::utils::document().get_element_by_id("model").unwrap();

        assert_eq!(elem.checked(), true);
        assert_eq!(elem.get_attribute("type").unwrap(), "checkbox");
        assert_eq!(elem.get_attribute("value").unwrap(), "true");
        assert_eq!(value.inner_html(), "true");
        assert_eq!(model.inner_html(), "true");

        elem.click();
        sleep(Duration::from_millis(100)).await;

        assert_eq!(elem.checked(), false);
        assert_eq!(elem.get_attribute("value").unwrap(), "false");
        assert_eq!(value.inner_html(), "false");
        assert_eq!(model.inner_html(), "false");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let html = render::<Test>().await;
        assert_eq!(
            html,
            r#"<input value="true" checked type="checkbox"><div id="value">true</div><div id="model">true</div>"#
        );
    }
}

#[test]
async fn test_input() {
    #[derive(Clone, PartialEq, Model)]
    struct Model {
        input: String,
    }

    #[function_component(Test)]
    pub fn test() -> Html {
        let form = use_form(|| Model {
            input: "test".into(),
        });

        html! {
            <>
                <Input<String> form={form.input_form()}/>
                <div id="value">{form.state().input()}</div>
                <div id="model">{&form.model().input}</div>
            </>
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        render::<Test>().await;
        let output = gloo::utils::document().get_element_by_id("output").unwrap();
        let elem = output
            .first_element_child()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>();
        let value = gloo::utils::document().get_element_by_id("value").unwrap();
        let model = gloo::utils::document().get_element_by_id("model").unwrap();

        assert_eq!(elem.get_attribute("type").unwrap(), "text");
        assert_eq!(elem.value(), "test");
        assert_eq!(value.inner_html(), "test");
        assert_eq!(model.inner_html(), "test");

        elem.set_value("output");
        elem.dispatch_event(
            &InputEvent::new_with_event_init_dict(
                "input",
                InputEventInit::new().data(Some("output")),
            )
            .unwrap(),
        )
        .unwrap();
        sleep(Duration::from_millis(100)).await;

        assert_eq!(value.inner_html(), "output");
        assert_eq!(model.inner_html(), "output");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let html = render::<Test>().await;
        assert_eq!(
            html,
            r#"<input value="test" type="text" autocomplete="off" placeholder=""><div id="value">test</div><div id="model">test</div>"#
        );
    }
}
