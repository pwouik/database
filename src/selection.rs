pub struct Selection{
    pub indices:Vec<usize>,
}

impl Selection {
    pub fn empty()->Self{
        Selection{
            indices:Vec::new()
        }
    }
    pub fn new(indices:Vec<usize>)->Self{
        Selection{
            indices
        }
    }
    fn or(&mut self,other:&mut Self)->Self{
        let mut r = self.indices.clone();
        'outer: for i in &other.indices{
            for j in &self.indices{
                if i==j{
                    continue 'outer;
                }
            }
            r.push(*i);
        }
        Selection{
            indices:r,
        }
    }
    fn and(&mut self,other:&mut Self)->Self{
        let mut r = Vec::new();
        for i in &other.indices{
            for j in &self.indices{
                if i==j{
                    r.push(*i);
                    break
                }
            }
        }
        Selection{
            indices:r,
        }
    }
}