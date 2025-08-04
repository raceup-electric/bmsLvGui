mod gui;
use gui::BmsLvGui;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
       println!("gui [can_node]");
       return;
    }


    let mut bms = BmsLvGui::<12>::new("Bms Lv Gui", 800, 600,&args[1]).await;

    bms.update();
}
