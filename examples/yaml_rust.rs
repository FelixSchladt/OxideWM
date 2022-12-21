use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
struct DataSource {
    name: String,
    url: String,
    source_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    update_frequency_sec: u32,
    num_threads: u32,
    data_sources: Vec<DataSource>,
}
fn main() {
    let f = std::fs::File::open("./examples/config.yml").expect("Could not open file.");
    let mut scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    println!("{:?}", scrape_config);

    println!(
        "update_frequency_sec: {}",
        scrape_config.update_frequency_sec
    );

    for data_source in scrape_config.data_sources.iter() {
        println!(
            "name: {}, type: {}, url {}",
            data_source.name, data_source.source_type, data_source.url
        );
    }

    scrape_config.num_threads = 2;

    scrape_config.data_sources.push(DataSource {
        name: "NYTimes".to_string(),
        url: "www.nytimes.com".to_string(),
        source_type: "news".to_string(),
    });
    scrape_config.data_sources.push(DataSource {
        name: "Yahoo News".to_string(),
        url: "news.yahoo.com".to_string(),
        source_type: "news".to_string(),
    });

    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("./examples/new_config.yml")
        .expect("Couldn't open file");
    serde_yaml::to_writer(f, &scrape_config).unwrap();
}
