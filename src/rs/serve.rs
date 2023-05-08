use cubing::alg::Move;
use cubing::kpuzzle::KPuzzleDefinition;
use cubing::kpuzzle::KStateData;

use rouille::router;
use rouille::try_or_400;
use rouille::Request;
use rouille::Response;
use serde::Deserialize;
use serde::Serialize;

use std::sync::Mutex;

use crate::options::reset_args_from;
use crate::options::CommonSearchArgs;
use crate::options::ServeCommandArgs;
use crate::rust_api;
use crate::serialize::serialize_kpuzzle_definition;
use crate::serialize::serialize_scramble_state_data;
use crate::serialize::KPuzzleSerializationOptions;

fn set_definition(
    def: KPuzzleDefinition,
    options: &KPuzzleSerializationOptions,
) -> Result<(), Response> {
    let s = match serialize_kpuzzle_definition(def, Some(options)) {
        Ok(s) => s,
        Err(e) => {
            return Err(Response::text(format!("Invalid definition: {}", e)).with_status_code(400));
        }
    };
    rust_api::rust_setksolve(&s);
    Ok(())
}

#[derive(Serialize)]
struct ResponseAlg {
    alg: String, // TODO: support automatic alg serialization somehome
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScrambleSolve {
    definition: KPuzzleDefinition,
    scramble_alg: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StateSolve {
    definition: KPuzzleDefinition,
    state: KStateData,
    move_subset: Option<Vec<Move>>,
    start_state: Option<KStateData>,
}

fn solveposition(request: &Request, search_args: &CommonSearchArgs) -> Response {
    let state_solve: StateSolve = try_or_400!(rouille::input::json_input(request));
    reset_args_from(vec![search_args]);
    match set_definition(
        state_solve.definition,
        &KPuzzleSerializationOptions {
            move_subset: state_solve.move_subset,
            custom_start_state: state_solve.start_state,
        },
    ) {
        Ok(_) => {}
        Err(response) => return response,
    };
    let solution = rust_api::rust_solveposition(&serialize_scramble_state_data(
        "AnonymousScramble",
        &state_solve.state,
    )); // TODO: catch exceptions???
    Response::json(&ResponseAlg { alg: solution })
}

fn cors(response: Response) -> Response {
    response
        .with_additional_header("Access-Control-Allow-Origin", "*")
        .with_additional_header("Access-Control-Allow-Headers", "Content-Type")
}

pub fn serve(serve_command_args: ServeCommandArgs) {
    let solve_mutex = Mutex::new(());
    println!(
        "Starting `twsearch-rs serve` on port 2023.
Use with one of the following:

- https://experiments.cubing.net/cubing.js/twsearch/text-ui.html
- http://localhost:3333/experiments.cubing.net/cubing.js/twsearch/text-ui.html
"
    );
    rouille::start_server("0.0.0.0:2023", move |request: &Request| {
        println!("Request: {} {}", request.method(), request.url()); // TODO: debug flag
                                                                     // TODO: more fine-grained CORS?
        if request.method() == "OPTIONS" {
            // pre-flight!
            return cors(Response::empty_204());
        }
        cors(router!(request,
            (GET) (/) => {
                Response::text("twsearch-rs (https://github.com/cubing/twsearch)")
            },
            (POST) (/v0/solve/state) => { // TODO: `…/pattern`?
                if let Ok(guard) = solve_mutex.try_lock() {
                    let response = solveposition(request, &serve_command_args.search_args);
                    drop(guard);
                    response
                } else {
                    // TODO: support aborting the search on the C++ side.
                    Response::text("Only one non-abortable search at a time is currently supported.").with_status_code(409)
                }
            },
            _ => {
                println!("Invalid request: {} {}", request.method(), request.url());
                rouille::Response::empty_404()
            }
        ))
    });
}
