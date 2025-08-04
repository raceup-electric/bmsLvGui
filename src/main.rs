mod gui;

use gui::renderer::BmsLvGui;

#[tokio::main]
async fn main() {
    let mut bms = BmsLvGui::<12>::new("Bms Lv Gui", 800, 600).await;

    loop {
        for i in 0..12{
            bms.update_cell(i, 3655.5);
        }
        
    }
}
