mod message_state_machine;
mod state_machine;
mod symbols;

use crate::dsp::downsample::Downsample;
use crate::dsp::fir::{AsymmetricFir, FirBuilder};
use crate::message::Message;
use crate::units::Frequency;

use message_state_machine::MessageStateMachine;
use state_machine::StateMachine;

pub struct Rtty {
    input_sample_rate: Frequency,

    downsample: Downsample,

    space_filter: AsymmetricFir,
    space_envelope: AsymmetricFir,

    mark_filter: AsymmetricFir,
    mark_envelope: AsymmetricFir,

    state_machine: StateMachine,
    message_state_machine: MessageStateMachine,
}

impl Rtty {
    pub fn new(input_sample_rate: Frequency) -> Self {
        let downsample = Downsample::new(input_sample_rate.into(), Frequency::Hertz(4000.), 101);

        // let space_filter = FirBuilder::band_pass(
        //     101,
        //     downsample.output_sample_rate,
        //     Frequency::Hertz(930. - 45.45),
        //     Frequency::Hertz(930. + 45.45),
        // ).build_asymmetric();

        let space_filter = AsymmetricFir::new(&[
            0.004401685318163751,
            -0.0031891922519460252,
            -0.005165188071844155,
            -0.0013653218459616454,
            0.0039436731234518746,
            0.0015585905830777058,
            -0.005825031515704302,
            -0.004689183558195039,
            0.0056410799126460475,
            0.007395490202482083,
            -0.005298443511877412,
            -0.010951538013973658,
            0.0035524156595918037,
            0.014376560173230279,
            -0.0006770171000934032,
            -0.0175586323202373,
            -0.003574212126036032,
            0.01989902030842355,
            0.009003375772209643,
            -0.02098703986268017,
            -0.01536699489900781,
            0.02038646686324938,
            0.022204237030050023,
            -0.017814384252411962,
            -0.028951972908046224,
            0.01313396873022278,
            0.034953462311623704,
            -0.006432733388961946,
            -0.039550967372561814,
            -0.0019729165869723534,
            0.04213808492422313,
            0.011579358601583798,
            -0.04226223966010847,
            -0.02169283071687116,
            0.039669098354297876,
            0.03152322219166454,
            -0.034345745757671894,
            -0.04024444283089375,
            0.026550277361301743,
            0.047103848217292,
            -0.016779613245862458,
            -0.051484638921911394,
            0.005740036401254487,
            0.05299223956158216,
            0.005740036401254487,
            -0.051484638921911394,
            -0.016779613245862458,
            0.047103848217292,
            0.026550277361301743,
            -0.04024444283089375,
            -0.034345745757671894,
            0.03152322219166454,
            0.039669098354297876,
            -0.02169283071687116,
            -0.04226223966010847,
            0.011579358601583798,
            0.04213808492422313,
            -0.0019729165869723534,
            -0.039550967372561814,
            -0.006432733388961946,
            0.034953462311623704,
            0.01313396873022278,
            -0.028951972908046224,
            -0.017814384252411962,
            0.022204237030050023,
            0.02038646686324938,
            -0.01536699489900781,
            -0.02098703986268017,
            0.009003375772209643,
            0.01989902030842355,
            -0.003574212126036032,
            -0.0175586323202373,
            -0.0006770171000934032,
            0.014376560173230279,
            0.0035524156595918037,
            -0.010951538013973658,
            -0.005298443511877412,
            0.007395490202482083,
            0.0056410799126460475,
            -0.004689183558195039,
            -0.005825031515704302,
            0.0015585905830777058,
            0.0039436731234518746,
            -0.0013653218459616454,
            -0.005165188071844155,
            -0.0031891922519460252,
            0.004401685318163751,
        ]);

        // let mark_filter = FirBuilder::band_pass(
        //     101,
        //     downsample.output_sample_rate,
        //     Frequency::Hertz(1100. - 45.45),
        //     Frequency::Hertz(1100. + 45.45),
        // )
        // .build_asymmetric();

        let mark_filter = AsymmetricFir::new(&[
            -0.005568659595559122,
            -0.002068656431047643,
            0.006224656585256799,
            -0.002290355700810118,
            -0.005303417321919662,
            0.0031592243384618236,
            0.007109487756144221,
            -0.007558769063120972,
            -0.005513477557782942,
            0.010657149275514241,
            0.003626764093661728,
            -0.014580199411288826,
            0.0008784592914623675,
            0.016846143952880446,
            -0.006507374572180928,
            -0.017701930669220947,
            0.013573692423356121,
            0.015816798417601044,
            -0.020701825088456628,
            -0.011208810843360565,
            0.0270964673033883,
            0.003667638142382682,
            -0.03136957800877997,
            0.0061252372310419845,
            0.03256536627800372,
            -0.01725864901417401,
            -0.029867824265741607,
            0.028283016147926672,
            0.023081431356913054,
            -0.0376715009419309,
            -0.012550719992463799,
            0.0439187297696432,
            -0.0007127702128796392,
            -0.04584422829484191,
            0.015236495631439978,
            0.042816377224689076,
            -0.02921886385080467,
            -0.03487990605501333,
            0.04083723611903637,
            0.022788469688872964,
            -0.04851761160988279,
            -0.007922523163940431,
            0.0512041304755787,
            -0.007922523163940431,
            -0.04851761160988279,
            0.022788469688872964,
            0.04083723611903637,
            -0.03487990605501333,
            -0.02921886385080467,
            0.042816377224689076,
            0.015236495631439978,
            -0.04584422829484191,
            -0.0007127702128796392,
            0.0439187297696432,
            -0.012550719992463799,
            -0.0376715009419309,
            0.023081431356913054,
            0.028283016147926672,
            -0.029867824265741607,
            -0.01725864901417401,
            0.03256536627800372,
            0.0061252372310419845,
            -0.03136957800877997,
            0.003667638142382682,
            0.0270964673033883,
            -0.011208810843360565,
            -0.020701825088456628,
            0.015816798417601044,
            0.013573692423356121,
            -0.017701930669220947,
            -0.006507374572180928,
            0.016846143952880446,
            0.0008784592914623675,
            -0.014580199411288826,
            0.003626764093661728,
            0.010657149275514241,
            -0.005513477557782942,
            -0.007558769063120972,
            0.007109487756144221,
            0.0031592243384618236,
            -0.005303417321919662,
            -0.002290355700810118,
            0.006224656585256799,
            -0.002068656431047643,
            -0.005568659595559122,
        ]);

        let envelope =
            FirBuilder::low_pass(31, downsample.output_sample_rate, Frequency::Hertz(100.));

        let space_envelope = envelope.build_asymmetric();
        let mark_envelope = envelope.build_asymmetric();

        dbg!(&downsample.output_sample_rate);

        Self {
            input_sample_rate,

            space_filter,
            mark_filter,

            space_envelope,
            mark_envelope,

            state_machine: StateMachine::new(
                downsample.output_sample_rate,
                Frequency::Hertz(45.45),
            ),
            message_state_machine: MessageStateMachine::new(downsample.output_sample_rate),
            downsample,
        }
    }

    pub fn update(&mut self, sample: f32) -> (Option<f32>, Option<Box<dyn Message>>) {
        // Space is at 930 Hz
        // Mark is at 1100 Hz
        // Spacing is 170 Hz
        // Baudrate is 45.45 Hz
        // Tuner is at 884.55 Hz
        // Upper filter limit is at 260.9 Hz

        // Decimate to 4000 Hz
        if let Some(sample) = self.downsample.update(sample) {
            let mark_val = self.mark_filter.update(sample);
            let space_val = self.space_filter.update(sample);

            let mark_env = self.mark_envelope.update(mark_val.abs());
            let space_env = self.space_envelope.update(space_val.abs());

            let (c, mark) = if mark_env > space_env {
                (self.state_machine.update(true), Some(1.))
            } else {
                (self.state_machine.update(false), Some(0.))
            };

            let message = self.message_state_machine.update(c);
            let message: Option<Box<dyn Message>> = if let Some(message) = message {
                Some(Box::new(message))
            } else {
                None
            };

            (mark, message)
        } else {
            (None, None)
        }
    }
}
