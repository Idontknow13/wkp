use wikipedia_cli::schema::WikiResponse;

fn main() {
  let pet_door =
    match WikiResponse::get(vec!["Pet_door", "Anesthetic"]) {
        Ok(resp) => resp.pages(),
        Err(err) => {
            println!("{err:?}");
            panic!();
        }
    };
  println!("{pet_door:#?}");
}
