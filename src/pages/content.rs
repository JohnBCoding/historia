use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let results_state = use_state(|| Vec::<HashMap<String, String>>::new());
    let current_search_state = use_state(|| "".to_string());
    let current_page_state = use_state(|| 0);

    // Handle new search
    let on_search = {
        let results_state = results_state.clone();
        let current_search_state = current_search_state.clone();
        let current_page_state = current_page_state.clone();
        Callback::from(move |search_value: String| {
            let results_state = results_state.clone();
            let current_search_state = current_search_state.clone();
            let current_page_state = current_page_state.clone();
            get_results(
                search_value,
                current_search_state,
                current_page_state,
                results_state,
            );
        })
    };

    // Handle getting new results when page is changed
    let on_change_page = {
        let results_state = results_state.clone();
        let current_search_state = current_search_state.clone();
        let current_page_state = current_page_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            // Need to check what button was pressed to determine to go backwards or forward a page
            // API sends results in 10s, so need to change the state by 10s as well.
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
                    "https://api.api-ninjas.com/v1/historicalevents?{}&offset={}",
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

    // Handle adding tag to current search state
    let on_update_search = {
        let results_state = results_state.clone();
        let current_search_state = current_search_state.clone();
        let current_page_state = current_page_state.clone();
        Callback::from(move |(key, value)| {
            let current_search = current_search_state.deref().clone();
            let new_search = {
                if !current_search.contains(&key) {
                    format!("{}&{}={}", current_search, key, value)
                } else {
                    current_search
                }
            };

            let results_state = results_state.clone();
            let current_search_state = current_search_state.clone();
            let current_page_state = current_page_state.clone();
            get_results(
                new_search,
                current_search_state,
                current_page_state,
                results_state,
            );
        })
    };

    // Handle removing tag from current search state
    let on_remove_search = {
        let results_state = results_state.clone();
        let current_search_state = current_search_state.clone();
        let current_page_state = current_page_state.clone();
        Callback::from(move |tag_name: String| {
            let current_search = current_search_state.deref().clone();
            let new_search = { current_search.replace(&tag_name, "") };

            let results_state = results_state.clone();
            let current_search_state = current_search_state.clone();
            let current_page_state = current_page_state.clone();
            get_results(
                new_search,
                current_search_state,
                current_page_state,
                results_state,
            )
        })
    };

    // Generate html from results from search
    let result_html = results_state
        .iter()
        .map(|result| {
            html! {<Result result={result.clone()} handle_on_click={&on_update_search} />}
        })
        .collect::<Html>();

    // Generate search tag html from current search
    let tags_html = current_search_state
        .split("&")
        .filter(|tag| !tag.is_empty())
        .map(|tag| {
            html! {<Tag text={tag.to_string()} handle_on_click={&on_remove_search} />}
        })
        .collect::<Html>();

    html! {
        <main class="main-container col expand-x expand-y fade-in">
            <h1>{"Historia"}</h1>
            <h2>{"Search, explore, learn."}</h2>
            <Search handle_on_search={&on_search}/>
            <div class="row expand-x">
                {tags_html}
            </div>
            if !results_state.is_empty() {
                <div class="col expand-x expand-y scroll">
                    {result_html}
                </div>
                if results_state.len() >= 10 {
                    <div class="row expand-x flex-end-y">
                        if *current_page_state == 0 {
                            <button class="expand-x" name="back" disabled={true} onclick={&on_change_page}>{"BACK"}</button>
                        } else {
                            <button class="expand-x" name="back" onclick={&on_change_page}>{"BACK"}</button>
                        }

                        <button class="expand-x flex-end-x" name="next" onclick={&on_change_page}>{"NEXT"}</button>
                    </div>
                }
            }
        </main>
    }
}

fn get_results(
    search_value: String,
    current_search_state: UseStateHandle<String>,
    current_page_state: UseStateHandle<i32>,
    results_state: UseStateHandle<Vec<HashMap<String, String>>>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let api_key = std::env!("API_KEY");
        let search_uri = format!(
            "https://api.api-ninjas.com/v1/historicalevents?{}&offset={}",
            search_value, *current_page_state
        );

        let search_result = Request::get(&search_uri)
            .header("X-Api-Key", api_key)
            .send()
            .await;

        // We only update the search state if our response is ok
        // if it isn't we reset the search state to empty
        // just a small workaround from having to implement error feedback
        if let Ok(res) = search_result {
            if res.ok() {
                current_search_state.set(search_value.clone());
                let results: Vec<HashMap<String, String>> = res.json().await.unwrap();
                results_state.set(results);
            } else {
                current_search_state.set("".to_string());
                results_state.set(Vec::new());
            }
        }
    });
}
