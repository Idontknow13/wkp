use wikipedia_cli::schema::WikiResponse;

fn main() {
    let titles = vec!["Pet door", "Anesthetic"]
        .into_iter()
        .map(|s| s.into())
        .collect();

    let queries = match WikiResponse::get(titles) {
        Ok(resp) => resp.pages(),
        Err(err) => {
            println!("{err:?}");
            panic!(); // TODO: handle this
        }
    };
    println!("{queries:#?}");
}
