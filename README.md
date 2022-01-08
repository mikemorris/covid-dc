# covid-dc
Parses COVID-19 testing data from DC government ArcGIS APIs

## Example debug output from `cargo run`
```
RapidSite {
    attributes: RapidSiteAttributes {
        datetime_of_inventory: "Sat, 08 Jan 2022 14:24:28 -0500",
        tests: 937,
        is_last_inventory: false,
        id: 277,
        name: "Capitol View Library",
    },
},
```

## TODO
- Add support for parsing take-home PCR test inventory from https://coronavirus.dc.gov/testyourself
- Join with data to include address and lat/lng of test pickup locations
- Actually do useful things with the testing inventory data
