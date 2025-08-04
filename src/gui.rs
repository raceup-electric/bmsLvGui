use raylib::color::Color;
use raylib::init;
use raylib::prelude::RaylibDraw;
use tokio::sync::watch::{Sender,Receiver,channel};
use socketcan::{CanSocket, Socket};
use socketcan::BlockingCan;

type Volt = f32;

struct BmsCell{
    volt: Volt,
    rx: Receiver<Volt>
}


#[allow(unused)]
pub struct BmsLvGui<const N : usize>{
    tx_channels: [Sender<Volt>;N],
    socket_can: CanSocket,
}

#[allow(unused)]
impl<const N:usize> BmsLvGui<N> {
    
    pub async fn new(title: &'static str, w: i32, h: i32, can_node: &str) -> Self
    {
        let cell_width = 200;
        let cell_heidth = 100;
        let n_cell_in_a_row = 4;

        let can_node = CanSocket::open(can_node).unwrap();
        let channels : [_; N] = std::array::from_fn(|_|{
            let (tx,rx) = channel::<Volt>(0.0);
            (tx,rx)
        });

        let tx_channels = std::array::from_fn(|i|channels[i].0.clone());

        tokio::spawn(async move{
            let mut cells : [_;N] = std::array::from_fn(|i|BmsCell{ volt: 0.0, rx: channels[i].1.clone() });
            let (mut rl, rt) =init()
                .title(title)
                .width(w)
                .height(h)
                .build();


            while !rl.window_should_close()
            {
                let mut row = 0;
                let mut colomn = cell_heidth * 2;
                let mut d =rl.begin_drawing(&rt);

                for cell in cells.iter_mut()
                {
                    let mut cell_color = Color::RED;
                    let mut volt_val = cell.volt
                        .to_string();
                    volt_val.push_str(" mV");

                    if let Ok(true) = cell.rx.has_changed() {
                        cell.volt = *cell.rx.borrow_and_update();
                    }

                    d.clear_background(Color::LIGHTGRAY);
                    if cell.volt > 3000.0 && cell.volt < 4500.0 {
                        cell_color = Color::GREEN;
                    }
                    d.draw_text("Bms Lv Volts", 300, 50, 32, Color::BLACK);

                    d.draw_rectangle(row, colomn, cell_width, cell_heidth, cell_color);
                    d.draw_rectangle_lines(row, colomn, cell_width, cell_heidth, Color::WHITE);
                    d.draw_text(&volt_val, row + (cell_width/2) - 64, colomn + (cell_heidth/2), 32, Color::BLACK);
                    row += cell_width;
                    if row >= n_cell_in_a_row * cell_width {
                        row = 0;
                        colomn += cell_heidth;
                    }
                }


            }
        });

        Self{tx_channels, socket_can: can_node}
    }

    pub fn update_cell(&mut self, cell: usize, volt: Volt){
        let _ =self.tx_channels[cell].send(volt);
        
    }

    pub fn update(&mut self)-> !{
        loop{
            let mut data = [0_u8;8];
            let frame = self.socket_can.receive();

            //TODO: update logic and mex filters
        }
    }
}
