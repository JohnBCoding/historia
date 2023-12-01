use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub result: HashMap<String, String>,
    pub handle_on_click: Callback<(String, String)>,
}

#[function_component(Result)]
pub fn result(props: &Props) -> Html {
    let on_click = {
        let handle_on_click = props.handle_on_click.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let element = event.target_unchecked_into::<HtmlSpanElement>();
            let name = element.get_attribute("name").unwrap();
            let value = element.text_content().unwrap();

            handle_on_click.emit((name, value));
        })
    };

    let highlight_html = {
        let result = props.result.get("event").unwrap();
        let split = result.split(" ");
        let mut group_list = Vec::new();
        let mut same = Vec::new();
        for s in split {
            for c in s.chars() {
                if c.is_ascii_uppercase() || c.is_numeric() {
                    same.push(s);
                } else {
                    if !same.is_empty() {
                        group_list.push(
                            html! {<a href={format!("https://en.wikipedia.org/w/index.php?search={}&title=Special%3ASearch&ns0=1", same.join("+"))} target="_blank">{format!("{}", same.join(" "))}</a>}
                        );
                    }

                    group_list.push(html! (<span>{format!(" {} ", s)}</span>));
                    same = Vec::new();
                }

                break;
            }
        }

        if !same.is_empty() {
            group_list.push(html! {<a href={format!("https://en.wikipedia.org/w/index.php?search={}&title=Special%3ASearch&ns0=1", same.join("+"))} target="_blank">{format!("{}", same.join(" "))}</a>});
        }

        html! {
            <div class="col">
                <p>{group_list}</p>
                <p class="accent">
                    <span class="date accent" name="year" onclick={&on_click}>{format!("{}", props.result.get("year").unwrap())}</span>
                    <span>{" / "}</span>
                    <span class="date accent" name="month" onclick={&on_click}>{format!("{}", props.result.get("month").unwrap())}</span>
                    <span>{" / "}</span>
                    <span class="date accent" name="day" onclick={&on_click}>{format!("{}", props.result.get("day").unwrap())}</span>
                </p>
            </div>

        }
    };

    html! {
        <div class="result-container col expand-x fade-in">
            {highlight_html}
        </div>
    }
}
