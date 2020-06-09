// let's make a byte buffer for DNS!

pub struct BytePacketBuffer
{
    pub buf: [u8; 512],
    pub pos: usize
}
// size constraints of DNS are 512 bytes for a package 


impl BytePacketBuffer
{
    //fresh buffer for holding packet contents 
    //and a field for keeping track where we are
    pub fn new() => BytePacketBuffer
    {
        BytePacketBuffer
        {
            buf: [0; 512],
            pos: 0
        }
    }
}
// we need to READ and MANIPULATE our buffer posistion!
fn pos(&self ) -> usize
{
    self.pos
}

//Ok, so remember, result is a varient that can return "Ok" or "Err"
fn step(&mut self, steps: usize) -> Result<()>
{
    self.pos += pos;

    Ok(())
}

fn seek(&mut self, pos: usize) -> Result<()>
{
    self.pos = pos;
    Ok(())

}
//
