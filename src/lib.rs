#[macro_use]
extern crate env_logger;

#[macro_use]
extern crate tower_web;
use hyper::Response;
use futures::future::{ok, Future};
use web3::client::Web3;

impl_web! {

    #[derive(Clone)]
    pub struct MyStruct {
        pub endpoint: String,
    }

    impl MyStruct {

        #[get("/")]
        fn foo(&self) -> impl Future<Item = Response<String>, Error = Response<String>> {
            self.bar().map_err(|_err| {
                Response::builder().body("SomeError".to_string()).unwrap()
            })
            .and_then(move |_| {
                ok(Response::builder().body("SomeResponse".to_string()).unwrap())
            })
        }

        fn bar(&self) -> Box<dyn Future<Item = (), Error = ()> + Send> {
            let web3 = Web3::new(&self.endpoint);
            Box::new(
                web3.eth_accounts()
                .map_err(move |err| println!("Couldn't fetch nonce. Got error: {:#?}", err))
                .and_then(move |nonce| {
                    println!("Obtained nonce {}", nonce);
                    Ok(())
                })
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_this_work() {
        let _ = env_logger::try_init();
        let s = MyStruct {
            endpoint: "http://127.0.0.1:8545".to_string(),
        };

        s.foo().wait().unwrap();
    }
}
