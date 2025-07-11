use std::str::FromStr;
use thiserror::Error;

// Custom errors for Bitcoin operations
#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("Invalid transaction format")]
    InvalidTransaction,
    #[error("Invalid script format")]
    InvalidScript,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Parse error: {0}")]
    ParseError(String),
}

// Generic Point struct for Bitcoin addresses or coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        // TODO: Implement constructor for Point
        Self { x, y }
    }
}

// Custom serialization for Bitcoin transaction
pub trait BitcoinSerialize {
    fn serialize(&self) -> Vec<u8> {
        // TODO: Implement serialization to bytes
        Vec::new()
    }
}

// Legacy Bitcoin transaction
#[derive(Debug, Clone)]
pub struct LegacyTransaction {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl LegacyTransaction {
    pub fn builder() -> LegacyTransactionBuilder {
        // TODO: Return a new builder for constructing a transaction
        LegacyTransactionBuilder::default()
    }
}

// Transaction builder
pub struct LegacyTransactionBuilder {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Default for LegacyTransactionBuilder {
    fn default() -> Self {
        LegacyTransactionBuilder {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        }
    }
}

impl LegacyTransactionBuilder {
    pub fn new() -> Self {
        // TODO: Initialize new builder by calling default
        Self::default()
    }

    pub fn version(mut self, version: i32) -> Self {
        // TODO: Set the transaction version
        self.version = version;
        self
    }

    pub fn add_input(mut self, input: TxInput) -> Self {
        // TODO: Add input to the transaction
        self.inputs.push(input);
        self
    }

    pub fn add_output(mut self, output: TxOutput) -> Self {
        // TODO: Add output to the transaction
        self.outputs.push(output);
        self
    }

    pub fn lock_time(mut self, lock_time: u32) -> Self {
        // TODO: Set lock_time for transaction
        self.lock_time = lock_time;
        self
    }

    pub fn build(self) -> LegacyTransaction {
        // TODO: Build and return the final LegacyTransaction
        LegacyTransaction {
            version: self.version,
            inputs: self.inputs,
            outputs: self.outputs,
            lock_time: self.lock_time,
        }
    }
}

// Transaction components
#[derive(Debug, Clone)]
pub struct TxInput {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64, // in satoshis
    pub script_pubkey: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub vout: u32,
}

// Simple CLI argument parser
pub fn parse_cli_args(args: &[String]) -> Result<CliCommand, BitcoinError> {
    // TODO: Match args to "send" or "balance" commands and parse required arguments
    if args.is_empty() {
        return Err(BitcoinError::ParseError("Not enough arguments".into()));
    }
    match &args[0][..] {
        "send" if args[1..].len() >= 2 => {
            let amount = u64::from_str(&args[1][..]).map_err(|_| BitcoinError::InvalidAmount)?;
            let address = args[2].clone();
            Ok(CliCommand::Send { amount, address })
        }
        "balance" => Ok(CliCommand::Balance),
        _ => Err(BitcoinError::ParseError("Not enough arguments".into())),
    }
}

pub enum CliCommand {
    Send { amount: u64, address: String },
    Balance,
}

pub fn read_u32(data: &[u8]) -> Result<(u32, usize), BitcoinError> {
    if data.len() >= 4 {
        let num = u32::from_le_bytes(data[..4].try_into().unwrap());
        return Ok((num, 4));
    }
    Err(BitcoinError::InvalidTransaction)
}

pub fn read_u64(data: &[u8]) -> Result<(u64, usize), BitcoinError> {
    if data.len() >= 8 {
        let num = u64::from_le_bytes(data[..8].try_into().unwrap());
        return Ok((num, 8));
    }
    Err(BitcoinError::InvalidTransaction)
}

pub fn read_script(data: &[u8]) -> Result<(Vec<u8>, usize), BitcoinError> {
    let mut next = 0;
    let mut v = Vec::new();

    let (size, off) = read_compact_size(data)?;
    next += off;

    if data[next..].len() >= size as usize {
        let end = next + size as usize;
        v.extend_from_slice(&data[next..end]);
        return Ok((v, end));
    }
    Err(BitcoinError::InvalidTransaction)
}

pub fn read_compact_size(data: &[u8]) -> Result<(u64, usize), BitcoinError> {
    if !data.is_empty() {
        match data[0] {
            x @ 0..=0xfc => return Ok((x.into(), 1)),
            0xfd if data[1..].len() >= 2 => {
                return Ok((
                    u16::from_le_bytes(data[1..=2].try_into().unwrap()).into(),
                    3,
                ));
            }
            0xfe if data[1..].len() >= 4 => {
                return Ok((
                    u32::from_le_bytes(data[1..=4].try_into().unwrap()).into(),
                    5,
                ));
            }
            0xff if data[1..].len() >= 8 => {
                return Ok((u64::from_le_bytes(data[1..=8].try_into().unwrap()), 9));
            }
            _ => return Err(BitcoinError::InvalidTransaction),
        }
    }
    Err(BitcoinError::InvalidTransaction)
}

// pub fn read_input(data: &[u8]) -> Result<(TxInput, usize), BitcoinError> {
//     fn read_outpoint(data: &[u8]) -> Result<(OutPoint, usize), BitcoinError> {
//         if data.len() >= 36 {
//             let mut txid: [u8; 32] = [0; 32];
//             txid.clone_from(&data[0..32].try_into().unwrap());
//             return Ok((
//                 OutPoint {
//                     txid,
//                     vout: u32::from_le_bytes(data[33..=36].try_into().unwrap()),
//                 },
//                 36,
//             ));
//         }
//         Err(BitcoinError::InvalidTransaction)
//     }

//     let mut next = 0;

//     let (previous_output, off) = read_outpoint(data)?;
//     next += off;

//     let (script_sig, off) = read_script(&data[next..])?;
//     next += off;

//     let (sequence, off) = read_u32(&data[next..])?;
//     next += off;

//     Ok((
//         TxInput {
//             previous_output,
//             script_sig,
//             sequence,
//         },
//         next,
//     ))
// }

// pub fn read_output(data: &[u8]) -> Result<(TxOutput, usize), BitcoinError> {
//     let mut next = 0;

//     let (value, off) = read_u64(data)?;
//     next += off;

//     let (script_pubkey, off) = read_script(&data[next..])?;
//     next += off;

//     Ok((
//         TxOutput {
//             value,
//             script_pubkey,
//         },
//         next,
//     ))
// }

// Decoding legacy transaction
impl TryFrom<&[u8]> for LegacyTransaction {
    type Error = BitcoinError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // TODO: Parse binary data into a LegacyTransaction
        // Minimum length is 10 bytes (4 version + 4 inputs count + 4 lock_time)
        let mut next = 0;

        let (version, off) = read_u32(data)?;
        next += off;

        let (input_count, off) = read_u32(&data[next..])?;
        next += off;

        let inputs = Vec::with_capacity(input_count as usize);

        // for _ in 0..input_count {
        //     let (input, off) = read_input(&data[next..])?;
        //     next += off;
        //     inputs.push(input);
        // }

        let (output_count, off) = read_u32(&data[next..])?;
        next += off;

        let outputs = Vec::with_capacity(output_count as usize);

        // for _ in 0..output_count {
        //     let (output, off) = read_output(&data[next..])?;
        //     next += off;
        //     outputs.push(output);
        // }

        let (lock_time, _off) = read_u32(&data[next..])?;
        // next += off;

        Ok(LegacyTransaction {
            version: version as i32,
            inputs,
            outputs,
            lock_time,
        })
    }
}

// Custom serialization for transaction
impl BitcoinSerialize for LegacyTransaction {
    fn serialize(&self) -> Vec<u8> {
        // TODO: Serialize only version and lock_time (simplified)
        let mut v = Vec::new();
        v.extend_from_slice(&self.version.to_le_bytes());
        v.extend_from_slice(&self.lock_time.to_le_bytes());
        v
    }
}
