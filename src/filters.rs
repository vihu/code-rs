use dsp::fir::FIRFilter;
use dsp::dcblock::DCBlocker;

pub struct TransmitFilter {
    rcf: FIRFilter<RaisedCosineFIR>,
    preemph: FIRFilter<PreemphasisFIR>,
}

impl TransmitFilter {
    pub fn new() -> TransmitFilter {
        TransmitFilter {
            rcf: FIRFilter::new(),
            preemph: FIRFilter::new(),
        }
    }

    pub fn feed(&mut self, s: f32) -> f32 {
        self.preemph.feed(self.rcf.feed(s))
    }
}

pub struct ReceiveFilter {
    deemph: FIRFilter<DeemphasisFIR>,
    dcblock: DCBlocker<f32>,
}

impl ReceiveFilter {
    pub fn new() -> ReceiveFilter {
        ReceiveFilter {
            deemph: FIRFilter::new(),
            dcblock: DCBlocker::new(0.999),
        }
    }

    pub fn feed(&mut self, s: f32) -> f32 {
        self.deemph.feed(self.dcblock.feed(s))
    }
}

/// Construct a FIR filter approximating the frequency response of the "Nyquist Raised
/// Cosine" filter described in the P25 standard.
impl_fir!(RaisedCosineFIR, f32, 121, [
    -0.0000000000000000,
    -0.0002914178875877,
    -0.0006110820215192,
    -0.0009237061350181,
    -0.0011884933789757,
    -0.0013635438839528,
    -0.0014110552136596,
    -0.0013027556276180,
    -0.0010249119463935,
    -0.0005822273129838,
    0.0000000000000000,
    0.0006759457825454,
    0.0013818309223325,
    0.0020409960801970,
    0.0025711800935818,
    0.0028934041068590,
    0.0029416048897079,
    0.0026719763415362,
    0.0020708967035154,
    0.0011603599401470,
    -0.0000000000000000,
    -0.0013149112548725,
    -0.0026597198860545,
    -0.0038906708694845,
    -0.0048585110110598,
    -0.0054242689683386,
    -0.0054757111001610,
    -0.0049427473443866,
    -0.0038100046365077,
    -0.0021249167442686,
    0.0000000000000000,
    0.0023915250143001,
    0.0048269842712816,
    0.0070518954771218,
    0.0088028256783799,
    0.0098336414102615,
    0.0099428305324617,
    0.0089992722326031,
    0.0069637718475148,
    0.0039038822764757,
    -0.0000000000000000,
    -0.0044585796957301,
    -0.0090880926482690,
    -0.0134346962700486,
    -0.0170073191983843,
    -0.0193165798559884,
    -0.0199166735305692,
    -0.0184466738048939,
    -0.0146675521740148,
    -0.0084914165444563,
    0.0000000000000000,
    0.0105497419401663,
    0.0227340119781362,
    0.0359844238881347,
    0.0496228004793832,
    0.0629060521291882,
    0.0750777255961289,
    0.0854221506501576,
    0.0933168001531859,
    0.0982785522408229,
    0.1000000000000000,
    0.0982785522408229,
    0.0933168001531859,
    0.0854221506501576,
    0.0750777255961289,
    0.0629060521291882,
    0.0496228004793832,
    0.0359844238881347,
    0.0227340119781362,
    0.0105497419401663,
    0.0000000000000000,
    -0.0084914165444563,
    -0.0146675521740148,
    -0.0184466738048939,
    -0.0199166735305692,
    -0.0193165798559884,
    -0.0170073191983843,
    -0.0134346962700486,
    -0.0090880926482690,
    -0.0044585796957301,
    -0.0000000000000000,
    0.0039038822764757,
    0.0069637718475148,
    0.0089992722326031,
    0.0099428305324617,
    0.0098336414102615,
    0.0088028256783799,
    0.0070518954771218,
    0.0048269842712816,
    0.0023915250143001,
    0.0000000000000000,
    -0.0021249167442686,
    -0.0038100046365077,
    -0.0049427473443866,
    -0.0054757111001610,
    -0.0054242689683386,
    -0.0048585110110598,
    -0.0038906708694845,
    -0.0026597198860545,
    -0.0013149112548725,
    -0.0000000000000000,
    0.0011603599401470,
    0.0020708967035154,
    0.0026719763415362,
    0.0029416048897079,
    0.0028934041068590,
    0.0025711800935818,
    0.0020409960801970,
    0.0013818309223325,
    0.0006759457825454,
    0.0000000000000000,
    -0.0005822273129838,
    -0.0010249119463935,
    -0.0013027556276180,
    -0.0014110552136596,
    -0.0013635438839528,
    -0.0011884933789757,
    -0.0009237061350181,
    -0.0006110820215192,
    -0.0002914178875877,
    -0.0000000000000000,
]);

/// Construct a FIR filter that approximates the frequency response of the "Shaping"
/// filter described in the P25 standard.
impl_fir!(PreemphasisFIR, f32, 39, [
    -0.0178961626433530,
    0.0346928432330632,
    0.0163584472672260,
    -0.0063501390283224,
    -0.0344309411843599,
    -0.0521852145631016,
    -0.0398168317034308,
    0.0098706634384552,
    0.0798024315844839,
    0.1310503794999127,
    0.1214473883896009,
    0.0321842498422649,
    -0.1129971268725311,
    -0.2499342309996987,
    -0.3007279489902446,
    -0.2137087398356056,
    0.0042796502053343,
    0.2825258492739728,
    0.5139813439472742,
    0.6037091782781188,
    0.5139813439472742,
    0.2825258492739728,
    0.0042796502053343,
    -0.2137087398356056,
    -0.3007279489902446,
    -0.2499342309996987,
    -0.1129971268725311,
    0.0321842498422649,
    0.1214473883896009,
    0.1310503794999127,
    0.0798024315844839,
    0.0098706634384552,
    -0.0398168317034308,
    -0.0521852145631016,
    -0.0344309411843599,
    -0.0063501390283224,
    0.0163584472672260,
    0.0346928432330632,
    -0.0178961626433530,
]);

/// Construct a filter that approximates the frequency response of the "Integrate and
/// Dump" filter described in the P25 standard.
impl_fir!(DeemphasisFIR, f32, 39, [
    -0.0000000279259602,
    -0.0000004257533422,
    -0.0000032294948985,
    -0.0000162231698379,
    -0.0000604998924598,
    -0.0001775038066773,
    -0.0004219454782066,
    -0.0008181669612132,
    -0.0012614408313168,
    -0.0013734081042356,
    -0.0003697797010594,
    0.0029562466880931,
    0.0100294416491135,
    0.0220325488602858,
    0.0392906486169593,
    0.0607362554466107,
    0.0837434930280595,
    0.1045259714643429,
    0.1190677153117150,
    0.1242981612358404,
    0.1190677153117150,
    0.1045259714643429,
    0.0837434930280595,
    0.0607362554466107,
    0.0392906486169593,
    0.0220325488602858,
    0.0100294416491135,
    0.0029562466880931,
    -0.0003697797010594,
    -0.0013734081042356,
    -0.0012614408313168,
    -0.0008181669612132,
    -0.0004219454782066,
    -0.0001775038066773,
    -0.0000604998924598,
    -0.0000162231698379,
    -0.0000032294948985,
    -0.0000004257533422,
    -0.0000000279259602,
]);

#[cfg(test)]
mod test {
    use super::*;
    use dsp::fir::FIRCoefs;

    #[test]
    fn verify_symmetry() {
        RaisedCosineFIR::verify_symmetry();
        PreemphasisFIR::verify_symmetry();
        DeemphasisFIR::verify_symmetry();
    }
}
