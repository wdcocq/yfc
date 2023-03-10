use std::time::Duration;

use yew::{
    html::ChildrenProps,
    prelude::*,
    suspense::{use_future_with_deps, SuspensionResult},
    BaseComponent,
};

#[cfg(target_arch = "wasm32")]
pub async fn render<T>() -> String
where
    T: BaseComponent,
    T::Properties: Default,
{
    yew::Renderer::<T>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();
    yew::platform::time::sleep(Duration::from_millis(200)).await;
    get_output()
}

#[cfg(target_arch = "wasm32")]
pub fn get_output() -> String {
    gloo::utils::document()
        .get_element_by_id("output")
        .unwrap()
        .inner_html()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn render<T>() -> String
where
    T: BaseComponent,
    T::Properties: Default,
{
    yew::ServerRenderer::<T>::new()
        .hydratable(false)
        .render()
        .await
}

#[function_component(Test)]
pub fn test(ChildrenProps { children }: &ChildrenProps) -> Html {
    html! {
        <Suspense fallback={html!{}}>
            {children.clone()}
        </Suspense>
    }
}

#[macro_export]
macro_rules! create_test_comp {
    ($t:ty) => {
        #[function_component(Test)]
        pub fn test() -> Html {
            html! {
                <Suspense>
                    <$t/>
                </Suspense>
            }
        }
    };
}

#[hook]
pub fn use_once<F>(f: F) -> SuspensionResult<()>
where
    F: FnOnce() + 'static,
{
    use_future_with_deps(
        |_| async {
            f();
            yew::platform::time::sleep(Duration::from_millis(100)).await;
        },
        (),
    )?;

    Ok(())
}
