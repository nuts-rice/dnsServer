#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType {
    UNKNOWN(u16),
    A,  //1
}

//implement a new type here: QueryType: we'll need a way to
//represent the record type being queried:
impl QueryType {

    //from a refrence to self to a number
    pub fn to_num(&self) -> read_u16{
        match *self {
            QueryType::UNKNOWN(x) => x,
            QueryType::A => 1,
        }
    }
    //from a number to a refrence to self (QueryType)
    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            _ => QueryType:: UNKNOWN(num),
        }
    }
}
