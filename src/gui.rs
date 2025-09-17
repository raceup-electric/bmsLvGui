use std::sync::atomic;
use std::time::Duration;

use embedded_can::Frame;
use raylib::color::Color;
use raylib::init;
use raylib::prelude::RaylibDraw;
use tokio::sync::watch::{Sender,Receiver,channel};
use socketcan::{CanSocket, Socket};
use socketcan::BlockingCan;

use super::messages::*;

type Volt = f32;
type Amper= f32;
type Temp = f32;

struct BmsCell{
    volt: Volt,
    rx: Receiver<Volt>
}

struct BmsTemp{
    temp: Temp,
    rx: Receiver<Temp>
}

static RUN: atomic::AtomicBool = atomic::AtomicBool::new(true);

#[allow(unused)]
pub struct BmsLvGui<const N : usize>{
    tx_channels: [Sender<Volt>;N],
    tx_channel_2: [Sender<Temp>;N],
    min_volt_tx: Sender<Volt>,
    max_volt_tx: Sender<Volt>,
    avg_volt_tx: Sender<Volt>,
    tot_volt_tx: Sender<Volt>,
    current_tx: Sender<Amper>,
    min_temp_tx: Sender<Amper>,
    max_temp_tx: Sender<Amper>,
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

        let channels : [_; N] = std::array::from_fn(|_|{
            let (tx,rx) = channel::<Volt>(0.0);
            (tx,rx)
        });

        let channel_2: [_;N] = std::array::from_fn(|_| {
            let (tx, rx) = channel::<Temp>(0.0);
            (tx,rx)
        });

        let (min_volt_channel_tx, min_volt_channel_rx) =  channel::<Volt>(0.0);
        let (max_volt_channel_tx, max_volt_channel_rx) =  channel::<Volt>(0.0);
        let (avg_volt_channel_tx, avg_volt_channel_rx) =  channel::<Volt>(0.0);
        let (tot_volt_channel_tx, tot_volt_channel_rx) =  channel::<Volt>(0.0);
        let (current_channel_tx, current_channel_rx) =  channel::<Volt>(0.0);
        let (min_temp_channel_tx, min_temp_channel_rx) =  channel::<Volt>(0.0);
        let (max_temp_channel_tx, max_temp_channel_rx) =  channel::<Volt>(0.0);

        let tx_channels = std::array::from_fn(|i|channels[i].0.clone());
        let tx_channel_2 = std::array::from_fn(|i|channel_2[i].0.clone());


        tokio::spawn(async move{
            let mut cells : [_;N] = std::array::from_fn(|i|BmsCell{ volt: 0.0, rx: channels[i].1.clone() });
            let mut min_volt= BmsCell{ volt: 0.0, rx: min_volt_channel_rx};
            let mut max_volt= BmsCell{ volt: 0.0, rx: max_volt_channel_rx};
            let mut avg_volt= BmsCell{ volt: 0.0, rx: avg_volt_channel_rx};
            let mut tot_volt= BmsCell{ volt: 0.0, rx: tot_volt_channel_rx};
            let mut temps : [_;N] = std::array::from_fn(|i|BmsTemp{ temp: 0.0, rx: channel_2[i].1.clone() });


            let (mut rl, rt) =init()
                .title(title)
                .width(w)
                .height(h)
                .build();


            while !rl.window_should_close()
            {
                let mut row = 0;
                let mut colomn = cell_heidth;
                let mut d =rl.begin_drawing(&rt);

                for (i, temp) in temps.iter_mut().enumerate() {
                    let mut temp_val = format!("{:.1}{}", temp.temp, " C");
                    let mut temp_color = Color::RED;

                    if let Ok(true) = temp.rx.has_changed() {
                        temp.temp = *temp.rx.borrow_and_update();
                    }

                    d.clear_background(Color::LIGHTGRAY);
                    if temp.temp > 15.0 && temp.temp < 58.0 {
                        temp_color = Color::GREENYELLOW;
                    }

                    d.draw_rectangle(row, colomn, cell_width, cell_heidth, temp_color);
                    d.draw_rectangle_lines(row, colomn, cell_width, cell_heidth, Color::WHITE);
                    d.draw_text(&temp_val, row + (cell_width/2) - 64, colomn + (cell_heidth/2), 32, Color::BLACK);

                    row+=cell_width;
                }


                colomn += cell_heidth;
                row=0;
                for cell in cells.iter_mut()
                {
                    let mut cell_color = Color::RED;
                    let mut volt_val = format!("{:.1}{}", cell.volt, " mV");

                    if let Ok(true) = cell.rx.has_changed() {
                        cell.volt = *cell.rx.borrow_and_update();
                    }

                    d.clear_background(Color::LIGHTGRAY);
                    if cell.volt > 3200.0 && cell.volt < 4250.0 {
                        cell_color = Color::GREEN;
                    }
                    d.draw_text("Bms Lv Voltages & Temperatures", 130, 50, 32, Color::BLACK);

                    d.draw_rectangle(row, colomn, cell_width, cell_heidth, cell_color);
                    d.draw_rectangle_lines(row, colomn, cell_width, cell_heidth, Color::WHITE);
                    d.draw_text(&volt_val, row + (cell_width/2) - 64, colomn + (cell_heidth/2), 32, Color::BLACK);
                    row += cell_width;
                    if row >= n_cell_in_a_row * cell_width {
                        row = 0;
                        colomn += cell_heidth;
                    }
                }

                let mut min_volt_val = format!("{:.1}{}", min_volt.volt, " mV");
                let mut max_volt_val = format!("{:.1}{}", max_volt.volt, " mV");
                let mut avg_volt_val = format!("{:.1}{}", avg_volt.volt, " mV");
                let mut tot_volt_val = format!("{:.2}{}", tot_volt.volt, " V");

                if let Ok(true) = min_volt.rx.has_changed() {
                    min_volt.volt = *min_volt.rx.borrow_and_update();
                }

                if let Ok(true) = max_volt.rx.has_changed() {
                    max_volt.volt = *max_volt.rx.borrow_and_update();
                }

                if let Ok(true) = avg_volt.rx.has_changed() {
                    avg_volt.volt = *avg_volt.rx.borrow_and_update();
                }

                if let Ok(true) = tot_volt.rx.has_changed() {
                    tot_volt.volt = *tot_volt.rx.borrow_and_update();
                }

                d.draw_rectangle(row, colomn, cell_width, cell_heidth, Color::CORAL);
                d.draw_rectangle_lines(row, colomn, cell_width, cell_heidth, Color::WHITE);
                d.draw_text("Min volt:", row + (cell_width/2) - 64, colomn, 32, Color::BLACK);
                d.draw_text(&min_volt_val, row + (cell_width/2) - 64, colomn + (cell_heidth/2), 32, Color::BLACK);

                row+=cell_width;

                d.draw_rectangle(row, colomn, cell_width, cell_heidth, Color::CORAL);
                d.draw_rectangle_lines(row, colomn, cell_width, cell_heidth, Color::WHITE);
                d.draw_text("Max volt:", row + (cell_width/2) - 64, colomn, 32, Color::BLACK);
                d.draw_text(&max_volt_val, row + (cell_width/2) - 64, colomn + (cell_heidth/2), 32, Color::BLACK);

                row+=cell_width;

                d.draw_rectangle(row, colomn, cell_width, cell_heidth, Color::CORAL);
                d.draw_rectangle_lines(row, colomn, cell_width, cell_heidth, Color::WHITE);
                d.draw_text("Avg volt:", row + (cell_width/2) - 64, colomn, 32, Color::BLACK);
                d.draw_text(&avg_volt_val, row + (cell_width/2) - 64, colomn + (cell_heidth/2), 32, Color::BLACK);

                row+=cell_width;

                d.draw_rectangle(row, colomn, cell_width, cell_heidth, Color::CORAL);
                d.draw_rectangle_lines(row, colomn, cell_width, cell_heidth, Color::WHITE);
                d.draw_text("Tot volt:", row + (cell_width/2) - 64, colomn, 32, Color::BLACK);
                d.draw_text(&tot_volt_val, row + (cell_width/2) - 64, colomn + (cell_heidth/2), 32, Color::BLACK);

            }
            RUN.store(false, atomic::Ordering::Relaxed);
        });

        Self
        {
            tx_channels,
            tx_channel_2,
            min_volt_tx: min_volt_channel_tx,
            max_volt_tx: max_volt_channel_tx,
            avg_volt_tx: avg_volt_channel_tx,
            tot_volt_tx: tot_volt_channel_tx,
            current_tx: current_channel_tx,
            min_temp_tx: min_temp_channel_tx,
            max_temp_tx: max_temp_channel_tx,
            socket_can: can_node
        }
    }

    pub fn update_min_volt(&mut self, volt: Volt){
        let _ =self.min_volt_tx.send(volt);
    }

    pub fn update_max_volt(&mut self, volt: Volt){
        let _ =self.max_volt_tx.send(volt);
    }

    pub fn update_avg_volt(&mut self, volt: Volt){
        let _ =self.avg_volt_tx.send(volt);
    }

    pub fn update_tot_volt(&mut self, volt: Volt){
        let _ =self.tot_volt_tx.send(volt);
    }

    pub fn update_current(&mut self, amper: Amper){
        let _ =self.current_tx.send(amper);
    }

    pub fn update_min_temp(&mut self, amper: Amper){
        let _ =self.min_temp_tx.send(amper);
    }

    pub fn update_max_temp(&mut self, amper: Amper){
        let _ =self.max_temp_tx.send(amper);
    }

    pub fn update_cell(&mut self, cell: usize, volt: Volt){
        let _ =self.tx_channels[cell].send(volt);
        
    }

    pub fn update_temp(&mut self, temp_index: usize, temp: Temp){
        let _ =self.tx_channel_2[temp_index].send(temp);
        
    }

    pub fn update(&mut self){

        let cell_enable = BmsLvCellControl::new(true).ok().unwrap();
        let id = socketcan::StandardId::new(BmsLvCellControl::MESSAGE_ID.try_into().unwrap()).unwrap();
        let frame = socketcan::frame::CanFrame::new(id, cell_enable.raw()).unwrap();
        let _ = self.socket_can.transmit(&frame);


        while RUN.load(atomic::Ordering::Relaxed){
            self.socket_can.set_read_timeout(Duration::from_millis(500));
            let frame = self.socket_can.receive();

            if let Ok(mex) = frame {
                if let socketcan::Id::Standard(standard_id) = mex.id(){
                    let mex = Messages::from_can_message(standard_id.as_raw().into(), mex.data());
                    if let Ok(mex) = mex {
                        match mex {
                            Messages::BmsLvCell1(data) => {
                                self.update_cell(0, data.cell_0());
                                self.update_cell(1, data.cell_1());
                                self.update_cell(2, data.cell_2());
                                self.update_cell(3, data.cell_3());
                            },
                            Messages::BmsLvCell2(data) => {
                                self.update_cell(4, data.cell_4());
                                self.update_cell(5, data.cell_5());
                                self.update_cell(6, data.cell_6());
                                self.update_cell(7, data.cell_7());
                            },
                            Messages::BmsLvCell3(data) => {
                                self.update_cell(8, data.cell_8());
                                self.update_cell(9, data.cell_9());
                                self.update_cell(10, data.cell_10());
                                self.update_cell(11, data.cell_12());
                            },
                            Messages::BmsLv1(data) =>
                            {
                                self.update_min_volt(data.min_volt());
                                self.update_max_volt(data.max_volt());
                                self.update_avg_volt(data.avg_volt());
                                self.update_tot_volt(data.tot_volt());
                            },
                            Messages::BmsLvTemps(data) =>
                            {
                                self.update_temp(0, data.temp_0());
                                self.update_temp(1, data.temp_1());
                                self.update_temp(2, data.temp_2());
                                self.update_temp(3, data.temp_3());

                            },
                            _ => println!("ignored mex"),
                        };
                    }
                }
            }
        }

        let cell_enable = BmsLvCellControl::new(false).ok().unwrap();
        let id = socketcan::StandardId::new(BmsLvCellControl::MESSAGE_ID.try_into().unwrap()).unwrap();
        let frame = socketcan::frame::CanFrame::new(id, cell_enable.raw()).unwrap();
        let _ =self.socket_can.transmit(&frame);
    }
}
