mod gui;

use gui::renderer::BmsLvGui;

#[tokio::main]
async fn main() {
    BmsLvGui::<12>::new("Bms Lv Gui", 800, 600).await;
}
