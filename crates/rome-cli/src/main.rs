// use rome_server::RomeCommands;
// use rustyline::{error::ReadlineError, *};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    println!("🏛️  Rome");
    println!("formavi, mundavi, compixi");

    let rome = rome_server::server::RomeBuilder::new();
    let client = rome.start();

    client.watch_dir("C:/github/romepoc/sampleproject").await;
    client
        .project_start("javascript", "C:/github/romepoc/sampleproject")
        .await;

    let r = client
        .analyse_what("C:/github/romepoc/sampleproject/main.js".to_string(), 0, 0)
        .await;
    println!("{:?}", r);

    std::thread::sleep(std::time::Duration::from_millis(5000));

    let r = client
        .analyse_what("C:/github/romepoc/sampleproject/main.js".to_string(), 0, 0)
        .await;
    println!("{:?}", r);

    // let mut history_file = dirs::home_dir().unwrap();
    // history_file.push(".rome");
    // let _ = std::fs::create_dir_all(&history_file);
    // history_file.push("history");

    // let mut rl = Editor::<()>::new();
    // let _ = rl.load_history(&history_file);
    // loop {
    //     let readline = rl.readline("> ");
    //     match readline {
    //         Ok(line) => {
    //             let _ = rl.add_history_entry(line.as_str());
    //             let _ = rl.save_history(&history_file);

    //             let cmd = RomeCommands::from(format!("rome {}", line));
    //             let _ = client.send_async(cmd).await;
    //         }
    //         Err(ReadlineError::Interrupted) => {
    //             println!("CTRL-C");
    //             break;
    //         }
    //         Err(ReadlineError::Eof) => {
    //             println!("CTRL-D");
    //             break;
    //         }
    //         Err(err) => {
    //             println!("Error: {:?}", err);
    //             break;
    //         }
    //     }
    // }
}
