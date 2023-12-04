use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub result: HashMap<String, String>,
    pub handle_on_click: Callback<(String, String)>,
}

#[function_component(Result)]
pub fn result(props: &Props) -> Html {
    let clipboard = use_clipboard();
    let parent_ref = use_node_ref();

    // Pass year/date/month value to callback
    // this will be handled in content.rs to create a new search tag
    let on_click_date = {
        let handle_on_click = props.handle_on_click.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let element = event.target_unchecked_into::<HtmlSpanElement>();
            let name = element.get_attribute("name").unwrap();
            let value = element.text_content().unwrap();

            handle_on_click.emit((name, value));
        })
    };

    // Collect all text in result and write it to the clipboard
    // Use the yew_hooks handle for this
    let on_click_copy = {
        let clipboard = clipboard.clone();
        let parent_ref = parent_ref.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            // Collection of text content of elements with the copy class
            let parent = parent_ref.cast::<HtmlDivElement>().unwrap();
            let copy_elements = parent.get_elements_by_class_name("copy");
            let mut text = "".to_string();
            for i in 0..copy_elements.length() {
                let element = copy_elements.get_with_index(i).unwrap();
                text = format!("{} {}", text, element.text_content().unwrap().trim());
            }

            clipboard.write_text(text);
        })
    };

    // Creates html of the event
    // also turns titled text into links to wikipedia
    let highlight_html = {
        let result = props.result.get("event").unwrap();
        let split = result.split(" ");
        let mut group_list = Vec::new();
        let mut same = Vec::new();

        // We iter our split strings here, looking for uppercase letters/numbers on first iteration
        // If uppercase/numbers are found, we group those strings into a vector together
        // until a lowercase string is found. This group of strings is then turned into a
        // link to wikipedia and the process starts again.
        for s in split {
            for c in s.chars() {
                if c.is_ascii_uppercase() || c.is_numeric() {
                    same.push(s);
                } else {
                    if !same.is_empty() {
                        group_list.push(
                            html! {<a class="copy" href={format!("https://en.wikipedia.org/w/index.php?search={}&title=Special%3ASearch&ns0=1", same.join("+"))} target="_blank">{format!("{}", same.join(" "))}</a>}
                        );
                    }

                    group_list.push(html! (<span class="copy">{format!(" {} ", s)}</span>));
                    same = Vec::new();
                }

                break;
            }
        }

        // Push any remaining groups after end of iter
        if !same.is_empty() {
            group_list.push(html! {<a class="copy" href={format!("https://en.wikipedia.org/w/index.php?search={}&title=Special%3ASearch&ns0=1", same.join("+"))} target="_blank">{format!("{}", same.join(" "))}</a>});
        }

        // Also split the date into multiple clickable spans here
        html! {
            <div class="col" ref={parent_ref}>
                <p>{group_list}</p>
                <div class="row">
                    <p class="copy accent">
                        <span class="date accent" name="year" onclick={&on_click_date}>{format!("{}", props.result.get("year").unwrap())}</span>
                        <span>{" / "}</span>
                        <span class="date accent" name="month" onclick={&on_click_date}>{format!("{}", props.result.get("month").unwrap())}</span>
                        <span>{" / "}</span>
                        <span class="date accent" name="day" onclick={&on_click_date}>{format!("{}", props.result.get("day").unwrap())}</span>
                    </p>
                    <a class="copy-a flex-end-x" onclick={&on_click_copy}>{"Copy"}</a>
                </div>
            </div>
        }
    };

    html! {
        <div class="result-container col expand-x fade-in">
            {highlight_html}
        </div>
    }
}
