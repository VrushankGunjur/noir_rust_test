use noir_rs::prove;
use noir_rs::verify;
use std::fs::read_to_string;

use acvm::{
    acir::native_types::{Witness, WitnessMap},
    FieldElement,
};

use num_bigint::{BigInt, Sign, ToBigInt};

fn main() {
    // prove
    let mut initial_witness = WitnessMap::new();

    // sk
    //initial_witness.insert(Witness(1), FieldElement::from_hex("0x00000000000000000000000000000000000000000000000000000000000046B3").unwrap());
    initial_witness.insert(Witness(16), FieldElement::from(18099_u128));
    //FieldElement::zero();
    // cts
    initial_witness.insert(Witness(2), FieldElement::from_hex("0x00ee1ef97f8a061cb6cf8b664f267888e644d5f2f8b3ea33acce4dda65d3c5c6").unwrap());
    initial_witness.insert(Witness(3), FieldElement::from_hex("0x150b67cc89afadb58f877a1353eda2f52838cde52455daaaaf4e54441afdcd24").unwrap()); 
    initial_witness.insert(Witness(4), FieldElement::from_hex("0x172dba8d231345b865223308fe44ba307c159de512787257113e2a7137f831b4").unwrap()); 
    initial_witness.insert(Witness(5), FieldElement::from_hex("0x25316c6b88a089801a57e442545e52c559cba980f84f44a8fca45c0f821c5cd7").unwrap()); 
    initial_witness.insert(Witness(6), FieldElement::from_hex("0x0f9f02a7711587226f5b124c6f6df35b9acdd3fb5c59fbef8769f4bf6f99c6ec").unwrap()); 
    initial_witness.insert(Witness(7), FieldElement::from_hex("0x0f7d4b13668486bcbbf5de4c42bbb33738b891e824b6deab3f4055c94c340a59").unwrap()); 
    initial_witness.insert(Witness(8), FieldElement::from_hex("0x10c347ae3592776a678f9fc29ecd029f8297367209a4f28f7cb49b09bffd4145").unwrap()); 
    initial_witness.insert(Witness(9), FieldElement::from_hex("0x113f8246f336e8d375c9daab552816825f481c210226f307bc863e017642566c").unwrap()); 
    initial_witness.insert(Witness(10), FieldElement::from_hex("0x0d71deba24a8bb87ba9a7d01be8668595b607b96a49a4fae5f9c955ba651bd53").unwrap()); 
    initial_witness.insert(Witness(11), FieldElement::from_hex("0x1ae06a2f8af0069ebf78d1d0a437c7e2195972fb803202db511ce65b40b807d6").unwrap()); 
    initial_witness.insert(Witness(12), FieldElement::from_hex("0x2c6bfc7fe056ed38e26ec136ec8aec5f63ecc4b52c44afba967649cf1e6e2311").unwrap()); 
    initial_witness.insert(Witness(13), FieldElement::from_hex("0x1ac7675df6265f6e12d1c79a2b3b6658a0d46a320fba497ad0b817f9b19e0f21").unwrap()); 

    // msg
    //initial_witness.insert(Witness(14), FieldElement::from_hex("0x0000000000000000000000000000000000000000000000000000000000000032").unwrap()); 
    //initial_witness.insert(Witness(15), FieldElement::from_hex("0x0000000000000000000000000000000000000000000000000000000000000014").unwrap()); 
    //initial_witness.insert(Witness(16), FieldElement::from_hex("0x0000000000000000000000000000000000000000000000000000000000000003").unwrap()); 
    initial_witness.insert(Witness(14), FieldElement::from(50_u128));
    initial_witness.insert(Witness(15), FieldElement::from(20_u128));
    initial_witness.insert(Witness(16), FieldElement::from(3_u128));

    let bytecode = read_to_string("bytecode.txt").unwrap().trim().to_string();
    //println!("{}", bytecode);
    // read the bytecode from a file
    let (proof, vk) = prove(bytecode.clone(), initial_witness).unwrap();
    let verdict = verify(bytecode.clone(), proof, vk).unwrap();
    println!("Verdict: {}", verdict);
}