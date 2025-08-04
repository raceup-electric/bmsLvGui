use std::sync::atomic;
use std::time::Duration;

use embedded_can::Frame;
use raylib::color::Color;
use raylib::init;
use raylib::prelude::RaylibDraw;
use tokio::sync::watch::{Sender,Receiver,channel};
use socketcan::{CanSocket, Socket, SocketOptions};
use socketcan::BlockingCan;

use super::messages::*;

type Volt = f32;

struct BmsCell{
    volt: Volt,
    rx: Receiver<Volt>
}

static RUN: atomic::AtomicBool = atomic::AtomicBool::new(false);

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

        let mut can_node = CanSocket::open(can_node).unwrap();

        let filters = 
        [
            socketcan::CanFilter::new(BmsLvCell1::MESSAGE_ID, u32::MAX ^ 0b11),
        ];

        can_node.set_filters(&filters);

        
        RUN.store(true, atomic::Ordering::Relaxed);

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
            RUN.store(false, atomic::Ordering::Relaxed);
        });

        Self{tx_channels, socket_can: can_node}
    }

    pub fn update_cell(&mut self, cell: usize, volt: Volt){
        let _ =self.tx_channels[cell].send(volt);
        
    }

    pub fn update(&mut self){

        fn get_mv(raw: u16) -> f32{
            f32::from(raw)/10.0
        }

        let cell_enable = BmsLvCellControl::new(true).ok().unwrap();
        let id = socketcan::StandardId::new(BmsLvCellControl::MESSAGE_ID.try_into().unwrap()).unwrap();
        let frame = socketcan::frame::CanFrame::new(id, cell_enable.raw()).unwrap();
        let _ = self.socket_can.transmit(&frame);


        while RUN.load(atomic::Ordering::Relaxed){
            self.socket_can.set_read_timeout(Duration::from_millis(500));
            let frame = self.socket_can.receive();

            if let Ok(mex) = frame {
                if let socketcan::Id::Standard(standard_id) = mex.id(){
                    let mex = Messages::from_can_message(standard_id.as_raw().into(), mex.data()).ok().unwrap();
                    match mex {
                        Messages::BmsLvCell1(data) => {
                            self.update_cell(0, get_mv(data.cell_0()));
                            self.update_cell(1, get_mv(data.cell_1()));
                            self.update_cell(2, get_mv(data.cell_2()));
                            self.update_cell(3, get_mv(data.cell_3()));
                        },
                        Messages::BmsLvCell2(data) => {
                            self.update_cell(4, get_mv(data.cell_4()));
                            self.update_cell(5, get_mv(data.cell_5()));
                            self.update_cell(6, get_mv(data.cell_6()));
                            self.update_cell(7, get_mv(data.cell_7()));
                        },
                        Messages::BmsLvCell3(data) => {
                            self.update_cell(8, get_mv(data.cell_8()));
                            self.update_cell(9, get_mv(data.cell_9()));
                            self.update_cell(10, get_mv(data.cell_10()));
                            self.update_cell(11, get_mv(data.cell_12()));
                        },
                        _ => println!("unexpected mex"),
                    };
                }
            }
        }

        let cell_enable = BmsLvCellControl::new(false).ok().unwrap();
        let id = socketcan::StandardId::new(BmsLvCellControl::MESSAGE_ID.try_into().unwrap()).unwrap();
        let frame = socketcan::frame::CanFrame::new(id, cell_enable.raw()).unwrap();
        let _ =self.socket_can.transmit(&frame);
    }
}
