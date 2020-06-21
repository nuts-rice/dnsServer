#derive[(Clone, Debug)]
pub struct DnsHeader {

	//Field : type
	pub id: u16,
	pub recursion_desired:bool, //1 bit
	pub truncated_message:bool, //1 bit
	pub authoritative_answer: bool, //1 bit
	pub opcode: u8, //4 bits
	pub response: bool, // 1 bit

	pub rescode: ResultCode, //4 bits
	pub checking_disabled: bool, //1 bit
	pub authed_data: bool, //1 bit
	pub z: bool, //1 bit
	pub recursion_available: bool, //1 bit

	pub questions: u16, //16 bits
	pub answers: u16, //16 bits
	pub authoritative_entries: u16, //16 bits
	pub resource_entries: u16 //16 bits

}
impl DnsHeader {
	pub fn new() -> DnsHeader{
		DnsHeader {
			id:0,

			recursion_desired: false,
			truncated_message: false,
			authoritative_answer: false,
			opcode:0,
			response: false,

			rescode: ResultCode :: NOERROR,
			checking_disabled: false,
			authed_data: false,
			z: false,
			recursion_available: false,

			questions:0,
			answers:0,
			authoritative_entries:0,
			resource_entries:0,
			



		}
	}
}