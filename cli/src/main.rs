type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod mesure {
    #[derive(Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "vigicrues")]
    pub struct Entity {
        pub time: chrono::NaiveDateTime,
        pub installation_id: i32,
        pub level: f32,
        pub flow: f32,
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
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let elephantry = elephantry::Connection::new(&database_url)?;

    let installations =
        elephantry.find_where::<installation::Model>("station is not null", &[], None)?;

    for installation in installations {
        let level = vigicrues::level(&installation.station).await?;
        let flow = vigicrues::flow(&installation.station).await?;

        let mesures = level
            .serie
            .mesures
            .iter()
            .zip(flow.serie.mesures)
            .map(|(l, f)| {
                if l.time != f.time {
                    panic!();
                }
                mesure::Entity {
                    time: l.time,
                    installation_id: installation.id,
                    level: l.mesure,
                    flow: f.mesure,
                }
            })
            .collect::<Vec<_>>();

        for mesure in mesures {
            elephantry.upsert_one::<mesure::Model>(&mesure, "", "nothing")?;
        }
    }

    Ok(())
}
