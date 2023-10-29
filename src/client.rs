use num_bigint::BigUint;
use std::io::stdin;

pub mod zkp_auth {
    include!("./zkp_auth.rs");
}

use zkp_auth::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    RegisterRequest,
};

use zkp_chaum_pedersen::ZKP;

#[tokio::main]
async fn main() {
    let addr = "http://127.0.0.1:50051";
    let mut buf = String::new();

    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP {
        alpha: alpha.clone(),
        beta: beta.clone(),
        p: p.clone(),
        q: q.clone(),
    };

    let mut client = AuthClient::connect(addr)
        .await
        .expect("could not connect to the server");

    println!("✅ Connected to the server!");

    println!("Please provide the username!");
    stdin()
        .read_line(&mut buf)
        .expect("Could not get the username from stdin");
    let username = buf.trim().to_string();
    buf.clear();

    println!("Please provide the password!");
    stdin()
        .read_line(&mut buf)
        .expect("Could not get the password from stdin");
    let password = ZKP::deserialize(buf.trim().as_bytes());
    buf.clear();

    // let y1 = ZKP::exponentiate(&alpha, &password, &p);
    // let y2 = ZKP::exponentiate(&beta, &password, &p);

    let (y1, y2) = zkp.compute_pair(&password);
    let request = RegisterRequest {
        user: username.clone(),
        y1: ZKP::serialize(&y1),
        y2: ZKP::serialize(&y2),
    };

    let _response = client
        .register(request)
        .await
        .expect("Could not register in server");
    // println!("The response to register is: {:?}", _response);
    println!("✅ Registration was successful");

    println!("Please provide the password (to login)!");
    stdin()
        .read_line(&mut buf)
        .expect("Could not get the password from stdin");
    let password = ZKP::deserialize(buf.trim().as_bytes());
    buf.clear();

    let k = ZKP::generate_random_number_below(&q);
    // let r1 = ZKP::exponentiate(&alpha, &k, &p);
    // let r2 = ZKP::exponentiate(&beta, &k, &p);

    let (r1, r2) = zkp.compute_pair(&k);
    let request = AuthenticationChallengeRequest {
        user: username,
        r1: ZKP::serialize(&r1),
        r2: ZKP::serialize(&r2),
    };

    let response = client
        .create_authentication_challenge(request)
        .await
        .expect("Could not request challenge in server")
        .into_inner();
    println!("The response to auth challenge is: {:?}", response);

    let auth_id = response.auth_id;
    let cc = &response.c;
    let c = ZKP::deserialize(cc);

    let s = zkp.solve(&k, &c, &password);

    let request = AuthenticationAnswerRequest {
        auth_id,
        s: ZKP::serialize(&s),
    };

    let response = client
        .verify_authentication(request)
        .await
        .expect("Could not verify authentication in server")
        .into_inner();
    println!("You logged in!! session_id: {:?}", response.session_id);
}
