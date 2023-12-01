use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub handle_on_search: Callback<String>,
}

#[function_component(Search)]
pub fn search(props: &Props) -> Html {
    let search_ref = use_node_ref();

    let on_search = {
        let handle_on_search = props.handle_on_search.clone();
        Callback::from(move |event: KeyboardEvent| {
            event.prevent_default();

            let element = event.target_unchecked_into::<HtmlInputElement>();
            if event.key() == "Enter" {
                let value = element.value();
                element.set_value("");
                handle_on_search.emit(format!("text={}", value));
            }
        })
    };

    let on_click = {
        let handle_on_search = props.handle_on_search.clone();
        let search_ref = search_ref.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let value = search_ref.cast::<HtmlInputElement>().unwrap().value();
            handle_on_search.emit(value);
        })
    };

    html! {
        <div class="row expand-x">
            <input class="expand-x" id="search" placeholder="The Roman Empire perhaps?" onkeyup={&on_search} ref={search_ref}/>
            <button onclick={&on_click}>{"Go"}</button>
        </div>
    }
}
