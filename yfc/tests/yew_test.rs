mod common;

use common::render;
#[cfg(not(target_arch = "wasm32"))]
use tokio::test;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::prelude::*;
use yfc::prelude::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
async fn test_component() {
    #[derive(Model, Debug, PartialEq)]
    struct Model {
        id: u32,
    }

    #[derive(Properties, PartialEq)]
    struct Props {
        form: Form<Model>,
    }

    #[function_component(FormComp)]
    fn form_comp(Props { form }: &Props) -> Html {
        html! {
            <p>{&form.state().id}</p>
        }
    }

    #[function_component(Comp)]
    fn comp() -> Html {
        let form = use_form(|| Model { id: 1 });

        html! {
            <FormComp {form}/>
        }
    }

    let html = render::<Comp>().await;
    assert_eq!(html, "<p>1</p>");
}

#[test]
async fn test_field_component() {
    #[derive(Model, Debug, PartialEq)]
    struct Model {
        id: u32,
    }

    #[derive(Properties, PartialEq)]
    struct Props {
        form: Form<u32>,
    }

    #[function_component(FormComp)]
    fn form_comp(Props { form }: &Props) -> Html {
        html! {
            <p>{form.state().value()}</p>
        }
    }

    #[function_component(Comp)]
    fn comp() -> Html {
        let form = use_form(|| Model { id: 1 });

        html! {
            <FormComp form={form.id_form()}/>
        }
    }

    let html = render::<Comp>().await;
    assert_eq!(html, "<p>1</p>");
}
