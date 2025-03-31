use maxminddb::Reader;
use std::net::IpAddr;
use std::error::Error;

#[derive(Debug, serde::Serialize)]
pub struct GeoData {
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
}

pub struct GeoLocator {
    reader: Reader<Vec<u8>>,
}

impl GeoLocator {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let reader = Reader::open_readfile(db_path)?;
        Ok(Self {reader})
    }

    pub fn lookup(&self, ip: &str) -> Result<GeoData, Box<dyn Error>> {
        let ip: IpAddr = ip.parse()?;
        let result: maxminddb::geoip2::City = self.reader.lookup(ip)?;

        Ok(GeoData { 
            country: result.country
                .and_then(|c| c.names)
                .and_then(|names| names.get("en").map(|s| s.to_string())),
            city: result.city
                .and_then(|c| c.names)
                .and_then(|names| names.get("en").map(|s| s.to_string())), 
            latitude: result.location.as_ref().and_then(|l| l.latitude), 
            longitude: result.location.as_ref().and_then(|l| l.longitude), 
            timezone: result.location.as_ref().and_then(|l| l.time_zone).map(String::from),
        
         })
    }

}