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
//this will take domain names, yes

//here we return the pos
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
//ok we READ a single byte and move one step forward here
fn read(&mut self) -> Result<u8>
{
    //Remember, DNS packets are traditionally limited to 512 bytes!
    //So, if we reach a position greater or equal to 512, we return a new Error of invalid input
    if self.pos >= 512
    {
        return Err(Error:: new (ErrorKind:: InvalidInput, "End of Buffer!"));
    }

    let res = self.buf[self.pos];
    self.pos += 1;

    Ok(res)
}

//here we fetch data at a given position
//while also not mdifying position within stack
fn get(&mut self, pos: usize) -> Result<u8>
{
    if pos >= 512
    {
        return Err(Error:: new (ErrorKind:: InvalidInput, "End of Buffer!"));
    }
    Ok(self.buf[pos])
}

//fetch range of data i think
fn get_range(&mut self, start:usize, len: usize) -> Result<&[u8]>
{
    if start + len >= 512
    {
        return Err(Error :: new (ErrorKind :: InvalidInput, "End of Buffer!"));

    }
    Ok(&self.buf[start..start+len as usize])
}
//methods for reading unsigned 16 and 32 from buffer
//while moving forward 2 to 4 bytes

fn read_u16(&mut self) -> Result<u16>
{
    let res = ((try!(self.read()) as u16) << 8 | 
                (try!(self.read()) as u16);
    Ok(res)
}

fn read_u32(&mut self ) -> Result<u32>
{
    let res = ((try!(self.read()) as u32) << 24) |
    ((try!(self.read()) as u32) << 16) |
    ((try!(self.read()) as u32) << 8) |
    ((try!(self.read()) as u32) << 0);
    Ok(res)
}

//okay dokie here be dragons. read query name
//reading domain names, take labels
//example: [3]www[6]google[3]com
//then append www.google.come to outstring
fn read_qname(&mut self, outstr: &mut String ) -> Result<()>
{
    //keep position locally vs using position in struct
    //we can move shared posistion to a point past current qname
    //while keeping track of pos in current qname
    //using pos variable
    let mut pos = self.pos();

    //have we jumped OR no
    let mut jumped = false;

    //what is a delimeter?
    //we append this for each label 
    //since we dont have dots at the beginning of each domain name
    //we keep this blank for now, 
    //then set to "." at end of first iteration
    let mut delim = "";
    loop
    {
        //right here, we're going to ALWAYS be at the 
        //start of the label. 
        //!!! Note: labels always start with length byte !!!
        let len = try!(self.get(pos));

        //if len has the two most signifigant bit are set
        //this represents a jump to some other offset in packet
        if(len & 0xC0) == 0xC0
        {
            //Update buffer pos to a point past current
            //label 
            if !jumped
            {
                try!(self.seek(pos+2));
            }
            //read another byte, calculate offset and
            //perform the jump by updating local pos variable
            let b2 = try!(self.get(pos+1)) as u16;
            let offset = (((len as u16) ^ 0xC0 << 8) | b2;
            pos = offset as usize;
            //set jumped to true
            jumped = true;
        }
        //base case where we read a single label
        //and appending to the output
        else
        {
            //move a single byte forward to move past length byte
            pos+= 1;

            //domain names are TERMINATED by an empty lenth label of 0 
            //so if len is 0, we finish
            if len == 0
            {
                break;
            }
            //append delimiter to output buffer first
            outstr.push_str(delim);

            //Extract the actual ASCII bytes by their label
            //and append to output buffer

            let str_buffer = try!(self.get_range(pos, len as usize));
            
        }
    }
}
