use eframe::egui;
use rand::Rng;
use chrono::prelude::*;



fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 360.0]),
        ..Default::default()
    };


    let _ = eframe::run_native(
        "Random Number Generator",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light()); // 设置默认为浅色模式[^1^]
            Ok(Box::new(RandomNumberGeneratorApp::default()))
        }),
    );
}

#[derive(Default)]
pub struct RandomNumberGeneratorApp {
    lower_bound: i64,
    upper_bound: i64,
    num_to_generate: usize,
    allow_duplicates: bool,
    generated_numbers: Vec<i64>,

}



impl eframe::App for RandomNumberGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {


            //白天夜晚切换与标题
            ui.horizontal(|ui|{
                ui.heading("Random Number Generator");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("Dark").clicked(){
                        ctx.set_visuals(egui::Visuals::dark());
                    }
                    if ui.button("Light").clicked(){
                        ctx.set_visuals(egui::Visuals::light());
                    }
                });

            });
            // 输入下限
            ui.horizontal(|ui| {
                ui.label("Lower Bound:");
                ui.add(egui::DragValue::new(&mut self.lower_bound)); // 使用 DragValue
            });

            // 输入上限
            ui.horizontal(|ui| {
                ui.label("Upper Bound:");
                ui.add(egui::DragValue::new(&mut self.upper_bound)); // 使用 DragValue
            });

            // 输入生成数量
            ui.horizontal(|ui| {
                ui.label("Number of Random Numbers to Generate:");
                ui.add(egui::DragValue::new(&mut self.num_to_generate).range(1..=1000)); // 使用 DragValue
            });

            // 是否允许重复值
            ui.checkbox(&mut self.allow_duplicates, "Allow Duplicates");

            //时间
            let local_time = Local::now();
            let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(formatted_time);
            });
            ctx.request_repaint();


            // 显示生成的随机数
            ui.separator(); //分割线

            ui.horizontal(|ui|{
                ui.heading("Generated Numbers");
                // 生成随机数
                if ui.button("Generate").clicked() {
                    self.generated_numbers.clear(); // 清空之前的结果
                    let mut rng = rand::rngs::ThreadRng::default(); // 使用新的 API

                    //检查下限是否大于上限
                    if self.lower_bound > self.upper_bound{
                        self.generated_numbers = vec![];
                    } else {
                        if self.allow_duplicates {
                            // 允许重复值，直接生成随机数
                            for _ in 0..self.num_to_generate {
                                let num = rng.random_range(self.lower_bound..=self.upper_bound);
                                self.generated_numbers.push(num);
                            }
                        } else {
                            // 不允许重复值
                            let range = self.upper_bound - self.lower_bound + 1;
                            if self.num_to_generate > range as usize {
                                // 如果生成数量超过范围，无法生成不重复的随机数
                                self.generated_numbers = vec![];
                            } else {
                                // 生成不重复的随机数
                                let mut unique_numbers = Vec::new();
                                while unique_numbers.len() < self.num_to_generate {
                                    let num = rng.random_range(self.lower_bound..=self.upper_bound);
                                    if !unique_numbers.contains(&num) {
                                        unique_numbers.push(num);
                                    }
                                }

                                self.generated_numbers = unique_numbers;
                            }
                        }
                    }
                }
                if ui.button("Clear").clicked() {
                    self.generated_numbers.clear();
                }
            });
                
            ui.label(self.generated_numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
            
            //项目地址及许可证
            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::Center),
                |ui| {
                    
                    ui.label("https://github.com/Daihongyi/random-tool-github");
                },
            );

            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::RIGHT),

                           |ui| {

                               ui.label("MPL2.0");
                           },
            );
            
        });
    }
}
