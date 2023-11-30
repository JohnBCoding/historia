use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let results_state = use_state(|| Vec::<HashMap<String, String>>::new());
    let current_search_state = use_state(|| "".to_string());
    let current_page_state = use_state(|| 0);

    let on_search = {
        let results_state = results_state.clone();
        let current_search_state = current_search_state.clone();
        let current_page_state = current_page_state.clone();
        Callback::from(move |search_value: String| {
            let results_state = results_state.clone();
            let current_search_state = current_search_state.clone();
            let current_page_state = current_page_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let api_key = std::env!("API_KEY");
                let search_uri = format!(
                    "https://api.api-ninjas.com/v1/historicalevents?text={}&offset={}",
                    search_value, *current_page_state
                );

                let search_result = Request::get(&search_uri)
                    .header("X-Api-Key", api_key)
                    .send()
                    .await;

                if let Ok(res) = search_result {
                    if res.ok() {
                        current_search_state.set(search_value);
                        let results: Vec<HashMap<String, String>> = res.json().await.unwrap();
                        results_state.set(results);
                    }
                }
            });
        })
    };

    let on_change_page = {
        let results_state = results_state.clone();
        let current_search_state = current_search_state.clone();
        let current_page_state = current_page_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let name = event.target_unchecked_into::<HtmlButtonElement>().name();
            let page_amt = match name.as_str() {
                "back" => {
                    if *current_page_state > 0 {
                        -10
                    } else {
                        0
                    }
                }
                "next" => 10,
                _ => 0,
            };

            let results_state = results_state.clone();
            let current_search_state = current_search_state.clone();
            let current_page_state = current_page_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let api_key = std::env!("API_KEY");
                let search_value = current_search_state.deref().clone();
                let search_uri = format!(
                    "https://api.api-ninjas.com/v1/historicalevents?text={}&offset={}",
                    search_value,
                    *current_page_state + page_amt
                );
                let search_result = Request::get(&search_uri)
                    .header("X-Api-Key", api_key)
                    .send()
                    .await;

                if let Ok(res) = search_result {
                    if res.ok() {
                        let results: Vec<HashMap<String, String>> = res.json().await.unwrap();
                        results_state.set(results);
                        current_page_state.set(*current_page_state + page_amt);
                    }
                }
            });
        })
    };

    let result_html = results_state
        .iter()
        .map(|result| {
            html! {<Result result={result.clone()} />}
        })
        .collect::<Html>();

    html! {
        <main class="main-container col expand-x expand-y fade-in">
            <h1>{"Historia"}</h1>
            <h2>{"Search, explore, learn."}</h2>
            <Search handle_on_search={&on_search}/>
            if !results_state.is_empty() {
                <div class="col expand-y scroll">
                    {result_html}
                </div>
                <div class="row expand-x flex-end-y">
                    <button name="back" onclick={&on_change_page}>{"BACK"}</button>
                    <button class="flex-end-x" name="next" onclick={&on_change_page}>{"NEXT"}</button>
                </div>
            }
        </main>
    }
}
