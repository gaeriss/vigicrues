pub type Result<T = ()> = std::result::Result<T, reqwest::Error>;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, serde::Deserialize)]
pub enum Scale {
    #[default]
    #[serde(rename = "H")]
    Level,
    #[serde(rename = "Q")]
    Flow,
}

impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Flow => "Q",
            Self::Level => "H",
        };

        f.write_str(s)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    #[serde(rename = "VersionFlux")]
    pub version: String,
    pub serie: Serie,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Serie {
    #[serde(rename = "CdStationHydro")]
    pub code_station: String,
    #[serde(rename = "LbStationHydro")]
    pub label_station: String,
    pub link: String,
    #[serde(rename = "GrdSerie")]
    pub scale: Scale,
    #[serde(rename = "ObssHydro")]
    pub mesures: Vec<Mesure>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mesure {
    #[serde(rename = "DtObsHydro")]
    pub time: chrono::DateTime<chrono::offset::Local>,
    #[serde(rename = "ResObsHydro")]
    pub mesure: f32,
}

pub async fn level(station: &str) -> Result<Data> {
    fetch(station, Scale::Level).await
}

pub async fn flow(station: &str) -> Result<Data> {
    fetch(station, Scale::Flow).await
}

async fn fetch(station: &str, scale: Scale) -> Result<Data> {
    let url = format!("https://www.vigicrues.gouv.fr/services/observations.json/index.php?CdStationHydro={station}&GrdSerie={scale}&FormatDate=iso");

    reqwest::get(&url).await?.json().await
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn level() {
        assert!(dbg!(crate::level("M730242011").await).is_ok());
    }

    #[tokio::test]
    async fn flow() {
        assert!(dbg!(crate::flow("M730242011").await).is_ok());
    }
}
