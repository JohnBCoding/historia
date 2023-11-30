use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub result: HashMap<String, String>,
}

#[function_component(Result)]
pub fn result(props: &Props) -> Html {
    let highlight_html = {
        let result = props.result.get("event").unwrap();
        let split = result.split(" ");
        let mut group_list = Vec::new();
        let mut same = Vec::new();
        for s in split {
            for c in s.chars() {
                if c.is_ascii_uppercase() {
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
            same = Vec::new();
        }

        html! {
            <div class="col">
                <p>{group_list}</p>
                <p class="accent">{format!("{} / {} / {}", props.result.get("year").unwrap(), props.result.get("month").unwrap(), props.result.get("day").unwrap())}</p>
            </div>

        }
    };

    html! {
        <div class="result-container col expand-x fade-in">
            {highlight_html}
        </div>
    }
}
