use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct RequestHeapFrameLayout {
    pub bytes: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct SetComputeUnitLimitLayout {
    pub units: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct SetComputeUnitPriceLayout {
    pub micro_lamports: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct SetLoadedAccountsDataSizeLimitLayout {
    pub bytes: u32,
}

#[derive(Debug)]
pub struct Instruction {
    pub name: String,
    pub requestHeapFrameArg: RequestHeapFrameLayout,
    pub setComputeUnitLimitArg: SetComputeUnitLimitLayout,
    pub setComputeUnitPriceArg: SetComputeUnitPriceLayout,
    pub setLoadedAccountsDataSizeLimitArg: SetLoadedAccountsDataSizeLimitLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut instruction_name = String::default();

    let mut requestHeapFrameArg: RequestHeapFrameLayout = RequestHeapFrameLayout::default();
    let mut setComputeUnitLimitArg: SetComputeUnitLimitLayout =
        SetComputeUnitLimitLayout::default();
    let mut setComputeUnitPriceArg: SetComputeUnitPriceLayout =
        SetComputeUnitPriceLayout::default();
    let mut setLoadedAccountsDataSizeLimitArg: SetLoadedAccountsDataSizeLimitLayout =
        SetLoadedAccountsDataSizeLimitLayout::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::try_from_slice(disc_bytes).unwrap();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        0 => {
            instruction_name = String::from("Unused");
        }
        1 => {
            instruction_name = String::from("RequestHeapFrame");
            requestHeapFrameArg = RequestHeapFrameLayout::deserialize(rest_bytes).unwrap();
        }
        2 => {
            instruction_name = String::from("SetComputeUnitLimit");
            setComputeUnitLimitArg = SetComputeUnitLimitLayout::deserialize(rest_bytes).unwrap();
        }
        3 => {
            instruction_name = String::from("SetComputeUnitPrice");
            setComputeUnitPriceArg = SetComputeUnitPriceLayout::deserialize(rest_bytes).unwrap();
        }
        4 => {
            instruction_name = String::from("SetLoadedAccountsDataSizeLimit");
            setLoadedAccountsDataSizeLimitArg =
                SetLoadedAccountsDataSizeLimitLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    let result: Instruction = Instruction {
        name: instruction_name,
        requestHeapFrameArg: requestHeapFrameArg,
        setComputeUnitLimitArg: setComputeUnitLimitArg,
        setComputeUnitPriceArg: setComputeUnitPriceArg,
        setLoadedAccountsDataSizeLimitArg: setLoadedAccountsDataSizeLimitArg,
    };

    return result;
}
