use egui::plot::{Line, Plot, PlotPoints};
use egui::{RichText, Color32};

pub enum PlotType {
    Function2d,
    Equation2d,
    Parametric2d,
    High2d,
    Scatter2d,
    Linear2d,
    Histogram2d,
    Parametric3d,
    Equation3d,
}

pub enum PlotProvider{
    Egui,
    Plotters,
    Poloto
}

impl crate::Plotter {

    //////// Input forms for different plot types ////////

    pub fn input(&mut self, ui: &mut egui::Ui){

        match self.plot_type {

            // 2D
            PlotType::Function2d => {
                self.input_function2d(ui);
            },
            PlotType::Equation2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotType::Parametric2d => {
                self.input_parametric2d(ui);
            },
            PlotType::High2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },

            // Data
            PlotType::Scatter2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotType::Linear2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotType::Histogram2d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },

            // 3D

            PlotType::Parametric3d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotType::Equation3d => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn input_function2d(&mut self, ui: &mut egui::Ui) {

        self.input_equation(ui);
        self.input_uniform_grid(ui);
    }

    fn input_parametric2d(&mut self, ui: &mut egui::Ui) {

        ui.horizontal(|ui|{
            self.input_equation(ui);
            self.input_equation(ui);
        });
        self.input_uniform_grid(ui);
    }

    //////// Common patterns ////////

    fn input_uniform_grid(&mut self, ui: &mut egui::Ui){
        ui.horizontal(|ui| {
            ui.label("Lower limit: ");
            if ui.text_edit_singleline(&mut self.lower_limit).changed() {
                self.are_data_computed = false;
            };
        });

        ui.horizontal(|ui| {
            ui.label("Higher limit: ");
            if ui.text_edit_singleline(&mut self.higher_limit).changed(){
                self.are_data_computed = false;
            };
        });

        ui.horizontal(|ui| {
            ui.label("Intervals amount: ");
            if ui.text_edit_singleline(&mut self.intervals_amount).changed(){
                self.are_data_computed = false;
            };
        });
    }

    fn input_equation(&mut self, ui: &mut egui::Ui){
        ui.horizontal(|ui| {
            ui.label("Equation: ");
            if ui.text_edit_singleline(&mut self.equation).changed(){
                self.are_data_computed = false;
            }
        });
    }

    //////// Plots for different plot types and providers ////////

    pub fn plot(&mut self, ui: &mut egui::Ui){
        match self.plot_type {

            // 2D
            PlotType::Function2d => {
                self.plot_function2d(ui);
            },
            PlotType::Equation2d => {
                self.plot_equation2d(ui);
            },
            PlotType::Parametric2d => {
                self.plot_parametric2d(ui);
            },
            PlotType::High2d => {
                self.plot_high2d(ui);
            },

            // Data
            PlotType::Scatter2d => {
                self.plot_scatter2d(ui);
            },
            PlotType::Linear2d => {
                self.plot_linear2d(ui);
            },
            PlotType::Histogram2d => {
                self.plot_histogram2d(ui);
            },

            // 3D

            PlotType::Parametric3d => {
                self.plot_parametric3d(ui);
            },
            PlotType::Equation3d => {
                self.plot_equation3d(ui);
            }
        }
    }

    //////// Plot functions for type ////////

    fn plot_function2d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                self.plot_function2d_egui(ui);
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn plot_equation2d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn plot_parametric2d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                self.plot_parametric2d_egui(ui);
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn plot_high2d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn plot_scatter2d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn plot_linear2d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn plot_histogram2d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn plot_parametric3d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    fn plot_equation3d(&mut self, ui: &mut egui::Ui){
        match self.plot_provider {
            PlotProvider::Egui => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Poloto => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            },
            PlotProvider::Plotters => {
                ui.label(RichText::new("Not implemented.").color(Color32::RED));
                //TODO
            }
        }
    }

    //////// Specific plot functions for providers and types ////////

    fn plot_function2d_egui(&mut self, ui: &mut egui::Ui) {

        let (a, b, n) = self.parse_equal_grid();

        self.compute_equal_grid(a, b, n);

        self.plot_points_egui(ui, n);
    }

    fn plot_parametric2d_egui(&mut self, ui: &mut egui::Ui){
        let (a, b, n) = self.parse_equal_grid();

        self.compute_parametric_equal_grid(a, b, n);

        self.plot_points_egui(ui, n);
    }

    //////// Common patterns ////////

    fn plot_points_egui(&mut self, ui: &mut egui::Ui, n: u32){
        let sin: PlotPoints = (0..n + 1).map(|i| {
            self.points[i as usize]
        }).collect();

        let line = Line::new(sin);

        Plot::new("Plot")
            .width(self.width - 15.0)
            .height(self.height - 140.0)
            .data_aspect(1.0)
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}