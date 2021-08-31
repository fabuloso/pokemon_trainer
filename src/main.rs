use rouille::router;

fn main() {
    rouille::start_server("localhost:4567", move |request| {
        router!(request,
        (POST)(/pokemons)=> {
            rouille::Response::text("ciao")
        },
        _ => {
            rouille::Response::empty_400()
        })
    });
}
