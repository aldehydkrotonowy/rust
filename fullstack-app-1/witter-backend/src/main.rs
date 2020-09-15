use dotenv;

use sqlx::query;
use sqlx::PgPool;
use sqlx::Pool;
use tide::Request;
use tide::Server;

#[async_std::main]
async fn main() -> Result<(), Error> {
	dotenv::dotenv().ok();
	pretty_env_logger::init();

	let db_url = std::env::var("DATABASE_URL")?;
	// dbg!(db_url);

	let db_pool: PgPool = Pool::new(&db_url).await?;
	let rows = query!("select 1 as one").fetch_one(&db_pool).await?;
	dbg!(rows);
	let myName = "ngajda".to_string();
	let mut app: Server<State> = Server::with_state(State { db_pool, myName });
	app.at("/").get(|req: Request<State>| async move {
		Ok(format!("Hello, world. {}!", &req.state().myName))
	});

	app.listen("127.0.0.1:9000").await?;

	Ok(())
}

#[derive(Debug)]
struct State {
	db_pool: PgPool,
	myName: String,
}

#[derive(thiserror::Error, Debug)]
enum Error {
	#[error(transparent)]
	DbError(#[from] sqlx::Error),

	#[error(transparent)]
	IoError(#[from] std::io::Error),
	#[error(transparent)]
	VarError(#[from] std::env::VarError),
}
