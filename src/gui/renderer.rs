use raylib::init;
use tokio::sync::watch::{Sender,Receiver,channel};

struct BmsCell{
    volt: u32,
    rx: Receiver<u32>
}


#[allow(unused)]
pub struct BmsLvGui<const N : usize>{
    tx_channels: [Sender<u32>;N]
}

#[allow(unused)]
impl<const N:usize> BmsLvGui<N> {
    
    pub async fn new(title: &'static str, w: i32, h: i32) -> Self
    {
        let channels : [_; N] = std::array::from_fn(|_|{
            let (tx,rx) = channel(12);
            (tx,rx)
        });

        let tx_channels = std::array::from_fn(|i|channels[i].0.clone());

        tokio::spawn(async move{
            let mut cells : [_;N] = std::array::from_fn(|i|BmsCell{ volt: 0, rx: channels[i].1.clone() });
            let (mut rl, rt) =init()
                .title(title)
                .width(w)
                .height(h)
                .build();


            while !rl.window_should_close()
            {
                for cell in cells.iter_mut()
                {
                    if let Ok(true) = cell.rx.has_changed() {
                        cell.volt = *cell.rx.borrow_and_update();
                    }
                }
                let mut d =rl.begin_drawing(&rt);

            }
        });

        Self{tx_channels}
    }

    pub fn update_cell(&mut self, cell: usize, volt: u32){
        self.tx_channels[cell].send(volt).unwrap()
        
    }
}
