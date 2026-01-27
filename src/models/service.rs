

pub struct Service{
    pub id:String,
    pub name:String,
    pub online:String,
    pub errors:String,
    pub time:String
}

impl Service{
    pub fn new(id:String,
    name:String,
    online:String, errors:String,
     time:String)->Self{
        Self{id, name, online, errors, time}
    }
}