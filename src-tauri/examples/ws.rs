// use kiwi_app_lib::commands::server::Engine;
// use local_ip_address::local_ip;
// use std::time::Duration;

// #[tokio::main]
// async fn main() {
//     let mut engine = Engine::new_any();
//     let failed_handler = Box::new(|error| {
//         dbg!(error);
//     });
//     let _ = engine.serve_in_background(failed_handler).await;

//     dbg!(&engine);

//     tokio::time::sleep(Duration::from_secs(3)).await;

//     let ip = {
//         if engine.is_local() {
//             engine.ip.clone()
//         } else {
//             local_ip().unwrap().to_string()
//         }
//     };

//     match Engine::is_remote_alive(&ip, engine.port).await {
//         Ok(_) => println!("engine is alive"),
//         Err(e) => {
//             println!("engine is not alive: {}", e.to_string())
//         }
//     }
//     tokio::time::sleep(Duration::from_secs(3)).await;

//     engine.shutdown().await;

//     tokio::time::sleep(Duration::from_secs(3)).await;

//     match Engine::is_remote_alive(&ip, engine.port).await {
//         Ok(_) => println!("engine is alive"),
//         Err(e) => {
//             println!("engine is not alive: {}", e.to_string())
//         }
//     }

//     tokio::time::sleep(Duration::from_secs(1000)).await;
// }

fn main() {}
