/*
    KMS-ECDSA

    Copyright 2018 by Kzen Networks

    This file is part of KMS library
    (https://github.com/KZen-networks/kms)

    Cryptography utilities is free software: you can redistribute
    it and/or modify it under the terms of the GNU General Public
    License as published by the Free Software Foundation, either
    version 3 of the License, or (at your option) any later version.

    @license GPL-3.0+ <https://github.com/KZen-networks/kms/blob/master/LICENSE>
*/
use cryptography_utils::BigInt;
use multi_party_ecdsa::protocols::two_party_ecdsa::lindell_2017::party_one::Signature;
pub trait ManagementSystem<PK, SK> {
    fn rotate(self, &BigInt) -> Self;
   // fn get_child(&self, index: BigInt, height: BigInt) -> (PK,SK);

}