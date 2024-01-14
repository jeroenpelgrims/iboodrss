use clap::Parser;
use curl::easy::Easy;
use rss::{ChannelBuilder, Item, ItemBuilder};
use scraper::{Html, Selector};
use serde_json::Value;
use std::str;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    country: String,

    #[arg(short, long)]
    lang: String,
}

#[derive(Debug)]
struct Offer {
    offer_id: String,
    title: String,
    slug: String,
    description: String,
    image_url: String,
    start_date: String,
    delivery_price_currency: String,
    delivery_price_value: f64,
    price_currency: String,
    price_value: f64,
}

fn main() {
    let args = Args::parse();
    let url = format!("https://www.ibood.com/{}/s-{}", args.lang, args.country);
    let data = get_data(url);
    let rss = generate_rss(args.lang, args.country, data);
    print!("{:?}", rss);
}

fn get_data(url: String) -> Vec<Offer> {
    let html = get_html(&url);
    let next_data = get_next_data(&html);
    let json: Value = serde_json::from_str(&next_data).unwrap();
    let json_offers = json["props"]["pageProps"]["offers"].as_array().unwrap();
    let offers: Vec<Offer> = json_offers
        .iter()
        .map(|offer| Offer {
            offer_id: offer["offerItemClassicId"].as_str().unwrap().to_string(),
            title: offer["title"].as_str().unwrap().to_string(),
            slug: offer["slug"].as_str().unwrap().to_string(),
            description: offer["description"].as_str().unwrap().to_string(),
            image_url: offer["image"].as_str().unwrap().to_string(),
            start_date: offer["start"].as_str().unwrap().to_string(),
            delivery_price_currency: offer["deliveryPrice"]["currency"]
                .as_str()
                .unwrap()
                .to_string(),
            delivery_price_value: offer["deliveryPrice"]["value"].as_f64().unwrap() as f64,
            price_currency: offer["price"]["currency"].as_str().unwrap().to_string(),
            price_value: offer["price"]["value"].as_f64().unwrap() as f64,
        })
        .collect();

    offers
}

fn get_html(url: &str) -> String {
    let mut handle = Easy::new();
    let mut data = Vec::new();

    let _ = handle.useragent("curl/7.88.1");
    handle.url(&url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    let html = str::from_utf8(&data).unwrap();
    html.to_string()
}

fn get_next_data(html: &str) -> String {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("script#__NEXT_DATA__").unwrap();
    let script_tag = fragment.select(&selector).next().unwrap();
    let json = script_tag.inner_html();
    json
}

fn generate_rss(lang: String, country: String, offers: Vec<Offer>) -> String {
    let mut channel = ChannelBuilder::default()
        .title("iBOOD.com".to_string())
        .link("https://www.ibood.com".to_string())
        .description("Unofficial iBOOD RSS feed".to_string())
        .build();

    let mut items: Vec<Item> = offers
        .iter()
        .map(|offer| {
            let offer_url = offer_to_url(&lang, &country, &offer);
            let enclosure = rss::EnclosureBuilder::default()
                .url(offer.image_url.to_string())
                .mime_type("image/avif".to_string())
                .build();
            ItemBuilder::default()
                .title(Some(offer.title.to_string()))
                // .description(Some(offer.description.to_string()))
                .pub_date(Some(offer.start_date.to_string()))
                .link(Some(offer_url))
                .content(Some("FOO BAR".to_string())) //TODO, Tera template
                .enclosure(Some(enclosure))
                .build()
        })
        .collect();
    channel.items.append(&mut items);

    channel.to_string()
}

fn offer_to_url(lang: &String, country: &String, offer: &Offer) -> String {
    format!(
        "https://www.ibood.com/{}/s-{}/o/{}/{}",
        lang, country, offer.slug, offer.offer_id
    )
}
