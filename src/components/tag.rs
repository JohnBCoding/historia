use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub text: String,
    pub handle_on_click: Callback<String>,
}

#[function_component(Tag)]
pub fn tag(props: &Props) -> Html {
    let on_click = {
        let handle_on_click = props.handle_on_click.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let value = event
                .target_unchecked_into::<HtmlButtonElement>()
                .text_content()
                .unwrap();
            handle_on_click.emit(value);
        })
    };
    html! {
        <button class="tag expand-x" onclick={&on_click}>{&props.text}</button>
    }
}
