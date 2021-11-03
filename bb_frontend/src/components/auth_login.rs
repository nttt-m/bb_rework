use crate::GlobalContext;
use hmac_sha256::HMAC;
use log::{error, info};
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use rand::{rngs::OsRng, RngCore};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::ops::{Add, Mul, MulAssign, Sub};
use tiny_keccak::{Hasher, Sha3};
use web_sys::{Element, EventTarget, HtmlFormElement, Node};
use yew::prelude::*;

fn submit(ctx: GlobalContext, event: FocusEvent, node_ref: NodeRef, state: UseStateHandle<bool>) {
    let form = event.target();
    if form.is_some() {
        // let form = node_ref.cast::<HtmlFormElement>().unwrap();
        event.prevent_default();
        event.stop_propagation();
        // let children = form.elements();
        // let loginBtn = children.named_item("loginBtn").unwrap();
        // let spinnerBtn = children.named_item("spinnerBtn").unwrap();
        // loginBtn.

        state.set(true);
        // let elems = node_ref.cast::<Element>().unwrap().has_attributes();
        // info!("{:?}", elems);
        login(ctx, String::from("wolfskin"), String::from("test1234"))
        // let uelem = elems.named_item("username").unwrap();
        // let pelem = elems.named_item("password").unwrap();
        //
        // if let [Some(username), Some(password)] = [uelem.text_content(), pelem.text_content()] {
        //     login(ctx, username, password).await;
        // }
    }
}

#[derive(Deserialize)]
struct Init {
    pub prime: Vec<u8>,
    pub generator: u8,
    pub salt: String,
    #[serde(alias = "pubkey")]
    pub server: Vec<u8>,
    pub nonce: String,
}

#[derive(Serialize)]
struct KeySend {
    pubkey: Vec<u8>,
    nonce: String,
}

#[derive(Deserialize)]
struct Key {
    nonce: String,
}

#[derive(Serialize)]
struct Verify {
    hmac: [u8; 32],
}

fn pow(ini: BigUint, exponent: BigUint) -> BigUint {
    let mut im = ini.clone();
    for _ in 0..exponent.to_u128().unwrap() {
        im.mul_assign(ini.clone());
    }
    im
}

#[derive(Deserialize)]
struct Carrier<T> {
    status: u16,
    message: T,
}

fn login(ctx: GlobalContext, username: String, p: String) {
    // TODO: Send identifier -- Receive Public + Salt + Prime + Generator + Nonce
    // TODO: Generate ephemeral keys and send off -- Receive new nonce
    // TODO: Calculate
    // TODO: Send HMAC for verification
    // I, P = <read from user>
    // N, g, s, B = <read from server>
    // a = random()
    // A = g^a % N
    // u = SHA1(PAD(A) | PAD(B))
    // k = SHA1(N | PAD(g))
    // x = SHA1(s | SHA1(I | ":" | P))
    // <premaster secret> = (B - (k * g^x)) ^ (a + (u * x)) % N
    wasm_bindgen_futures::spawn_local(async move {
        let mut a: [u8; 256] = [0u8; 256];
        OsRng.fill_bytes(&mut a);
        info!("Initial Call!");
        let res_1_1 = Request::get(&format!("http://127.0.0.1:3000/login/{}", username))
            .send()
            .await;
        info!("Call done!");
        match res_1_1 {
            Ok(res_1_2) => {
                let res_1 = res_1_2.json::<Carrier<Init>>().await;
                info!("Parsed JSON");
                match res_1 {
                    Ok(carrier) => {
                        let init = carrier.message;
                        info!("Starting generation...");
                        let A = num_bigint::BigUint::from(init.generator).modpow(
                            &num_bigint::BigUint::from_bytes_be(&a),
                            &num_bigint::BigUint::from_bytes_be(&init.prime),
                        );
                        let send = KeySend {
                            pubkey: A.to_bytes_be(),
                            nonce: init.nonce,
                        };
                        info!("Sending own key");
                        let res_2 =
                            Request::post(&format!("http://127.0.0.1:3000/login/{}", username))
                                .header("content-type", "application/json")
                                .body(json!(send).to_string())
                                .send()
                                .await
                                .unwrap()
                                .json::<Carrier<Key>>()
                                .await;
                        match res_2 {
                            Ok(carrier) => {
                                let key = carrier.message;
                                let u: &mut [u8] = &mut [];
                                let mut hasher = Sha3::v256();
                                hasher.update(&A.to_bytes_be());
                                hasher.update(&init.server);
                                hasher.finalize(u);
                                info!("u");
                                let k: &mut [u8] = &mut [];
                                let mut hasher = Sha3::v256();
                                hasher.update(&init.prime);
                                hasher.update(&[init.generator]);
                                hasher.finalize(k);
                                info!("k");
                                let v1: &mut [u8] = &mut [];
                                let mut hasher = Sha3::v256();
                                hasher.update(&username.clone().into_bytes());
                                hasher.update(&[':' as u8]);
                                hasher.update(&p.into_bytes());
                                hasher.finalize(v1);
                                info!("v1");
                                let verifier: &mut [u8] = &mut [];
                                let mut hasher = Sha3::v256();
                                hasher.update(&init.salt.into_bytes());
                                hasher.update(v1);
                                hasher.finalize(verifier);
                                info!("verifier");
                                // (B - (k * g^x)) ^ (a + (u * x)) % N
                                let premaster = BigUint::from_bytes_be(&init.server)
                                    .sub(BigUint::from_bytes_be(k).mul(pow(
                                        BigUint::from_bytes_be(&[init.generator]),
                                        BigUint::from_bytes_be(verifier),
                                    )))
                                    .modpow(
                                        &BigUint::from_bytes_be(&a).add(
                                            &BigUint::from_bytes_be(u)
                                                .mul(BigUint::from_bytes_be(verifier)),
                                        ),
                                        &BigUint::from_bytes_be(&init.prime),
                                    );
                                info!("premaster");
                                let hmac = hmac_sha256::HMAC::mac(
                                    &username.clone().into_bytes(),
                                    &premaster.to_bytes_be(),
                                );
                                let mac = Verify { hmac };
                                info!("Sending PM");
                                let res_3 = Request::put(&format!(
                                    "http://127.0.0.1:3000/login/{}",
                                    username
                                ))
                                .header("content-type", "application/json")
                                .body(json!(mac).to_string())
                                .send()
                                .await
                                .unwrap()
                                .status();
                                info!("Server Reply: {:?}", res_3);
                            }
                            Err(e) => info!("{:?}", e),
                        }
                    }
                    Err(e) => error!("Error on JSON: {:?}", e),
                }
            }
            Err(e) => info!("{:?}", e),
        }
    });
}

#[function_component(Login)]
pub fn login() -> Html {
    let li = use_state(|| false);
    let li_sub = li.clone();
    let node_ref = NodeRef::default();
    let ctx: GlobalContext = use_context::<GlobalContext>().expect("no ctx found");
    html! {
        <div class={classes!("container-md")}>
            <h1>{"Login"}</h1>
            <form onsubmit={Callback::from(move |e| submit(ctx.clone(), e, node_ref.clone(), li_sub.clone()))} ref={node_ref.clone()}>
                <div class={classes!("mb-3")}>
                    <label for="username" class={classes!("form-label")}>{"Username"}</label>
                    <input type="text" class={classes!("form-control")} id="username" />
                </div>
                <div class={classes!("mb-3")}>
                    <label for="password" class={classes!("form-label")}>{"Password"}</label>
                    <input type="password" class={classes!("form-control")} id="password" />
                </div>
                <button id="loginBtn" type="submit" class={classes!("btn", "btn-dark", {if *li {"d-none"} else {""}})}>{"Login"}</button>
                <button id="spinnerBtn" type="button" class={classes!("btn", "btn-dark", {if *li {""} else {"d-none"}})} disabled=true>
                    <span class={classes!("spinner-border", "spinner-border-sm")} role="status" aria-hidden="true"></span>
                    {" Loading..."}
                </button>
            </form>
        </div>
    }
}
