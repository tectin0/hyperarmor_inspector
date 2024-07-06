// const URL: &str = "https://docs.google.com/spreadsheets/d/1j4bpTbsnp5Xsgw9TP2xv6d8R4qk0ErpE9r_5LGIDraU/gviz/tq?tqx=out:csv&sheet=Poise%20Damage%20-%20PvP"; // this those not export cells with a "+" correctly
const URL: &str = "https://docs.google.com/spreadsheets/d/1j4bpTbsnp5Xsgw9TP2xv6d8R4qk0ErpE9r_5LGIDraU/export?format=csv&gid=419422255";

pub fn download_poise_data() {
    log::info!("Downloading Poise data from {}", URL);

    let client = reqwest::blocking::Client::new();
    let response = client.get(URL).send().unwrap();

    let mut rdr = csv::Reader::from_reader(response);

    let filename = "poise_data.csv";
    log::info!("Saving Poise data to {}", filename);

    let file = std::fs::File::create(filename).unwrap();

    let header = rdr.headers().unwrap();

    let mut writer = csv::Writer::from_writer(&file);

    writer.write_record(header).unwrap();

    writer.flush().unwrap();

    for result in rdr.records() {
        let record = result.unwrap();

        let name = record.get(1).unwrap();

        if name.is_empty() {
            continue;
        }

        if name == "Erdsteel Dagger" {
            dbg!(&record);
        }

        writer.write_record(&record).unwrap();

        writer.flush().unwrap();
    }

    log::info!("Downloaded Poise data");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_poise_data() {
        download_poise_data();
    }
}
