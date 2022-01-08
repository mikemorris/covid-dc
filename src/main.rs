use chrono::{DateTime, Utc, serde::ts_milliseconds};
use chrono_tz::US::Eastern;
use derivative::Derivative;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Derivative)]
#[derivative(Debug)]
struct RapidSiteAttributes {
    #[serde(rename(deserialize = "Datetime_of_Inventory"),
        with = "ts_milliseconds")]
    #[derivative(Debug(format_with = "human_datetime"))]
    datetime_of_inventory: DateTime<Utc>,
    #[serde(rename(deserialize = "How_Many_Tests_Left"))]
    tests: usize,
    #[serde(rename(deserialize = "Is_Last_Inventory"))]
    is_last_inventory: String,
    #[serde(rename(deserialize = "ObjectId"))]
    id: usize,
    #[serde(rename(deserialize = "TYS_Rapid_Site_Name"))]
    name: String,
}

fn human_datetime(d: &DateTime<Utc>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", d.with_timezone(&Eastern).to_rfc2822())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Feature {
    RapidSite {
        attributes: RapidSiteAttributes
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FeatureCollection {
    features: Vec<Feature>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.get("https://services.arcgis.com/neT9SoYxizqTHZPH/arcgis/rest/services/Join_Features_to_COVID_19_Testing_Locations_DB_WFL1___DCGov_Testing_Locations/FeatureServer/0/query")
    .query(&[
           ("f", "json"),
           ("cacheHint","true"),
           ("maxRecordCountFactor", "4"),
           ("resultOffset", "0"),
           ("resultRecordCount", "8000"),
           ("where", "TYS_Rapid_Site_Name IS NOT NULL"),
           ("orderByFields", "ObjectId"),
           ("outFields", &[
                "Datetime_of_Inventory",
                "How_Many_Tests_Left",
                "Is_Last_Inventory",
                "ObjectId",
                "TYS_Rapid_Site_Name"
           ].join(",")),
           ("outSR", "102100"),
           ("spatialRel", "esriSpatialRelIntersects")
        ])
        .send()
        .await?
        .json::<FeatureCollection>()
        .await?;

    println!("{:#?}", resp);

    Ok(())
}