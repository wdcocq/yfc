mod common;

use common::{render, use_once};
// use validator::Validate;
#[cfg(not(target_arch = "wasm32"))]
use tokio::test;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::prelude::*;
use yfc::prelude::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
async fn test_value() {
    #[derive(Model, Debug, PartialEq, Eq)]
    struct Model {
        value: u32,
    }

    #[function_component(Component)]
    fn component() -> HtmlResult {
        let form = use_form(|| Model { value: 0 });
        use_once({
            let form = form.clone();
            move || {
                (|form: Form<Model>| {
                    form.state_mut().set_value("42");
                })(form);
            }
        })?;
        Ok(html! {
            <>
                <p>{form.state().value()}</p>
                <p>{form.model().value}</p>
            </>
        })
    }

    create_test_comp!(Component);

    assert_eq!(render::<Test>().await, "<p>42</p><p>42</p>");
}

#[test]
async fn test_model_value() {
    #[derive(Model, Debug, PartialEq, Eq)]
    struct Parent {
        #[yfc(model)]
        child: Child,
    }

    #[derive(Model, Debug, PartialEq, Eq)]
    struct Child {
        value: u32,
    }

    #[function_component(Component)]
    fn component() -> HtmlResult {
        let form = use_form(|| Parent {
            child: Child { value: 0 },
        });
        use_once({
            let form = form.clone();
            move || form.state_mut().child().set_value("42")
        })?;

        Ok(html! {
            <>
                <p>{form.state().child().value()}</p>
                <p>{form.model().child.value}</p>
            </>
        })
    }

    create_test_comp!(Component);

    assert_eq!(render::<Test>().await, "<p>42</p><p>42</p>");
}

#[test]
async fn test_value_list() {
    #[derive(Model, Debug, PartialEq, Eq)]
    struct Model {
        #[yfc(list)]
        values: Vec<u32>,
    }

    #[function_component(Component)]
    fn component() -> HtmlResult {
        let form = use_form(|| Model {
            values: vec![0, 1, 2],
        });

        use_once({
            let form = form.clone();
            move || {
                form.state_mut().set_values(0, "42");
                form.state_mut().values().remove(1);
                form.state_mut().values().insert(1, 43);
                form.state_mut().values().remove(2);
                form.state_mut().values().push(44);
            }
        })?;

        Ok(html! {
            <>
                <p>{form.state().values(0)}</p>
                <p>{form.state().values(1)}</p>
                <p>{form.state().values(2)}</p>
            </>
        })
    }

    create_test_comp!(Component);

    assert_eq!(render::<Test>().await, "<p>42</p><p>43</p><p>44</p>");
}

#[test]
async fn test_option_value() {
    #[derive(Model, Debug, PartialEq, Eq)]
    struct Model {
        value: Option<u32>,
    }

    #[function_component(Component)]
    fn component() -> HtmlResult {
        let form = use_form(|| Model { value: Some(1) });

        use_once({
            let form = form.clone();
            move || {
                form.state_mut().value().set("2");
            }
        })?;

        Ok(html! {
            <p>{form.state().value()}</p>
        })
    }

    create_test_comp!(Component);

    assert_eq!(render::<Test>().await, "<p>2</p>");
}

#[test]
async fn test_option_value_none() {
    #[derive(Model, Debug, PartialEq, Eq)]
    struct Model {
        value: Option<u32>,
    }

    #[function_component(Component)]
    fn component() -> HtmlResult {
        let form = use_form(|| Model { value: Some(1) });

        use_once({
            let form = form.clone();
            move || {
                form.state_mut().value().set("");
            }
        })?;

        Ok(html! {
            <p>{form.state().value()}</p>
        })
    }

    create_test_comp!(Component);

    assert_eq!(render::<Test>().await, "<p></p>");
}
