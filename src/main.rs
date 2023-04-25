use color_eyre::Report;
// use data::db_actions::DbMockData;
use data::setup::setup;
use data::app::start_app;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup().await?;
    // let db_mock_data = DbMockData::new();
    // db_mock_data.clear().await?;
    // db_mock_data.fill().await?;
    start_app().await?;
    Ok(())
}

