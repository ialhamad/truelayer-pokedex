# Pokedex - TrueLayer

Pokedex is a service that lets you fetch minimum but useful information about your favorite Pokémon, and in addition to that you can have the description of the Pokémon in either Yoda or Shakespeare style.

## Endpoints

1. Basic Pokémon information

	```
	GET /pokemon/<Pokémon name>
	```
	Response:

	```json
	{
		"name": "onix",
		"description": "Opening its large mouth, it ingests massive amounts of soil and creates long tunnels.",
		"is_legendary": false,
		"habitat": "cave"
	}
	```
2. Basic Pokémon information with translated description. 

	The description will be translated to Yoda if the Pokémon is legendary or it's habitat is `cave`, otherwise it will be translated to Shakespeare.
	_It uses https://funtranslations.com for the translations_.
	

	```
	GET /translated/<Pokémon name>
	```
	Response:

	```json
	{
		"name": "onix",
		"description": "Opening its large mouth,Long tunnels,  it ingests massive amounts of soil and creates.",
		"is_legendary": false,
		"habitat": "cave"
	}
	```
## Running

* Using Rust

	* Requirements
		* `rust` - You can install it using https://rustup.rs/.
			* built using `rust-1.57`.

	* Steps
		1. Clone this `repo` to your machine and `cd` into it.
		2. You can do a release build using `cargo build --release`.
		3. To run the service `cargo run --release`.

* Using Docker

	* Requirements
		* `docker` - You can install it from https://www.docker.com/products/docker-desktop.

	* Steps
		1. Clone this `repo` to your machine and `cd` into it.
		2. Run `docker-compose up`.



##  Production Considerations

Here is a short list of things I will consider doing if this API needs to run on production.

1. Caching - Adding a cache for all the translation request and the most frequently requested Pokémon.
	* We are mostly working with static data, which can be easily cached.
2. Logging - Integrate a more sophisticated tracing and logging, maybe using the `tracing` crate.
3. Error handling - Better internal errors built with proper messaging instead of only relaying on the errors returned by the libraries used. 
	* Better handling of the rate limit case of the FunTranslation API with is 5 calls/hour.
4. Better testing - Create better unit and integration tests for the service.
5. CI/CD - Make use of something like GitHub Actions for building and running tests.

