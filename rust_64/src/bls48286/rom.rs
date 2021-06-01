/*
 * Copyright (c) 2012-2020 MIRACL UK Ltd.
 *
 * This file is part of MIRACL Core
 * (see https://github.com/miracl/core).
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::arch::Chunk;
use crate::bls48286::big::NLEN;

// Base Bits= 60
// bls48286 Modulus

pub const MODULUS: [Chunk; NLEN] = [
    0xE7502B9209C345B,
    0xF641C4528E352D9,
    0xC8C7E1AC04809AA,
    0xD7B0201C8145A86,
    0x2972C531EC7B,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x61F9539D245AF2C,
    0xA1991A6E51410D8,
    0x1603A99FC661885,
    0xC8A2CE485CD7822,
    0xD54A6F0B25E,
];
pub const ROI: [Chunk; NLEN] = [
    0xE7502B9209C345A,
    0xF641C4528E352D9,
    0xC8C7E1AC04809AA,
    0xD7B0201C8145A86,
    0x2972C531EC7B,
];
pub const SQRTM3: [Chunk; NLEN] = [
    0xCD7A472E6524A9C,
    0x535B8141D328998,
    0x758989A7084EB33,
    0xD870A3222987B99,
    0x2971C25478F5,
];
pub const CRU: [Chunk; NLEN] = [
    0x5A6539603773F7B,
    0x24CEA2CA30AEE39,
    0x1F28B5A98667A6F,
    0xD810619F5566B10,
    0x297243C332B8,
];
pub const MCONST: Chunk = 0x227E5D8F6EA242D;
pub const FRA: [Chunk; NLEN] = [
    0xEA8695F3FC90183,
    0xC556BED1BD3E936,
    0xD903EF0268F09A7,
    0x114E8AEC2FE0043,
    0xD9417EC522C,
];
pub const FRB: [Chunk; NLEN] = [
    0xFCC9959E0D332D8,
    0x30EB0580D0F69A2,
    0xEFC3F2A99B90003,
    0xC66195305165A42,
    0x1BDEAD459A4F,
];
pub const TWK: [Chunk; NLEN] = [
    0xA7D9735C3D05DCC,
    0x1997054542A70B9,
    0xBCCBD14CBAA2C6A,
    0x33519F37367221F,
    0x1D20EDEF448A,
];

//*** rom curve parameters *****
// Ate Bits= 17
// G2 Table size= 20
pub const CURVE_COF_I: isize = 62958;
pub const CURVE_COF: [Chunk; NLEN] = [0xF5EE, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B_I: isize = 10;
pub const CURVE_B: [Chunk; NLEN] = [0xA, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0xFC411B2FD612C81,
    0x98F28822F29701F,
    0xB262A94FBE4FE22,
    0x9EB01535FC9EDE6,
    0x86BC,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xC897EA3095D59E0,
    0xDD97475CEFBA15D,
    0x544741A4E84D19,
    0xDB88FB476C0F04C,
    0x6972433D120,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0xEEF69F970ABEB43,
    0x237CE6B580E3E2E,
    0x3308DEE53B0AC1F,
    0x6E2D78897F979AA,
    0x2038E40A1E65,
];
pub const CURVE_HTPC: [Chunk; NLEN] = [0x1, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_BNX: [Chunk; NLEN] = [0xF5EF, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_PXAAA: [Chunk; NLEN] = [
    0x8416349856106B5,
    0xCC3B1CECC4CC0FE,
    0xA952DACAFD9F51A,
    0x71D2A27B04CAEB7,
    0x22ACFD40531C,
];
pub const CURVE_PXAAB: [Chunk; NLEN] = [
    0xDECD1BD896D1A55,
    0x427718553891DA9,
    0xC8A32592FEF56B9,
    0x85A58D250C873E4,
    0x18DBB9996B5C,
];
pub const CURVE_PXABA: [Chunk; NLEN] = [
    0x8811A3A67D7A35E,
    0x9FAF4E8CC58A603,
    0xC19A506B52AF41C,
    0x7C317AE2BDE1D4D,
    0xA8292D5DE0A,
];
pub const CURVE_PXABB: [Chunk; NLEN] = [
    0x28E8520C2F023E8,
    0xF191B5CF33D2D0A,
    0xE725CF5DFB62D89,
    0x94CD8BBB1F92E81,
    0x202BE0D55FAC,
];
pub const CURVE_PXBAA: [Chunk; NLEN] = [
    0x8B98D5B8F14CB9B,
    0x1A17D62FE2554D8,
    0xC71EF6D551B96A2,
    0xF76284B52259647,
    0x17527369A1E3,
];
pub const CURVE_PXBAB: [Chunk; NLEN] = [
    0xBF6B286CB60DD54,
    0xB8532B9371A2443,
    0x638D54D7E6BE9C0,
    0x59F346C42D3760D,
    0x1101EA485E9D,
];
pub const CURVE_PXBBA: [Chunk; NLEN] = [
    0x20A5B0AB9D776DA,
    0xA657276121135FA,
    0xDE41458BE4ADABA,
    0xE41FCEF60E4AAD4,
    0xF606FCE261F,
];
pub const CURVE_PXBBB: [Chunk; NLEN] = [
    0xF3FE23828B50A8E,
    0x77554EF2609CEB5,
    0x72CE1923A1EC9A5,
    0x19566C5168C0002,
    0x1F0B30E6920D,
];
pub const CURVE_PYAAA: [Chunk; NLEN] = [
    0x26CD5FD67E988BE,
    0x1FE5EF8E978BCEE,
    0x39B2544F14E8B3,
    0xCD557C7F38694FF,
    0x21F0F8CDA962,
];
pub const CURVE_PYAAB: [Chunk; NLEN] = [
    0x778566D0DF11B92,
    0x83331801EBC5E08,
    0xB840129F0DB40CB,
    0xD7A943A25E15861,
    0xE01C1FEC6FE,
];
pub const CURVE_PYABA: [Chunk; NLEN] = [
    0x47CFA67ABFDD06A,
    0x4BCDAB08CFE5925,
    0x55AE0A98D2EB8E6,
    0xBEA2D9EBB35EFF2,
    0x12C0FFB8C3D6,
];
pub const CURVE_PYABB: [Chunk; NLEN] = [
    0xB3DDFD3788A47F4,
    0x195F9A36106B328,
    0x4B28538511F157D,
    0xFF3A1DDBDD352B3,
    0x156831E55BDF,
];
pub const CURVE_PYBAA: [Chunk; NLEN] = [
    0xB6EDB7345E39E77,
    0x4C8190FF9538C4C,
    0xF647CDEF2511932,
    0x84EB4F5EF4193ED,
    0x738E40AA018,
];
pub const CURVE_PYBAB: [Chunk; NLEN] = [
    0x3282814364975C6,
    0xBE24AD9CA88C0B0,
    0x1969B0DAE613723,
    0x24391C50369BE1A,
    0x13658766896E,
];
pub const CURVE_PYBBA: [Chunk; NLEN] = [
    0xBD339ABC1BF2D7C,
    0xB713EF1B3AA2440,
    0x6145A099D3837F1,
    0xC9C5EE34A88F81B,
    0x1FB0A8375143,
];
pub const CURVE_PYBBB: [Chunk; NLEN] = [
    0xAA9A0D1E51130A6,
    0xEB9E708362553DA,
    0xEDA47676A7C687E,
    0x9E9A1777B2D6330,
    0x193F378DC6AC,
];

pub const USE_GLV: bool = true;
pub const USE_GS_G2: bool = true;
pub const USE_GS_GT: bool = true;
pub const GT_STRONG: bool = false;
