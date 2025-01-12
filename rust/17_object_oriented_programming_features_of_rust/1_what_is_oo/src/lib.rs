pub struct AveragedCollection {
    list: Vec<f64>,
    average: f64,
}

impl AveragedCollection {
    pub fn get_average(&self) -> f64 {
        self.average
    }
    pub fn add_item(&mut self, new_item: f64) {
        self.list.push(new_item);
        self.update_average();
    }
    pub fn remove(&mut self) {
        self.list.pop();
        self.update_average();
    }
    fn update_average(&mut self) {
        self.average = self.list.iter().sum::<f64>() as f64 / self.list.len() as f64;
    }
}
