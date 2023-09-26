type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod mesure {
    #[derive(Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "vigicrues")]
    pub struct Entity {
        pub time: chrono::DateTime<chrono::offset::Local>,
        pub installation_id: i32,
        pub level: Option<f32>,
        pub flow: Option<f32>,
    }
}

mod installation {
    #[derive(Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "installation")]
    pub struct Entity {
        #[elephantry(pk, column = "installation_id")]
        pub id: i32,
        pub name: String,
        pub station: String,
    }
}

#[tokio::main]
async fn main() -> Result {
    let database_url = envir::get("DATABASE_URL")?;
    let elephantry = elephantry::Connection::new(&database_url)?;

    let installations =
        elephantry.find_where::<installation::Model>("station is not null", &[], None)?;

    for installation in installations {
        let level = vigicrues::level(&installation.station).await?;

        let mesures = level
            .serie
            .mesures
            .iter()
            .map(|l| {
                mesure::Entity {
                    time: l.time,
                    installation_id: installation.id,
                    level: Some(l.mesure),
                    flow: None,
                }
            });

        for mesure in mesures {
            elephantry.upsert_one::<mesure::Model>(&mesure, "", "nothing")?;
        }

        let flow = vigicrues::flow(&installation.station).await?;

        let mesures = flow
            .serie
            .mesures
            .iter()
            .map(|f| {
                mesure::Entity {
                    time: f.time,
                    installation_id: installation.id,
                    level: None,
                    flow: Some(f.mesure),
                }
            });

        for mesure in mesures {
            elephantry.upsert_one::<mesure::Model>(&mesure, "(\"time\", installation_id)", "update set flow = excluded.flow")?;
        }
    }

    Ok(())
}
